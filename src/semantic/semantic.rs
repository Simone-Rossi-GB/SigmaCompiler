use std::collections::HashMap;
use crate::parser::{Program, Type};

struct SymbolTable {
    scopes: Vec<HashMap<String, SymbolInfo>>
}

struct SymbolInfo {
    name: String,
    tipo: Type,
    location: usize
}



pub fn analyze_program(ast: Program) -> Result<String, String> {

    // scrivere l'analisi semantica

    analyze_function("sigma");

    Ok(("AST correct".to_string()))
}

fn analyze_function(p0: &str) {

}