pub fn chunker(code: &str) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut chunk: String = String::new();
    let mut in_string = false;
    let mut in_char = false;

    for c in code.chars() {
        // Gestione virgolette doppie (stringhe)
        if c == '"' && !in_char {
            chunk.push(c);
            if in_string {
                // Fine della stringa - aggiungi il chunk
                chunks.push(chunk.clone());
                chunk.clear();
                in_string = false;
            } else {
                // Inizio della stringa
                in_string = true;
            }
        }
        // Gestione apici singoli (caratteri)
        else if c == '\'' && !in_string {
            chunk.push(c);
            if in_char {
                // Fine del carattere - aggiungi il chunk
                chunks.push(chunk.clone());
                chunk.clear();
                in_char = false;
            } else {
                // Inizio del carattere
                in_char = true;
            }
        }
        // Se siamo dentro una stringa o un carattere, aggiungi tutto
        else if in_string || in_char {
            chunk.push(c);
        }

        // Simboli speciali - separali sempre
        else if "(){}[];,".contains(c) {
            if !chunk.is_empty() {
                chunks.push(chunk.clone());
                chunk.clear();
            }
            chunks.push(c.to_string());
        }

        // Spazi bianchi fuori dalle stringhe
        else if c.is_whitespace() {
            if !chunk.is_empty() {
                chunks.push(chunk.clone());
                chunk.clear();
            }
        }

        // Caratteri normali
        else {
            chunk.push(c);
        }
    }

    // Aggiungi l'ultimo chunk se non è vuoto
    if !chunk.is_empty() {
        chunks.push(chunk);
    }

    println!("\nChunks generati:");
    println!("Totale: {}", chunks.len());
    for (i, chunk) in chunks.iter().enumerate() {
        println!("  [{}] \"{}\"", i, chunk);
    }

    return chunks;
}