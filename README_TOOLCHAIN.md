# 🚀 Guida Completa: Sigma Compiler

## 📖 Indice
1. [Uso del Compiler](#uso-del-compiler)
2. [Installazione Toolchain RISC-V](#installazione-toolchain-risc-v)
3. [Installazione QEMU](#installazione-qemu)
4. [Esempi](#esempi)

---

## 🎯 Uso del Compiler

### Sintassi Base
```bash
cargo run -- <file.sgm> [opzioni] [output.s]
```

### Opzioni Disponibili
- **Nessuna opzione**: Genera solo il file assembly `.s`
- **`-a, --assemble`**: Compila e assembla in eseguibile RISC-V
- **`-x, --execute`**: Compila, assembla ed esegue con QEMU
- **`-r, --run`**: Alias per `--execute`
- **`-h, --help`**: Mostra la guida

### Esempi di Utilizzo

#### 1. Solo Compilazione (Sigma → Assembly)
```bash
cargo run -- program.sgm
# Output: program.s
```

#### 2. Compilazione + Assemblaggio
```bash
cargo run -- program.sgm -a
# Output: program.s + eseguibile program
```

#### 3. Compilazione + Assemblaggio + Esecuzione
```bash
cargo run -- program.sgm -x
# Output: program.s + program + esecuzione immediata
```

---

## 🔧 Installazione Toolchain RISC-V

### Windows

#### Opzione 1: MSYS2 (Raccomandato)
1. Installa MSYS2 da: https://www.msys2.org/
2. Apri MSYS2 MINGW64 e esegui:
```bash
pacman -S mingw-w64-x86_64-riscv64-unknown-elf-gcc
```
3. Aggiungi al PATH: `C:\msys64\mingw64\bin`

#### Opzione 2: Binari Precompilati
1. Scarica da: https://github.com/stnolting/riscv-gcc-prebuilt
2. Estrai in una cartella (es. `C:\riscv`)
3. Aggiungi al PATH: `C:\riscv\bin`

### Linux (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install gcc-riscv64-linux-gnu
```

### WSL (Windows Subsystem for Linux) - Raccomandato
```bash
# Installa WSL se non ce l'hai
wsl --install

# Dentro WSL:
sudo apt update
sudo apt install gcc-riscv64-linux-gnu qemu-user
```

### Verifica Installazione
```bash
riscv64-unknown-elf-gcc --version
# oppure
riscv64-linux-gnu-gcc --version
```

---

## 🖥️ Installazione QEMU

### Windows
1. Scarica da: https://qemu.weilnetz.de/w64/
2. Installa ed aggiungi al PATH

Oppure con Chocolatey:
```bash
choco install qemu
```

### Linux (Ubuntu/Debian)
```bash
sudo apt install qemu-user qemu-user-static
```

### WSL
```bash
sudo apt install qemu-user
```

### Verifica Installazione
```bash
qemu-riscv32 --version
```

---

## 💡 Esempi Completi

### Esempio 1: Hello World
**File:** `hello.sgm`
```sigma
bussin ghost sigma() {
    vibes msg slay "Hello, World!";
    flex msg;
}
```

**Compilazione ed esecuzione:**
```bash
cargo run -- hello.sgm -x
```

**Output:**
```
✓ Codice RISC-V generato in: hello.s
✓ Assemblaggio completato: hello
=== Esecuzione con QEMU ===
Hello, World!
```

### Esempio 2: Ciclo e Calcoli
**File:** `loop.sgm`
```sigma
bussin ghost sigma() {
    based somma slay 0;

    mewing (somma < 100) {
        somma slay somma + 10;
        flex somma;
    }
}
```

**Solo assembly:**
```bash
cargo run -- loop.sgm
```

**Con esecuzione:**
```bash
cargo run -- loop.sgm -x
```

### Esempio 3: Pipeline Manuale

Se il toolchain non è installato, puoi fare tutto manualmente:

```bash
# 1. Compila Sigma -> Assembly
cargo run -- program.sgm

# 2. Assembla manualmente (se hai installato il toolchain dopo)
riscv64-unknown-elf-gcc -march=rv32im -mabi=ilp32 program.s -o program -nostdlib -static

# 3. Esegui con QEMU
qemu-riscv32 program
```

---

## 🐛 Troubleshooting

### Errore: "Compilatore RISC-V non trovato"
**Causa:** Il toolchain RISC-V non è installato o non è nel PATH

**Soluzione:**
1. Installa il toolchain (vedi sezione sopra)
2. Verifica che sia nel PATH: `riscv64-unknown-elf-gcc --version`
3. Riavvia il terminale dopo l'installazione

### Errore: "QEMU non trovato"
**Causa:** QEMU non è installato o non è nel PATH

**Soluzione:**
1. Installa QEMU (vedi sezione sopra)
2. Verifica: `qemu-riscv32 --version`

### Errore di Assemblaggio
**Causa:** Errore nel codice assembly generato

**Soluzione:**
1. Controlla il file `.s` generato
2. Verifica che il programma Sigma sia corretto
3. Apri un issue su GitHub con il codice che genera l'errore

### Il programma compila ma non si esegue
**Causa:** Possibile errore di runtime

**Soluzione:**
1. Esegui manualmente con debug:
```bash
qemu-riscv32 -d in_asm,cpu program
```
2. Controlla il codice assembly in `program.s`

---

## 📚 Riferimenti Utili

- **RISC-V Spec:** https://riscv.org/technical/specifications/
- **QEMU Docs:** https://www.qemu.org/docs/master/
- **Sigma Language Syntax:** Vedi `src/lexer/tokenizer.rs` per le keyword

---

## 🎓 Workflow Consigliato

### Per Sviluppo Rapido (con WSL)
```bash
# Setup una volta sola
wsl --install
# Dentro WSL:
sudo apt install gcc-riscv64-linux-gnu qemu-user

# Ogni volta che compili:
cargo run -- program.sgm -x
```

### Per Produzione
```bash
# Compila e genera eseguibile standalone
cargo run -- program.sgm -a

# Distribuisci il file binario generato
# (può essere eseguito con qemu-riscv32 su qualsiasi sistema)
```

---

## 🔥 Tips & Tricks

### Automatizza con Script
**build_and_run.sh** (Linux/WSL):
```bash
#!/bin/bash
cargo run -- "$1" -x
```

**build_and_run.bat** (Windows):
```batch
@echo off
cargo run -- %1 -x
```

### Usa Alias
```bash
# Aggiungi a ~/.bashrc o ~/.zshrc
alias sigma="cargo run --quiet --"

# Ora puoi fare:
sigma program.sgm -x
```

### Debug Assembly
```bash
# Genera assembly con commenti dettagliati
cargo run -- program.sgm

# Visualizza assembly
cat program.s | less
```

---

**Made with ❤️ by Sigma Compiler Team**
