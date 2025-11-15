# 📋 Changelog & Bug Fixes - Sigma Compiler

## 🐛 Bug Risolti (Novembre 2025)

### ✅ Fix #1: Toolchain non trovato su macOS
**Problema:** `brew install riscv-tools` installava un metapackage vuoto senza binari.

**Soluzione:**
- Installare `riscv-gnu-toolchain` invece di `riscv-tools`
- Eseguire `brew link riscv-gnu-toolchain` per creare i symlink

**File modificati:**
- `setup_toolchain.sh` (linea 63)

**Commit:** `fix: toolchain installation on macOS`

---

### ✅ Fix #2: `.Lnewline` non definito
**Problema:** Il codice assembly faceva riferimento a `.Lnewline` ma la label non veniva generata se non c'erano stringhe letterali.

**Errore:**
```
undefined reference to `.Lnewline'
```

**Soluzione:** Generare sempre la sezione `.data` con `.Lnewline`.

**File modificati:**
- `src/codegen/riscv.rs:501-514`

**Codice prima:**
```rust
if !ctx.string_literals.is_empty() {
    output.push_str("\n.data\n");
    // ...
    output.push_str(".Lnewline: .byte 10\n\n");
}
```

**Codice dopo:**
```rust
// Genera sempre la sezione .data
output.push_str("\n.data\n");
// ... stringhe letterali ...
output.push_str(".Lnewline: .byte 10\n\n");
```

**Commit:** `fix: always generate .data section with .Lnewline`

---

### ✅ Fix #3: Segmentation Fault (errore 139)
**Problema:** Offset delle variabili errati causavano accessi fuori dall'area allocata dello stack.

**Sintomi:**
- Segmentation fault all'esecuzione
- Offset negativi (-4, -8, -12...) usati con `sp`
- Spazio stack insufficiente per temporanei

**Analisi del problema:**
```assembly
# CODICE ERRATO:
addi sp, sp, -20      # Alloca 20 byte
sw ra, 16(sp)         # ✓ ra dentro area (0 a 20)
sw a0, -4(sp)         # ✗ FUORI! (-4 è sotto sp)
```

**Soluzione - Parte 1:** Offset positivi invece di negativi

**File modificati:**
- `src/codegen/context.rs:37-45`

**Codice prima:**
```rust
pub fn allocate_variable(&mut self, name: String, var_type: Type) -> i32 {
    self.stack_offset -= 4; // Decrementa: -4, -8, -12...
    self.variables.insert(name.clone(), self.stack_offset);
    self.stack_offset
}
```

**Codice dopo:**
```rust
pub fn allocate_variable(&mut self, name: String, var_type: Type) -> i32 {
    let offset = self.stack_offset;
    self.stack_offset += 4; // Incrementa: 0, 4, 8, 12...
    self.variables.insert(name.clone(), offset);
    offset
}
```

**Soluzione - Parte 2:** Aggiungere spazio per temporanei

**File modificati:**
- `src/codegen/riscv.rs:48-55`

**Codice prima:**
```rust
let total_stack = (local_count + param_count) * 4 + 4; // Solo variabili + ra
```

**Codice dopo:**
```rust
let max_expr_depth = estimate_max_expression_depth(&func.body);
let temp_space = max_expr_depth * 4;
let total_stack = (local_count + param_count) * 4 + 4 + temp_space;
```

**Nuove funzioni aggiunte:**
- `estimate_max_expression_depth()`
- `statement_expr_depth()`
- `expr_depth()`

**Soluzione - Parte 3:** Frame pointer fisso

**Problema:** `sp` viene modificato durante espressioni (`addi sp, sp, -4`), quindi gli offset diventano errati.

**File modificati:**
- `src/codegen/riscv.rs:65, 73, 90, 118, 134, 337`

**Codice prima:**
```assembly
addi sp, sp, -20
sw ra, 16(sp)        # OK inizialmente
sw a0, 0(sp)         # salva variabile

addi sp, sp, -4      # sp cambia!
lw a0, 0(sp)         # ERRORE! Ora 0(sp) punta al temporaneo, non alla variabile
```

**Codice dopo:**
```assembly
addi sp, sp, -44
mv s0, sp            # s0 = frame pointer FISSO
sw ra, 40(s0)        # Usa s0
sw a0, 0(s0)         # Usa s0 per variabili

addi sp, sp, -4      # sp cambia, ma s0 no!
lw a0, 0(s0)         # OK! s0 punta sempre alla base
```

**Commit:** `fix: segfault due to incorrect stack offsets`

**Layout stack corretto:**
```
sp_alto (indirizzi alti)
    ↓
s0 →| ← Base del frame (FISSO)
    | 0(s0)  = variabile 1
    | 4(s0)  = variabile 2
    | 8(s0)  = variabile 3
    | ...
    | 40(s0) = return address (ra)
    ↓
sp → (può muoversi per temporanei)
```

**Commit:** `feat: add frame pointer for stable variable access`

---

### ✅ Fix #4: Esecuzione non supportata su macOS
**Problema:** QEMU user-mode non disponibile su macOS tramite Homebrew.

**Soluzione:** Messaggi di errore specifici per OS e documentazione per RARS.

**File modificati:**
- `src/main.rs:185-275`

**Codice aggiunto:**
```rust
#[cfg(target_os = "macos")]
return Err(format!(
    "Impossibile eseguire il programma.\n\
    Su macOS, QEMU user-mode non è disponibile nativamente.\n\
    Opzioni:\n\
    1. Usa solo compilazione (senza -x)\n\
    2. Installa spike + pk: brew install riscv-isa-sim riscv-pk\n\
    3. Usa Docker/VM Linux con qemu-user"
));
```

**Commit:** `feat: add macOS-specific error messages`

---

## 📝 Documentazione Aggiunta

### File creati:

1. **`example.txt`** (364 righe)
   - Guida completa conversione bare-metal → RARS
   - Differenze syscall dettagliate
   - Esempio completo step-by-step

2. **`TESTING_RARS.md`** (252 righe)
   - Tutorial completo testing con RARS
   - Workflow step-by-step
   - Debugging tips
   - Troubleshooting

3. **`QUICK_REFERENCE.md`** (167 righe)
   - Quick reference per comandi comuni
   - Syscall RARS
   - Debug tips rapidi
   - Checklist

4. **`convert_to_rars.sh`** (114 righe)
   - Script automatico di conversione
   - Sostituisce entry point
   - Sostituisce helper functions
   - Mantiene intatte le funzioni utente

5. **`example_rars.s`** (186 righe)
   - Esempio completo pronto per RARS
   - Codice commentato linea per linea
   - Output atteso documentato

6. **`CHANGELOG_FIXES.md`** (questo file)
   - Riepilogo di tutti i fix
   - Before/after code
   - Commit messages

---

## ⚠️ Note Importanti

### Syscall Write (64) vs PrintChar (11)

**CRITICO:** Il compilatore Sigma genera questo codice per il newline:

```assembly
# Bare-metal (ERRATO in RARS):
li a0, 1           # stdout
la a1, .Lnewline   # indirizzo di .byte 10
li a2, 1           # lunghezza
li a7, 64          # syscall write
ecall
```

**Deve essere convertito a:**

```assembly
# RARS (CORRETTO):
li a0, 10          # carattere '\n' (ASCII 10)
li a7, 11          # syscall PrintChar
ecall
```

Questo vale per:
- `print_int` (fine funzione)
- `print_string` (fine funzione)

Lo script `convert_to_rars.sh` fa questa conversione automaticamente.

---

## 🔄 Processo di Testing

### 1. Sviluppo
```bash
cargo run -- program.sigma
```

### 2. Test su macOS (solo compilazione)
```bash
cargo run -- program.sigma
# Verifica che program.s sia generato
```

### 3. Test con RARS
```bash
./convert_to_rars.sh program.s
java -jar rars.jar program_rars.s
```

### 4. Test su Linux (esecuzione completa)
```bash
cargo run -- program.sigma -x
# Compila, assembla ed esegue con QEMU
```

---

## 📊 Metriche

### Righe di codice modificate:
- `src/codegen/riscv.rs`: ~60 righe modificate, ~50 righe aggiunte
- `src/codegen/context.rs`: ~5 righe modificate
- `src/main.rs`: ~30 righe modificate
- `setup_toolchain.sh`: ~5 righe modificate

### Documentazione aggiunta:
- ~1,100 righe di documentazione
- 6 nuovi file
- 1 script bash

### Test cases verificati:
- ✅ `test_ops.sigma` - Operazioni aritmetiche
- ✅ `simple.sgm` - Print base
- ✅ `example_rars.s` - Esempio completo

---

## 🚀 Prossimi Passi

### Features da aggiungere:
- [ ] Supporto per array
- [ ] Supporto per struct
- [ ] Ottimizzazioni (constant folding, ecc.)
- [ ] More test cases

### Documentazione:
- [ ] Video tutorial
- [ ] More examples
- [ ] Performance benchmarks

---

## 📖 Riferimenti

- **RISC-V Spec:** https://riscv.org/technical/specifications/
- **RARS:** https://github.com/TheThirdOne/rars
- **Linux Syscalls:** https://man7.org/linux/man-pages/man2/syscalls.2.html
- **RARS Syscalls:** Help → Syscalls (in RARS)

---

**Autore:** Claude & Simone
**Data:** Novembre 2025
**Versione Compilatore:** 1.0.0-alpha
**Target:** RISC-V RV32IM
