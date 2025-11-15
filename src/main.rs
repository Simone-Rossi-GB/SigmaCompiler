mod lexer;
mod compiler;
mod parser;
mod semantic;
mod codegen;

use std::env;
use std::fs;
use std::process::{self, Command};
use crate::compiler::compile;

fn main() {
    // Prendi gli argomenti
    let args: Vec<String> = env::args().collect();

    // Controlla che sia stato passato il file path
    if args.len() < 2 {
        print_usage(&args[0]);
        process::exit(1);
    }

    // Parsing dei flag
    let mut should_assemble = false;
    let mut should_execute = false;
    let mut file_path = String::new();
    let mut output_path = String::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-a" | "--assemble" => should_assemble = true,
            "-x" | "--execute" | "-r" | "--run" => {
                should_assemble = true;
                should_execute = true;
            },
            "-h" | "--help" => {
                print_usage(&args[0]);
                process::exit(0);
            },
            arg => {
                if file_path.is_empty() {
                    file_path = arg.to_string();
                } else if output_path.is_empty() {
                    output_path = arg.to_string();
                }
            }
        }
        i += 1;
    }

    if file_path.is_empty() {
        eprintln!("Errore: Nessun file di input specificato");
        print_usage(&args[0]);
        process::exit(1);
    }

    // Determina il nome del file di output assembly
    if output_path.is_empty() {
        output_path = if file_path.ends_with(".sgm") {
            file_path.replace(".sgm", ".s")
        } else {
            format!("{}.s", file_path)
        };
    }

    // Leggi il contenuto del file
    let code = match fs::read_to_string(&file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Errore nella lettura del file '{}': {}", file_path, err);
            process::exit(1);
        }
    };

    // Compila il codice Sigma -> RISC-V assembly
    match compile(&code, &output_path) {
        Ok(_) => println!("\n✓ Compilazione completata con successo!"),
        Err(err) => {
            eprintln!("✗ Errore di compilazione: {}", err);
            process::exit(1);
        }
    }

    // Assembla se richiesto
    if should_assemble {
        let executable_path = output_path.replace(".s", "");
        match assemble_riscv(&output_path, &executable_path) {
            Ok(_) => println!("✓ Assemblaggio completato: {}", executable_path),
            Err(err) => {
                eprintln!("✗ Errore durante l'assemblaggio: {}", err);
                eprintln!("Suggerimento: Installa il toolchain RISC-V:");
                eprintln!("  - Windows: https://github.com/stnolting/riscv-gcc-prebuilt");
                eprintln!("  - Linux: sudo apt install gcc-riscv64-linux-gnu");
                eprintln!("  - WSL: sudo apt install gcc-riscv64-linux-gnu qemu-user");
                process::exit(1);
            }
        }

        // Esegui con QEMU se richiesto
        if should_execute {
            println!("\n=== Esecuzione con QEMU ===");
            match execute_qemu(&executable_path) {
                Ok(_) => {},
                Err(err) => {
                    eprintln!("\n✗ Errore durante l'esecuzione: {}", err);
                    eprintln!("Suggerimento: Installa QEMU:");
                    eprintln!("  - Windows: https://qemu.weilnetz.de/w64/");
                    eprintln!("  - Linux: sudo apt install qemu-user");
                    process::exit(1);
                }
            }
        }
    }
}

fn print_usage(program: &str) {
    eprintln!("Uso: {} <file.sgm> [opzioni] [output.s]", program);
    eprintln!();
    eprintln!("Opzioni:");
    eprintln!("  -a, --assemble    Assembla il file .s in un eseguibile RISC-V");
    eprintln!("  -x, --execute     Assembla ed esegue il programma con QEMU");
    eprintln!("  -r, --run         Alias per --execute");
    eprintln!("  -h, --help        Mostra questo messaggio");
    eprintln!();
    eprintln!("Esempi:");
    eprintln!("  {} program.sgm              # Genera solo program.s", program);
    eprintln!("  {} program.sgm -a           # Genera program.s e assembla", program);
    eprintln!("  {} program.sgm -x           # Compila, assembla ed esegue", program);
}

fn assemble_riscv(asm_file: &str, output_file: &str) -> Result<(), String> {
    // Prova diversi nomi del compilatore RISC-V
    let gcc_variants = vec![
        "riscv64-unknown-elf-gcc",
        "riscv64-linux-gnu-gcc",
        "riscv32-unknown-elf-gcc",
    ];

    for gcc in &gcc_variants {
        if is_command_available(gcc) {
            let status = Command::new(gcc)
                .args(&[
                    "-march=rv32im",
                    "-mabi=ilp32",
                    asm_file,
                    "-o", output_file,
                    "-nostdlib",
                    "-static"
                ])
                .status()
                .map_err(|e| format!("Errore nell'esecuzione di {}: {}", gcc, e))?;

            if !status.success() {
                return Err(format!("{} è terminato con errore", gcc));
            }
            return Ok(());
        }
    }

    Err("Compilatore RISC-V non trovato (provato: riscv64-unknown-elf-gcc, riscv64-linux-gnu-gcc)".to_string())
}

fn execute_qemu(executable: &str) -> Result<(), String> {
    // Prova diversi nomi di QEMU user-mode
    let qemu_variants = vec![
        "qemu-riscv32",
        "qemu-riscv32-static",
    ];

    for qemu in &qemu_variants {
        if is_command_available(qemu) {
            let status = Command::new(qemu)
                .arg(executable)
                .status()
                .map_err(|e| format!("Errore nell'esecuzione di {}: {}", qemu, e))?;

            if !status.success() {
                return Err(format!("Il programma è terminato con codice {}",
                    status.code().unwrap_or(-1)));
            }
            return Ok(());
        }
    }

    // Su Windows, prova con WSL
    #[cfg(target_os = "windows")]
    {
        if is_command_available("wsl") {
            println!("→ Usando WSL per eseguire il programma...");

            // Converti il path in assoluto
            use std::path::Path;
            let abs_path = Path::new(executable)
                .canonicalize()
                .map_err(|e| format!("Impossibile trovare il file '{}': {}", executable, e))?;

            let abs_path_str = abs_path.to_string_lossy();

            // Converti il path Windows in path WSL
            // Rimuovi il prefisso \\?\ se presente (aggiunto da canonicalize su Windows)
            let path_clean = abs_path_str.strip_prefix(r"\\?\").unwrap_or(&abs_path_str);

            // Converti backslash in forward slash
            let wsl_path = path_clean.replace("\\", "/");

            // Converti C:/Users/... in /mnt/c/Users/...
            let wsl_path = if wsl_path.len() >= 2 && wsl_path.chars().nth(1) == Some(':') {
                let drive = wsl_path.chars().next().unwrap().to_lowercase();
                format!("/mnt/{}{}", drive, &wsl_path[2..])
            } else {
                wsl_path
            };

            let status = Command::new("wsl")
                .args(&["-d", "Ubuntu", "qemu-riscv32", &wsl_path])
                .status()
                .map_err(|e| format!("Errore nell'esecuzione di WSL: {}", e))?;

            if !status.success() {
                return Err(format!("Il programma è terminato con codice {}",
                    status.code().unwrap_or(-1)));
            }
            return Ok(());
        }
    }

    Err(format!(
        "QEMU user-mode non trovato.\n\
        Su Windows, installa WSL e qemu-user:\n\
        1. wsl --install -d Ubuntu\n\
        2. wsl\n\
        3. sudo apt update && sudo apt install qemu-user"
    ))
}

fn is_command_available(cmd: &str) -> bool {
    Command::new(cmd)
        .arg("--version")
        .output()
        .is_ok()
}