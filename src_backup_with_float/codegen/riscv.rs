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
    ctx.stack_offset = 0;
    ctx.variables.clear();

    // Contiamo quante variabili locali vi sono
    let local_count = count_local_vars(&func.body);
    let param_count = func.parameters.len();
    let total_stack = (local_count + param_count) * 4 + 4; // aggiungiamo 4 per il ra

    // prologo della funzione in risc-v

    output.push_str("   # Prologo \n");
    if total_stack > 0 {
        output.push_str(&format!("  addi sp, sp, -{}\n", total_stack));
    }
    output.push_str(&format!("  sw ra, {}(sp)\n", total_stack - 4));

    // salviamo i parametri nello stack

    for (i, param) in func.parameters.iter().enumerate() {
        let offset = ctx.allocate_variable(param.name.clone());
        let reg = format!("a{}", i);
        output.push_str(&format!("   sw {}, {}(sp) # param {}\n", reg, offset, param.name));
    }

    // generiamo il body in risc-v
    for stmt in &func.body {
        generate_statement(output, ctx, stmt)?;
    }

    // Epilogo della funzione in risc-v

    output.push_str("   # Epilogo\n");
    output.push_str(&format!("  lw ra, {}(sp)\n", total_stack - 4));
    if total_stack > 0 {
        output.push_str(&format!("  addi sp, sp, {}\n", total_stack));
    }

    output.push_str("   ret\n\n");

    // ripristo il contesto precedente
    ctx.stack_offset = saved_offset;
    ctx.variables = saved_vars;

    Ok(())
}

fn generate_statement(output: &mut String, ctx: &mut CodeGenContext, stmt: &Statement) -> Result<(), String> {
    match stmt {
        Statement::VarDecl { var_type, name, value} => {
            output.push_str(&format!("   # VarDecl: {} {}\n",
                                    type_to_string(var_type), name));

            // allochiamo una bella variabile aahahaha
            let offset = ctx.allocate_variable(name.clone());

            // calcoliamo il valore (il risultato si troverà in a0)
            generate_expression(output, ctx, value)?;

            // salviamo nello stack
            output.push_str(&format!("  sw a0, {}(sp)   # salva in stack {}\n",
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

            // salviamo il valore all'offset della variabile
            output.push_str(&format!("  sw a0, {}(sp)   # {} =\n",
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
            // TODO: implementa loop
            output.push_str("   # Break (TODO)\n");
            Ok(())
        },
        Statement::Print { expr} => {
            output.push_str("   # Print\n");

            // calcoliamo l'espressione da stampare
            generate_expression(output, ctx, expr)?;

            // chiamiamo l'helper per stampare
            output.push_str("   call print_int\n");
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
        Expression::Float(_) => {
            // Float richiede l'estensione F on RV32IM
            Err("Float not supported yet".to_string())
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
            output.push_str(&format!("  lw a0, {}(sp)   # load {}\n", offset, name));
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
    if !ctx.string_literals.is_empty() {
        output.push_str("\n.data\n");
        for (i, s) in ctx.string_literals.iter().enumerate() {
            output.push_str(&format!("str_{}: .asciz \"{}\"\n", i, s));
        }
        // salvo la label per lo a capo \n
        output.push_str(".Lnewline: .byte 10\n\n");
    }
    Ok(())
}