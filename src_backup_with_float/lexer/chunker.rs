pub fn chunker(code: &str) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut chunk: String = String::new();
    let mut in_string = false;
    let mut in_char = false;

    let chars: Vec<char> = code.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

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
            i += 1;
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
            i += 1;
        }
        // Se siamo dentro una stringa o un carattere, aggiungi tutto
        else if in_string || in_char {
            chunk.push(c);
            i += 1;
        }
        // Operatori a due caratteri: ==, !=, <=, >=
        else if i + 1 < chars.len() {
            let next_c = chars[i + 1];
            if (c == '=' && next_c == '=') ||
               (c == '!' && next_c == '=') ||
               (c == '<' && next_c == '=') ||
               (c == '>' && next_c == '=') {
                // Salva chunk corrente se non vuoto
                if !chunk.is_empty() {
                    chunks.push(chunk.clone());
                    chunk.clear();
                }
                // Aggiungi operatore a due caratteri
                chunks.push(format!("{}{}", c, next_c));
                i += 2;
            }
            // Operatori singoli: +, -, *, /, =, <, >, !
            else if "+-*/=<>!".contains(c) {
                if !chunk.is_empty() {
                    chunks.push(chunk.clone());
                    chunk.clear();
                }
                chunks.push(c.to_string());
                i += 1;
            }
            // Simboli speciali - separali sempre
            else if "(){}[];,".contains(c) {
                if !chunk.is_empty() {
                    chunks.push(chunk.clone());
                    chunk.clear();
                }
                chunks.push(c.to_string());
                i += 1;
            }
            // Spazi bianchi
            else if c.is_whitespace() {
                if !chunk.is_empty() {
                    chunks.push(chunk.clone());
                    chunk.clear();
                }
                i += 1;
            }
            // Caratteri normali
            else {
                chunk.push(c);
                i += 1;
            }
        }
        // Ultimo carattere - stessa logica ma senza guardare next_c
        else {
            if "+-*/=<>!".contains(c) {
                if !chunk.is_empty() {
                    chunks.push(chunk.clone());
                    chunk.clear();
                }
                chunks.push(c.to_string());
            }
            else if "(){}[];,".contains(c) {
                if !chunk.is_empty() {
                    chunks.push(chunk.clone());
                    chunk.clear();
                }
                chunks.push(c.to_string());
            }
            else if c.is_whitespace() {
                if !chunk.is_empty() {
                    chunks.push(chunk.clone());
                    chunk.clear();
                }
            }
            else {
                chunk.push(c);
            }
            i += 1;
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