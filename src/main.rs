mod lexer;
mod compiler;
mod parser;
mod semantic;

use std::env;
use std::fs;
use std::process;
use crate::compiler::compile;

fn main() {
    // Prendi gli argomenti
    let args: Vec<String> = env::args().collect();

    // Controlla che sia stato passato il file path
    if args.len() < 2 {
        eprintln!("Uso: {} <file.sgm>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];

    // Leggi il contenuto del file
    let code = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Errore nella lettura del file '{}': {}", file_path, err);
            process::exit(1);
        }
    };

    // Compila il codice
    match compile(&code) {
        Ok(_) => println!("\n✓ Compilazione completata con successo!"),
        Err(err) => {
            eprintln!("✗ Errore di compilazione: {}", err);
            process::exit(1);
        }
    }
}