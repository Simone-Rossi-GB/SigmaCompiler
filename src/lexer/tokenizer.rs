use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::{Enumerate, Map};

#[derive(Debug, Clone)]
pub enum Token {
    // Tipi di dato
    Based,      // int
    SuperBased, // long
    Chill,      // float
    Vibes,      // string
    Ghost,      // void

    // Valori booleani
    Cap,        // false
    Fr,         // true

    // Operatori e keywords
    Slay,       // = (assignment)
    Flex,       // print
    Yeet,       // return

    // Strutture di controllo
    SixSeven,   // for
    Ong,        // if
    Mewing,     // while
    Ohio,       // break

    // Funzioni e strutture
    Bussin,     // function
    Sigma,      // main
    Skibidi,    // struct

    // Puntatori
    Gyatt,      // pointer

    // Identificatori e letterali
    Rizz(String),      // identifier/variable name
    Number(i64),       // numero
    StringLit(String), // stringa

    // Simboli
    OpenParen,    // (
    CloseParen,   // )
    OpenBrace,    // {
    CloseBrace,   // }
    Semicolon,    // ;
    Comma,        // ,

    // Unknown
    Unknown(String),
}

fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut chars = s.chars();
    let first = chars.next().unwrap();

    // Primo carattere deve essere lettera o underscore
    if !first.is_alphabetic() && first != '_' {
        return false;
    }

    // Altri caratteri possono essere alfanumerici o underscore
    chars.all(|c| c.is_alphanumeric() || c == '_')
}

pub fn tokenizer(chunks: Vec<String>) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    for chunk in chunks.iter() {
        // Skip chunk vuoti
        if chunk.is_empty() {
            continue;
        }

        // 1. Prima controlla le keywords
        if chunk.eq("based") {
            tokens.push(Token::Based)
        } else if chunk.eq("chill") {
            tokens.push(Token::Chill)
        } else if chunk.eq("superBased") {
            tokens.push(Token::SuperBased)
        } else if chunk.eq("vibes") {
            tokens.push(Token::Vibes)
        } else if chunk.eq("ghost") {
            tokens.push(Token::Ghost)
        } else if chunk.eq("cap") {
            tokens.push(Token::Cap)
        } else if chunk.eq("fr") {
            tokens.push(Token::Fr)
        } else if chunk.eq("slay") {
            tokens.push(Token::Slay)
        } else if chunk.eq("flex") {
            tokens.push(Token::Flex)
        } else if chunk.eq("yeet") {
            tokens.push(Token::Yeet)
        } else if chunk.eq("sixSeven") {
            tokens.push(Token::SixSeven)
        } else if chunk.eq("ong") {
            tokens.push(Token::Ong)
        } else if chunk.eq("mewing") {
            tokens.push(Token::Mewing)
        } else if chunk.eq("ohio") {
            tokens.push(Token::Ohio)
        } else if chunk.eq("bussin") {
            tokens.push(Token::Bussin)
        } else if chunk.eq("sigma") {
            tokens.push(Token::Sigma)
        } else if chunk.eq("skibidi") {
            tokens.push(Token::Skibidi)
        } else if chunk.eq("gyatt") {
            tokens.push(Token::Gyatt)

        // 2. Simboli
        } else if chunk.eq("(") {
            tokens.push(Token::OpenParen)
        } else if chunk.eq(")") {
            tokens.push(Token::CloseParen)
        } else if chunk.eq("{") {
            tokens.push(Token::OpenBrace)
        } else if chunk.eq("}") {
            tokens.push(Token::CloseBrace)
        } else if chunk.eq(";") {
            tokens.push(Token::Semicolon)
        } else if chunk.eq(",") {
            tokens.push(Token::Comma)

        // 3. Numeri
        } else if let Ok(num) = chunk.parse::<i64>() {
            tokens.push(Token::Number(num))

        // 4. Stringhe letterali (tra virgolette)
        } else if chunk.starts_with('"') && chunk.ends_with('"') && chunk.len() > 1 {
            let content = chunk[1..chunk.len()-1].to_string();
            tokens.push(Token::StringLit(content))

        // 5. Identificatori (Rizz)
        } else if is_valid_identifier(chunk) {
            tokens.push(Token::Rizz(chunk.clone()))

        // 6. Unknown
        } else {
            tokens.push(Token::Unknown(chunk.clone()))
        }
    }

    println!("\nTokens generati:");
    for token in tokens.iter() {
        println!("{:?}", token);
    }

    return tokens;
}