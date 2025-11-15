use crate::lexer::{chunker, tokenizer};
use crate::parser::{parse};
use crate::semantic::{analyze_program};
use crate::codegen::generate_riscv;
use std::fs;

pub fn compile(code: &str, output_path: &str) -> Result<(), String> {
    // 1. Prima chunker: divide il codice in pezzi
    let chunks = chunker(code);

    // 2. Poi tokenizer: trasforma i chunk in token
    let tokens = tokenizer(chunks);

    println!("\n=== Tokenizzazione completata ===");
    println!("Totale token: {}", tokens.len());

    let ast = parse(tokens)?;
    println!("\n===== Stampo AST =====");
    println!("{:#?}", ast);

    // Analisi semantica
    analyze_program(&ast)?;

    // Generazione codice RISC-V
    let riscv_code = generate_riscv(&ast)?;

    // Salva il codice assembly in un file .s
    fs::write(output_path, riscv_code)
        .map_err(|e| format!("Errore nella scrittura del file '{}': {}", output_path, e))?;

    println!("\n✓ Codice RISC-V generato in: {}", output_path);

    Ok(())
}