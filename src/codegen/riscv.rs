use crate::parser::ast::*;
use crate::codegen::context::*;

pub fn generate_riscv(ast: &Program) -> Result<String, String> {
    let mut output = String::new();
    let mut ctx = CodeGenContext::new();

    // header di riconoscimento compilatore sigma
    output.push_str("# Generato da Sigma Manny Compiler\n");
    output.push_str("# Target: RISC-V RV32IM Linux\n\n");

    // dichiaro il main _start globale (.text)
    output.push_str("\n.text\n");
    output.push_str(".global _start\n\n");

    // Entry point del _start
    generate_entry_point(&mut output);

    // Funzioni helper
    generate_helpers(&mut output);

    for function in &ast.functions {
        generate_function(&mut output, &mut ctx, function)?;
    }

    // variabili globaili e funzioni (.data)
    generate_data_section(&mut output, &mut ctx, ast)?;

    Ok(output)
}

fn generate_function(output: &mut String, ctx: &mut CodeGenContext, func: &Function) -> Result<(), String> {
    output.push_str(&format!("# Funzione {}\n", func.name));
    output.push_str(&format!("{}:\n", func.name));

    // Salva stato del contesto (ogni funzione ha scope separato)
    let saved_offset = ctx.stack_offset;
    let saved_vars = ctx.variables.clone();
    let saved_types = ctx.variable_types.clone();
    ctx.stack_offset = 0;
    ctx.variables.clear();
    ctx.variable_types.clear();

    // Contiamo quante variabili locali vi sono
    let local_count = count_local_vars(&func.body);
    let param_count = func.parameters.len();

    // Calcola spazio necessario:
    // - 4 byte per ogni variabile locale e parametro
    // - 4 byte per il return address (ra)
    // - Padding extra per temporanei durante valutazione espressioni
    //   (le espressioni binarie usano stack per temporanei)
    let max_expr_depth = estimate_max_expression_depth(&func.body);
    let temp_space = max_expr_depth * 4;
    let total_stack = (local_count + param_count) * 4 + 4 + temp_space;

    // prologo della funzione in risc-v
    // Usiamo s0 come frame pointer per accedere alle variabili
    // anche quando sp viene modificato per temporanei

    output.push_str("   # Prologo \n");
    if total_stack > 0 {
        output.push_str(&format!("  addi sp, sp, -{}\n", total_stack));
    }
    output.push_str("  mv s0, sp       # s0 = frame pointer\n");
    output.push_str(&format!("  sw ra, {}(s0)\n", total_stack - 4));

    // salviamo i parametri nello stack

    for (i, param) in func.parameters.iter().enumerate() {
        let offset = ctx.allocate_variable(param.name.clone(), param.parameter_type.clone());
        let reg = format!("a{}", i);
        output.push_str(&format!("   sw {}, {}(s0) # param {}\n", reg, offset, param.name));
    }

    // generiamo il body in risc-v
    for stmt in &func.body {
        generate_statement(output, ctx, stmt)?;
    }

    // Epilogo della funzione in risc-v

    output.push_str("   # Epilogo\n");

    // Se la funzione è ghost (void) e non ha return esplicito, imposta a0 = 0
    if func.return_type == Type::Ghost {
        output.push_str("  li a0, 0        # funzione ghost ritorna 0\n");
    }

    output.push_str(&format!("  lw ra, {}(s0)\n", total_stack - 4));
    if total_stack > 0 {
        output.push_str(&format!("  addi sp, sp, {}\n", total_stack));
    }

    output.push_str("   ret\n\n");

    // ripristo il contesto precedente
    ctx.stack_offset = saved_offset;
    ctx.variables = saved_vars;
    ctx.variable_types = saved_types;

    Ok(())
}

fn generate_statement(output: &mut String, ctx: &mut CodeGenContext, stmt: &Statement) -> Result<(), String> {
    match stmt {
        Statement::VarDecl { var_type, name, value} => {
            output.push_str(&format!("   # VarDecl: {} {}\n",
                                    type_to_string(var_type), name));

            // allochiamo una bella variabile aahahaha
            let offset = ctx.allocate_variable(name.clone(), var_type.clone());

            // calcoliamo il valore (il risultato si troverà in a0)
            generate_expression(output, ctx, value)?;

            // salviamo nello stack (usa s0 come frame pointer)
            output.push_str(&format!("  sw a0, {}(s0)   # salva in stack {}\n",
                                     offset, name));
            Ok(())
        },

        Statement::Assignment { name, value } => {
            output.push_str(&format!("  # Assignment: {} =\n", name));

            // troviamo l'offset della variabile
            let offset = ctx.get_variable_offset(name)
                .ok_or_else(|| format!("Variable '{}' not found", name))?;

            // calcoliamo il nuovo valore
            generate_expression(output, ctx, value)?;

            // salviamo il valore all'offset della variabile (usa s0)
            output.push_str(&format!("  sw a0, {}(s0)   # {} =\n",
                                     offset, name));
            Ok(())
        },
        Statement::Return { expr} => {
            output.push_str("   # Return\n");

            if let Some(e) = expr {
                // calcoliamo il valore di ritorno
                generate_expression(output, ctx, e)?;
                // il valore di ritorno si trova in a0
            } else {
                // nessun valore di ritorno
                output.push_str("   li a0, 0\n");
            }

            // ret lo farà l'epilogo della funzione
            Ok(())
        },
        Statement::Break => {
            if let Some(end_label) = ctx.current_loop_end() {
                output.push_str(&format!("   j    {}      # ohio (break)\n", end_label));
                Ok(())
            } else {
                Err("Break fuori da un loop porcodio!".to_string())
            }
        },

        Statement::Print { expr} => {
            output.push_str("   # Print\n");

            // calcoliamo l'espressione da stampare
            generate_expression(output, ctx, expr)?;

            // chiamiamo l'helper appropriato in base al tipo
            let should_print_string = match expr {
                Expression::StringLit(_) => true,
                Expression::Variable(name) => {
                    // controlliamo il tipo della variabile
                    if let Some(var_type) = ctx.get_variable_type(name) {
                        matches!(var_type, Type::Vibes)
                    } else {
                        false
                    }
                },
                _ => false,
            };

            if should_print_string {
                output.push_str("   call print_string\n");
            } else {
                output.push_str("   call print_int\n");
            }

            Ok(())
        },

        Statement::If { condition, then_body, else_body } => {
            // genero le label uniche per questo if
            let else_label = ctx.generate_label(".Lelse");
            let end_label = ctx.generate_label(".Lend_if");

            output.push_str("   # If statement (ong)\n");

            // valuto la condizione (risultato in a0)
            generate_expression(output, ctx, condition)?;

            // se a0 == 0 (falso) salto all'else
            output.push_str(&format!("   beqz a0, {}\n", else_label));

            // corpo del then
            output.push_str("   # Then body\n");
            for stmt in then_body {
                generate_statement(output, ctx, stmt)?;
            }

            // salto alla fine dell'if (skippo l'else)
            output.push_str(&format!("   j    {}\n", end_label));

            // label else
            output.push_str(&format!("{}:\n", else_label));

            // corpo dell'else se c'è
            if let Some(stmts) = else_body {
                output.push_str("   # Else body (nah)\n");
                for stmt in stmts {
                    generate_statement(output, ctx, stmt)?;
                }
            }

            // label fine if
            output.push_str(&format!("{}:\n", end_label));

            Ok(())
        },

        Statement::While { condition, body } => {
            // genero le label uniche per questo while
            let start_label = ctx.generate_label(".Lwhile_start");
            let end_label = ctx.generate_label(".Lwhile_end");

            // entro nel loop (per gestire break)
            ctx.enter_loop(end_label.clone());

            output.push_str("   # While loop (mewing)\n");

            // label inizio loop
            output.push_str(&format!("{}:\n", start_label));

            // valuto la condizione
            generate_expression(output, ctx, condition)?;

            // se a0 == 0 (falso) esco dal loop
            output.push_str(&format!("   beqz a0, {}\n", end_label));

            // corpo del while
            for stmt in body {
                generate_statement(output, ctx, stmt)?;
            }

            // torno all'inizio del loop
            output.push_str(&format!("   j    {}\n", start_label));

            // label fine loop
            output.push_str(&format!("{}:\n", end_label));

            // esco dal loop stack
            ctx.exit_loop();

            Ok(())
        },

        Statement::For { init, condition, increment, body } => {
            // genero le label uniche per questo for
            let start_label = ctx.generate_label(".Lfor_start");
            let end_label = ctx.generate_label(".Lfor_end");

            output.push_str("   # For loop (sixSeven)\n");

            // inizializzazione del for
            output.push_str("   # Init\n");
            generate_statement(output, ctx, init)?;

            // entro nel loop
            ctx.enter_loop(end_label.clone());

            // label inizio loop
            output.push_str(&format!("{}:\n", start_label));

            // condizione
            output.push_str("   # Condition\n");
            generate_expression(output, ctx, condition)?;

            // se falso esco
            output.push_str(&format!("   beqz a0, {}\n", end_label));

            // corpo del for
            output.push_str("   # Body\n");
            for stmt in body {
                generate_statement(output, ctx, stmt)?;
            }

            // incremento
            output.push_str("   # Increment\n");
            generate_statement(output, ctx, increment)?;

            // torno all'inizio
            output.push_str(&format!("   j    {}\n", start_label));

            // label fine loop
            output.push_str(&format!("{}:\n", end_label));

            // esco dal loop stack
            ctx.exit_loop();

            Ok(())
        }
    }
}

fn generate_expression(output: &mut String, ctx: &mut CodeGenContext, expr: &Expression) -> Result<(), String> {
    match expr {
        Expression::Integer(n) => {
            output.push_str(&format!("  li a0, {}\n", n));
            Ok(())
        },
        Expression::Long(n) => {
            // purtroppo in RV32 i registri sono a 32-bit quindi ci tocca troncare :(
            output.push_str(&format!("  li a0, {}\n", *n as i32));
            Ok(())
        },
        Expression::CharLit(c) => {
            output.push_str(&format!("  li a0, {}\n", *c as i32));
            Ok(())
        },
        Expression::StringLit(s) => {
            let label = ctx.add_string_literal(s.clone());
            output.push_str(&format!("  la a0, {}\n", label));
            Ok(())
        },
        Expression::Variable(name) => {
            let offset = ctx.get_variable_offset(name)
                .ok_or_else(|| format!("Variable '{}' not found", name))?;
            output.push_str(&format!("  lw a0, {}(s0)   # load {}\n", offset, name));
            Ok(())
        },
        Expression::BinOp {left, op, right} => {
            // generiamo left
            generate_expression(output, ctx, left)?;
            output.push_str("   addi sp, sp, -4\n");
            output.push_str("   sw a0, 0(sp)\n");

            // generiamo right
            generate_expression(output, ctx, right)?;

            // carico left in a1
            output.push_str("   lw a1, 0(sp)\n");
            output.push_str("   addi sp, sp, 4\n");

            // eseguo l'operazione
            use BinOp::*;
            match op {
                Add => output.push_str("    add a0, a1, a0\n"),
                Sub => output.push_str("    sub a0, a1, a0\n"),
                Div => output.push_str("    div a0, a1, a0\n"),
                Mul => output.push_str("    mul a0, a1, a0\n"),

                Equal => {
                    output.push_str("   sub a0, a1, a0\n");
                    output.push_str("   seqz a0, a0\n");
                },
                NotEqual => {
                    output.push_str("   sub a0, a1, a0\n");
                    output.push_str("   snez a0, a0\n");
                },
                Less => output.push_str("   slt a0, a1, a0\n"),
                Greater => output.push_str("    slt a0, a0, a1\n"),
                GreaterEq => {
                    output.push_str("   slt a0, a1, a0\n");
                    output.push_str("   xori a0, a0, 1\n");
                },
                LessEq => {
                    output.push_str("   slt a0, a0, a1\n");
                    output.push_str("   xori a0, a0, 1\n");
                }
            }

            Ok(())
        }
    }
}

fn type_to_string(var_type: &Type) -> &str {
    match var_type {
        Type::Based => "based",
        Type::Chill => "chill",
        Type::Chad => "chad",
        Type::SuperBased => "superBased",
        Type::Vibes => "vibes",
        Type::Ghost => "ghost"
    }
}

fn count_local_vars(stmts: &Vec<Statement>) -> usize {
    stmts.iter()
        .filter(|s| matches!(s, Statement::VarDecl { .. }))
        .count()
}

// Stima la profondità massima delle espressioni annidate
// Ogni livello di BinOp richiede 4 byte sullo stack per temporanei
fn estimate_max_expression_depth(stmts: &Vec<Statement>) -> usize {
    let mut max_depth = 0;
    for stmt in stmts {
        let depth = statement_expr_depth(stmt);
        if depth > max_depth {
            max_depth = depth;
        }
    }
    // Aggiungi un buffer di sicurezza
    max_depth + 4
}

fn statement_expr_depth(stmt: &Statement) -> usize {
    match stmt {
        Statement::VarDecl { value, .. } => expr_depth(value),
        Statement::Assignment { value, .. } => expr_depth(value),
        Statement::Print { expr } => expr_depth(expr),
        Statement::Return { expr: Some(e) } => expr_depth(e),
        Statement::If { condition, then_body, else_body } => {
            let cond_depth = expr_depth(condition);
            let then_depth = then_body.iter().map(|s| statement_expr_depth(s)).max().unwrap_or(0);
            let else_depth = else_body.as_ref()
                .map(|blk| blk.iter().map(|s| statement_expr_depth(s)).max().unwrap_or(0))
                .unwrap_or(0);
            cond_depth.max(then_depth).max(else_depth)
        }
        Statement::While { condition, body } => {
            let cond_depth = expr_depth(condition);
            let body_depth = body.iter().map(|s| statement_expr_depth(s)).max().unwrap_or(0);
            cond_depth.max(body_depth)
        }
        _ => 0
    }
}

fn expr_depth(expr: &Expression) -> usize {
    match expr {
        Expression::BinOp { left, right, .. } => {
            1 + expr_depth(left).max(expr_depth(right))
        }
        _ => 0
    }
}

fn generate_helpers(output: &mut String) {
    output.push_str("# Helper: stampa di numero intero in a0\n");
    output.push_str("print_int:\n");
    output.push_str("   addi sp, sp, -20\n");
    output.push_str("   mv t0, sp\n");
    output.push_str("   li t1, 10\n");
    output.push_str("   mv t2, a0\n");
    output.push_str("   li t3, 0\n");
    output.push_str("   bgez t2, .Lconv_loop\n");
    output.push_str("   li t3, 1\n");
    output.push_str("   neg t2, t2\n");
    output.push_str(".Lconv_loop:\n"); // converte un numero negativo in positivo e nella stampa lo rimette negativo
    output.push_str("   remu t4, t2, t1\n"); // t4 = numero / 10 (il resto)
    output.push_str("   divu t2, t2, t1\n"); // t2 = numero / 10 --> es. 38 / 10 resto = 8 e poi elimino 8 da 38 dividendo per 10
    output.push_str("   addi t4, t4, 48\n"); // t4 (0) += 48 che equivale a '0'
    output.push_str("   sb t4, 0(t0)\n"); // salvo nel primo byte la cifra
    output.push_str("   addi t0, t0, 1\n"); // aggiungo uno al puntatore
    output.push_str("   bnez t2, .Lconv_loop\n");
    output.push_str("   beqz t3, .Lrev_start\n");
    output.push_str("   li t4, 45\n");
    output.push_str("   sb t4, 0(t0)\n");
    output.push_str("   addi t0, t0, 1\n");
    output.push_str(".Lrev_start:\n");
    output.push_str("   mv t1, sp\n");
    output.push_str("   addi t2, t0, -1\n");
    output.push_str(".Lrev_loop:\n");
    output.push_str("   bge  t1, t2, .Lprint\n");
    output.push_str("   lb   t3, 0(t1)\n");
    output.push_str("   lb   t4, 0(t2)\n");
    output.push_str("   sb   t4, 0(t1)\n");
    output.push_str("   sb   t3, 0(t2)\n");
    output.push_str("   addi t1, t1, 1\n");
    output.push_str("   addi t2, t2, -1\n");
    output.push_str("   j    .Lrev_loop\n");
    output.push_str(".Lprint:\n");
    output.push_str("   li   a0, 1\n");
    output.push_str("   mv   a1, sp\n");
    output.push_str("   sub  a2, t0, sp\n");
    output.push_str("   li   a7, 64\n");
    output.push_str("   ecall\n");
    output.push_str("   # stampo una nuova linea \n");
    output.push_str("   li   a0, 1\n");
    output.push_str("   la   a1, .Lnewline\n");
    output.push_str("   li   a2, 1\n");
    output.push_str("   li   a7, 64\n");
    output.push_str("   ecall\n");
    output.push_str("   addi sp, sp, 20\n");
    output.push_str("   ret\n\n");

    // Helper per stampare stringhe (implementato da me diocristo)
    output.push_str("# Stampa stringa (vibes) in a0 (implementato da me porca puttana)\n");
    output.push_str("print_string:\n");
    output.push_str("   addi sp, sp, -4\n");
    output.push_str("   sw   a0, 0(sp)      # salvo l'indirizzo originale\n");
    output.push_str("   # calcolo la lunghezza della stringa\n");
    output.push_str("   mv   t0, a0         # t0 = puntatore stringa\n");
    output.push_str("   li   t1, 0          # t1 = contatore lunghezza\n");
    output.push_str(".Lstrlen_loop:\n");
    output.push_str("   lb   t2, 0(t0)      # carico byte corrente\n");
    output.push_str("   beqz t2, .Lstr_write # se '\0' (null terminator) stampa\n");
    output.push_str("   addi t0, t0, 1      # vado al prossimo carattere\n");
    output.push_str("   addi t1, t1, 1      # incremento il contatore\n");
    output.push_str("   j    .Lstrlen_loop  # continuo il loop dio boia\n");
    output.push_str(".Lstr_write:\n");
    output.push_str("   # ora faccio la syscall write(1, indirizzo, lunghezza)\n");
    output.push_str("   lw   a1, 0(sp)      # ripristino indirizzo originale in a1\n");
    output.push_str("   mv   a2, t1         # lunghezza in a2\n");
    output.push_str("   li   a0, 1          # stdout\n");
    output.push_str("   li   a7, 64         # syscall write\n");
    output.push_str("   ecall\n");
    output.push_str("   # stampo anche il newline\n");
    output.push_str("   li   a0, 1\n");
    output.push_str("   la   a1, .Lnewline\n");
    output.push_str("   li   a2, 1\n");
    output.push_str("   li   a7, 64\n");
    output.push_str("   ecall\n");
    output.push_str("   addi sp, sp, 4      # ripristino stack\n");
    output.push_str("   ret\n\n");
}

/*Spiegazione dettagliata di ogni riga:

| Riga  | Codice               | Cosa fa                                |
|-------|----------------------|----------------------------------------|
| 1     | addi sp, sp, -20     | Alloca 20 byte sullo stack per buffer  |
| 2     | mv t0, sp            | t0 = puntatore al buffer               |
| 3     | li t1, 10            | t1 = 10 (divisore)                     |
| 4     | mv t2, a0            | t2 = numero da convertire              |
| 5     | li t3, 0             | t3 = flag negativo (0 = positivo)      |
| 6     | bgez t2, .Lconv_loop | Se t2 >= 0, salta al loop              |
| 7     | li t3, 1             | Altrimenti: flag = negativo            |
| 8     | neg t2, t2           | t2 = valore assoluto                   |
| 9-16  | Loop conversione     | Dividi per 10, converti cifre in ASCII |
| 17-19 | Aggiungi '-'         | Se negativo, aggiungi '-' al buffer    |
| 20-30 | Inverti buffer       | Scambia byte dall'inizio alla fine     |
| 31-35 | write()              | Stampa il numero                       |
| 36-40 | write() newline      | Stampa '\n'                            |
| 41    | addi sp, sp, 20      | Dealloca buffer                        |
| 42    | ret                  | Ritorna                                |*/

fn generate_entry_point(output: &mut String) {
    output.push_str("_start:\n");
    output.push_str("# chiamo la funzione sigma (main) presente obbligatoriamente\n");
    output.push_str("call sigma\n");
    output.push_str("# exit con valore di ritorno in a0 di sigma\n");
    output.push_str("li a7, 93\n"); // syscall exit con return value nel registro a0
    output.push_str("ecall\n\n");
}

fn generate_data_section(output: &mut String, ctx: &mut CodeGenContext, ast: &Program) -> Result<(), String> {
    // Genera sempre la sezione .data perché le funzioni print_int e print_string usano .Lnewline
    output.push_str("\n.data\n");

    // Stringhe letterali (se presenti)
    for (i, s) in ctx.string_literals.iter().enumerate() {
        output.push_str(&format!("str_{}: .asciz \"{}\"\n", i, s));
    }

    // Label per il newline (sempre necessaria per print_int e print_string)
    output.push_str(".Lnewline: .asciz \"\n\" \n");

    Ok(())
}