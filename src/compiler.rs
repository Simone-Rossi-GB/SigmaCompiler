use crate::lexer::{chunker, tokenizer};

pub fn compile(code: &str) -> Result<(), String> {
    // 1. Prima chunker: divide il codice in pezzi
    let chunks = chunker(code);

    // 2. Poi tokenizer: trasforma i chunk in token
    let tokens = tokenizer(chunks);

    println!("\n=== Tokenizzazione completata ===");
    println!("Totale token: {}", tokens.len());

    Ok(())
}