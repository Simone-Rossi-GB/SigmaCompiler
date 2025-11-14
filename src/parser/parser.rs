use crate::lexer::tokenizer::Token;
use crate::parser::ast::*;

fn parse_function(tokens: &[Token], index: &mut usize) -> Result<Function, String> {
    *index += 1;

    let return_type = match &tokens[*index] {
        Token::Based => Type::Based,
        Token::SuperBased => Type::SuperBased,
        Token::Chill => Type::Chill,
        Token::Vibes => Type::Vibes,
        Token::Ghost => Type::Ghost,
        _ => return Err("Expected return type after 'bussin'".to_string())
    };

    *index += 1;

    let name = match &tokens[*index] {
        Token::Sigma => "sigma".to_string(),
        Token::Rizz(n) => n.clone(),
        _ => return Err("Expected function name after type of bussin".to_string())
    };

    *index += 1;

    if !matches!(tokens[*index], Token::OpenParen) {
        return Err("Expected '(' after function name".to_string());
    }

    *index += 1;

    let parameters = parse_parameters(tokens, index)?;

    if !matches!(tokens[*index], Token::OpenBrace) {
        return Err("Expected '{' to start function body".to_string());
    }

    *index += 1;

    let body = parse_body(tokens, index)?;

    if !matches!(tokens[*index], Token::CloseBrace) {
        return Err("Expected '}' to end function body".to_string());
    }

    *index += 1;

    Ok(Function {
        name,
        return_type,
        parameters,
        body
    })
}

fn parse_parameters(tokens: &[Token], index: &mut usize) -> Result<Vec<Parameter>, String> {
    let mut parameters = Vec::new();

    // Se c'è subito ), non ci sono parametri
    if matches!(tokens[*index], Token::CloseParen) {
        *index += 1; // Consuma )
        return Ok(parameters);
    }

    // Altrimenti, parsare i parametri in loop
    loop {
        // Parse tipo del parametro
        let parameter_type = match &tokens[*index] {
            Token::Based => Type::Based,
            Token::SuperBased => Type::SuperBased,
            Token::Chill => Type::Chill,
            Token::Vibes => Type::Vibes,
            Token::Chad => Type::Chad,
            Token::Ghost => Type::Ghost,
            _ => return Err("Expected parameter type".to_string())
        };

        *index += 1;

        // Parse nome del parametro
        let name = match &tokens[*index] {
            Token::Rizz(n) => n.clone(),
            _ => return Err("Expected parameter name after type".to_string())
        };

        *index += 1;

        // Aggiungi il parametro al vettore
        parameters.push(Parameter { name, parameter_type });

        // Controlla cosa viene dopo: virgola o )
        match &tokens[*index] {
            Token::Comma => {
                *index += 1; // Consuma la virgola e continua al prossimo parametro
            },
            Token::CloseParen => {
                *index += 1; // Consuma ) e termina
                break;
            },
            _ => return Err("Expected ',' or ')' after parameter".to_string())
        }
    }

    Ok(parameters)
}

// ==================== FUNZIONI HELPER PER GLI STATEMENT ====================

fn parse_var_decl(tokens: &[Token], index: &mut usize) -> Result<Statement, String> {
    let var_type = match &tokens[*index] {
        Token::Based => Type::Based,
        Token::SuperBased => Type::SuperBased,
        Token::Chill => Type::Chill,
        Token::Vibes => Type::Vibes,
        Token::Chad => Type::Chad,
        _ => return Err("Expected type in variable declaration".to_string())
    };

    *index += 1;

    let name = match &tokens[*index] {
        Token::Rizz(n) => n.clone(),
        _ => return Err("Expected variable name after type".to_string())
    };

    *index += 1;

    if !matches!(tokens[*index], Token::Slay) {
        return Err("Expected 'slay' after variable name".to_string());
    }

    *index += 1;

    let value = parse_expression(tokens, index)?;

    if !matches!(tokens[*index], Token::Semicolon) {
        return Err("Expected ';' after variable value".to_string());
    }

    *index += 1;

    Ok(Statement::VarDecl {var_type, name, value})
}

fn parse_assignment(tokens: &[Token], index: &mut usize) -> Result<Statement, String> {
    let name = match &tokens[*index] {
        Token::Rizz(n) => n.clone(),
        _ => return Err("Expected variable name in assignment".to_string())
    };

    *index += 1;

    if !matches!(tokens[*index], Token::Slay) {
        return Err("Expected 'slay' after variable name".to_string());
    }

    *index += 1;

    let value = parse_expression(tokens, index)?;

    if !matches!(tokens[*index], Token::Semicolon) {
        return Err("Expected ';' after variable value".to_string());
    }

    *index += 1;

    Ok(Statement::Assignment {name, value})
}

fn parse_print(tokens: &[Token], index: &mut usize) -> Result<Statement, String> {

    *index += 1;

    let expr = parse_expression(tokens, index)?;

    if !matches!(tokens[*index], Token::Semicolon) {
        return Err("Expected ';' after print statement".to_string());
    }

    *index += 1;

    Ok(Statement::Print {expr})
}

fn parse_return(tokens: &[Token], index: &mut usize) -> Result<Statement, String> {
    *index += 1;

    let expr = if matches!(tokens[*index], Token::Semicolon) {
        None
    } else {
        Some(parse_expression(tokens, index)?)
    };

    if !matches!(tokens[*index], Token::Semicolon) {
        return Err("Expected ';' after return statement".to_string());
    }

    *index += 1;

    Ok(Statement::Return {expr})
}

// Entry point per le espressioni - gestisce la precedenza più bassa
fn parse_expression(tokens: &[Token], index: &mut usize) -> Result<Expression, String> {
    parse_comparison(tokens, index)
}

// Livello 1: Comparazione (==, !=, <, >, <=, >=)
fn parse_comparison(tokens: &[Token], index: &mut usize) -> Result<Expression, String> {
    let mut left = parse_additive(tokens, index)?;

    while *index < tokens.len() {
        let op = match &tokens[*index] {
            Token::Equal => BinOp::Equal,
            Token::NotEqual => BinOp::NotEqual,
            Token::Less => BinOp::Less,
            Token::Greater => BinOp::Greater,
            Token::LessEq => BinOp::LessEq,
            Token::GreaterEq => BinOp::GreaterEq,
            _ => break
        };

        *index += 1;
        let right = parse_additive(tokens, index)?;
        left = Expression::BinOp {
            left: Box::new(left),
            op,
            right: Box::new(right)
        };
    }

    Ok(left)
}

// Livello 2: Addizione/Sottrazione (+, -)
fn parse_additive(tokens: &[Token], index: &mut usize) -> Result<Expression, String> {
    let mut left = parse_multiplicative(tokens, index)?;

    while *index < tokens.len() {
        let op = match &tokens[*index] {
            Token::Plus => BinOp::Add,
            Token::Minus => BinOp::Sub,
            _ => break
        };

        *index += 1;
        let right = parse_multiplicative(tokens, index)?;
        left = Expression::BinOp {
            left: Box::new(left),
            op,
            right: Box::new(right)
        };
    }

    Ok(left)
}

// Livello 3: Moltiplicazione/Divisione (*, /)
fn parse_multiplicative(tokens: &[Token], index: &mut usize) -> Result<Expression, String> {
    let mut left = parse_primary(tokens, index)?;

    while *index < tokens.len() {
        let op = match &tokens[*index] {
            Token::Star => BinOp::Mul,
            Token::Slash => BinOp::Div,
            _ => break
        };

        *index += 1;
        let right = parse_primary(tokens, index)?;
        left = Expression::BinOp {
            left: Box::new(left),
            op,
            right: Box::new(right)
        };
    }

    Ok(left)
}

// Livello 4: Primari (numeri, variabili, parentesi)
fn parse_primary(tokens: &[Token], index: &mut usize) -> Result<Expression, String> {
    let expr = match &tokens[*index] {
        Token::IntLit(n) => {
            if *n >= i32::MIN as i64 && *n <= i32::MAX as i64 {
                Expression::Integer(*n as i32)
            } else {
                Expression::Long(*n)
            }
        }
        Token::FloatLit(f) => Expression::Float(*f),
        Token::StringLit(s) => Expression::StringLit(s.clone()),
        Token::CharLit(c) => Expression::CharLit(*c),
        Token::Rizz(name) => Expression::Variable(name.clone()),

        // Gestione parentesi: (2 + 3) * 4
        Token::OpenParen => {
            *index += 1;
            let expr = parse_expression(tokens, index)?;
            if !matches!(tokens[*index], Token::CloseParen) {
                return Err("Expected ')' after expression".to_string());
            }
            *index += 1;
            return Ok(expr);
        }

        _ => return Err(format!("Expected expression, found {:?}", tokens[*index]))
    };

    *index += 1;
    Ok(expr)
}

// ==================== PARSING DEL BODY ====================

fn parse_body(tokens: &[Token], index: &mut usize) -> Result<Vec<Statement>, String> {
    let mut statements = Vec::new();

    // Loop finché non incontriamo "}"
    while !matches!(tokens[*index], Token::CloseBrace) {
        // Guarda che token è e decidi cosa fare
        let stmt = match &tokens[*index] {
            Token::Based | Token::SuperBased | Token::Chill | Token::Vibes | Token::Chad => parse_var_decl(tokens, index)?,
            Token::Flex => parse_print(tokens, index)?,
            Token::Ohio => {
                *index += 1;
                if !matches!(tokens[*index], Token::Semicolon) {
                    return Err("Expected ';' after 'ohio'".to_string());
                }
                *index += 1;
                Statement::Break
            }
            Token::Yeet => parse_return(tokens, index)?,
            Token::Rizz(_) => parse_assignment(tokens, index)?,
            _ => return Err(format!("Unexpected token in body: {:?}", tokens[*index]))
        };

        statements.push(stmt);
    }

    Ok(statements)
}

pub fn parse(tokens: Vec<Token>) -> Result<Program, String> {
    let mut index = 0;
    let mut functions = Vec::new();

    while index < tokens.len() {
        if tokens[index] == Token::Bussin {
            let func = parse_function(&tokens, &mut index);
            functions.push(func?);
        } else {
            return Err("Expected function declaration".to_string());
        }
    }

    //controlliamo se c'è almeno un main
    let has_main = functions.iter().any(|f| f.name == "sigma");
    if !has_main {
        return Err("Program must have at least one 'bussin sigma()' function".to_string());
    }

    Ok(Program { functions })
}