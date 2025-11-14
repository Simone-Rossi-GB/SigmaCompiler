use std::collections::HashMap;
use crate::parser::ast::*;

struct SymbolTable {
    scopes: Vec<HashMap<String, SymbolInfo>>
}

impl SymbolTable {
    fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()]
        }
    }
    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: String, tipo: Type, location: usize) -> Result<(), String> {
        let current_scope = self.scopes.last_mut().unwrap();

        if current_scope.contains_key(&name) {
            return Err(format!("Variable '{}' already declared in this scope", name));
        }

        current_scope.insert(name.clone(), SymbolInfo { name, tipo, location});

        Ok(())
    }

    fn lookup(&self, name: &str) -> Option<&SymbolInfo> {
        for scope in self.scopes.iter().rev() {
            if let Some(info) = scope.get(name) {
                return Some(info);
            }
        }
        None
    }
}

struct SymbolInfo {
    name: String,
    tipo: Type,
    location: usize
}

impl SymbolInfo {
    fn new(name: String, tipo: Type, location: usize) -> Self {
        return SymbolInfo {name, tipo, location}
    }
}

pub fn analyze_program(ast: Program) -> Result<(), String> {
    let mut stm_table = SymbolTable::new();
    // scrivere l'analisi semantica
    for func in &ast.functions {
        analyze_function(&mut stm_table, func)?;
    }

    Ok(())
}

fn analyze_function(stm_tab: &mut SymbolTable, func: &Function) -> Result<(), String> {
    stm_tab.enter_scope();
    for param in &func.parameters {
        stm_tab.declare(param.name.clone(), param.parameter_type.clone(), 0)?;
    }

    for stmt in &func.body {
        analyze_statement(stm_tab, stmt, &func.return_type)?;
    }

    stm_tab.exit_scope();
    Ok(())
}

fn analyze_statement(stm_tab: &mut SymbolTable, stmt: &Statement, expected_return: &Type) -> Result<(), String>{
    match stmt {
        Statement::VarDecl {var_type, name, value} => {
            let expr_type = analyze_expression(stm_tab, value)?;

            if &expr_type != var_type {
                return Err(format!("Type mismatch in variable declaration '{}': expected {:?}, got {:?}", name, var_type, expr_type));
            }

            stm_tab.declare(name.clone(), var_type.clone(), 0)?;
            Ok(())
        },
        Statement::Assignment {name, value} => {
            let var_info = stm_tab.lookup(name)
                .ok_or_else(|| format!("Variable '{}' not declared", name))?;

            // Analizza il tipo dell'espressione
            let expr_type = analyze_expression(stm_tab, value)?;

            // Controlla che i tipi siano compatibili
            if expr_type != var_info.tipo {
                return Err(format!(
                    "Type mismatch in assignment to '{}': expected {:?}, got {:?}",
                    name, var_info.tipo, expr_type
                ));
            }

            Ok(())
        },
        Statement::Print {expr} => {
            analyze_expression(stm_tab, expr)?;
            Ok(())
        }
        Statement::Return {expr} => {
            match expr {
                Some(e) => {
                    let return_type = analyze_expression(stm_tab, e)?;

                    if &return_type != expected_return {
                        return Err(format!("Return mismatch: expected '{:?}', got {:?}", expected_return, return_type))
                    }

                    Ok(())
                },
                None => {
                    if expected_return != &Type::Ghost {
                        return Err(format!("Function must return {:?}, but return statement has no return value", expected_return));
                    }
                    Ok(())
                }
            }
        },

        Statement::Break => {
            // gestire anche i break poi
            Ok(())
        }
    }

}

fn analyze_expression(stm_tab: &SymbolTable, expr: &Expression) -> Result<Type, String> {
    match expr {
        Expression::Integer(_) => Ok(Type::Based),
        Expression::Long(_) => Ok(Type::SuperBased),
        Expression::Float(_) => Ok(Type::Chill),
        Expression::StringLit(_) => Ok(Type::Vibes),
        Expression::CharLit(_) => Ok(Type::Chad),

        Expression::Variable(name) => {
            let var_info = stm_tab.lookup(name).ok_or_else(|| format!("Variable '{}' not declared", name))?;

            Ok(var_info.tipo.clone())
        },

        Expression::BinOp{left, op, right} => {
            let left_type = analyze_expression(stm_tab, left)?;
            let right_type = analyze_expression(stm_tab, right)?;
            check_binary_op(&left_type, op, &right_type)
        }
    }
}

fn check_binary_op(left_type: &Type, op: &BinOp, right_type: &Type) -> Result<Type, String> {
    use BinOp::*;

    match op {
        BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div => {
            let is_left_numeric = matches!(left_type, Type::Based | Type::SuperBased | Type::Chill);
            let is_right_numeric = matches!(right_type, Type::Based | Type::SuperBased | Type::Chill);

            if !is_left_numeric || !is_right_numeric {
                return Err(format!("Invalid operation {:?}, between '{:?}' and '{:?}': both must be numeric types!", op, left_type, right_type));
            }

            if left_type == &Type::Chill || right_type == &Type::Chill {
                Ok(Type::Chill)
            } else if left_type == &Type::SuperBased || right_type == &Type::SuperBased {
                Ok(Type::SuperBased)
            } else {
                Ok(Type::Based)
            }
        },

        BinOp::Equal | BinOp::NotEqual => {
            if left_type != right_type {
                return Err(format!("Cannot compare '{:?}' and '{:?}': types must match", left_type, right_type));
            }

            Ok(Type::Based) // 1 - true | 0 - false: per ora voglio usare un intero come bool
        },

        BinOp::Less | BinOp::Greater | BinOp::LessEq | BinOp::GreaterEq => {
            let is_left_numeric = matches!(left_type, Type::Based | Type::SuperBased | Type::Chill);
            let is_right_numeric = matches!(right_type, Type::Based | Type::SuperBased | Type::Chill);

            if !is_left_numeric || !is_right_numeric {
                return Err(format!("Invalid comparison {:?}, between '{:?}' and '{:?}': both must be numeric types!", op, left_type, right_type));
            }

            Ok(Type::Based) // 1 - true | 0 - false: per ora voglio usare un intero come bool
        }
    }
}