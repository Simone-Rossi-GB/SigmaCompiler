# 📌 Quick Reference - Sigma Compiler

## 🎯 Compilazione Rapida

```bash
# Compila un programma Sigma
cargo run -- program.sigma

# Compila e assembla (crea eseguibile RISC-V)
cargo run -- program.sigma -x
```

## 🧪 Testing con RARS

```bash
# 1. Compila
cargo run -- test.sigma

# 2. Converti per RARS
./convert_to_rars.sh test.s

# 3. Esegui in RARS
java -jar rars.jar test_rars.s
```

## 📁 File Importanti

| File | Uso |
|------|-----|
| `example.txt` | 📖 Guida completa conversione RARS |
| `TESTING_RARS.md` | 📚 Tutorial completo testing |
| `convert_to_rars.sh` | 🔧 Script conversione automatica |
| `example_rars.s` | ✅ Esempio pronto per RARS |
| `setup_toolchain.sh` | ⚙️ Setup toolchain RISC-V |

## 🔑 Syscall RARS

```assembly
# Print intero
li a0, 42      # valore da stampare
li a7, 1       # syscall PrintInt
ecall

# Print stringa
la a0, str     # indirizzo stringa
li a7, 4       # syscall PrintString
ecall

# Exit
li a7, 10      # syscall Exit
ecall
```

## 🐛 Debug Tips

### Vedere i registri
- Pannello **Registers** in RARS
- `s0` = frame pointer
- `a0` = valore di ritorno

### Vedere lo stack
- Pannello **Data Segment**
- Cerca indirizzo in `sp`
- Variabili: `0(s0)`, `4(s0)`, ...

### Single step
- `F7` = Step (una istruzione)
- `F8` = Step over (salta call)
- `F9` = Backstep (indietro)

## ⚡ Comandi Utili

```bash
# Compila release
cargo build --release

# Verifica toolchain
./setup_toolchain.sh
# Scegli opzione 4

# Genera solo assembly (senza assemblare)
cargo run -- program.sigma
```

## 📊 Layout dello Stack

```
sp_alto (indirizzi alti)
    ↓
    | ... vecchi frame ...
    |
s0 →| ← base del frame corrente
    | 0(s0)  = variabile locale 1
    | 4(s0)  = variabile locale 2
    | 8(s0)  = variabile locale 3
    | ...
    | N(s0)  = return address (ra)
    ↓
sp → (può muoversi per temporanei)
    ↓
sp_basso (indirizzi bassi)
```

## 🔄 Conversione Bare-metal → RARS

### Entry point
```assembly
# PRIMA:                 # DOPO:
.global _start           .globl main
_start:                  main:
    call sigma               call sigma
    li a7, 93                li a7, 10
    ecall                    ecall
```

### Print helpers
```assembly
# PRIMA: ~50 linee       # DOPO: ~5 linee
print_int:               print_int:
    # conversione int       li a7, 1
    # a stringa manuale     ecall
    li a7, 64                li a0, 10
    ecall                    li a7, 11
    ret                      ecall
                             ret
```

## ✅ Test Rapido

```bash
# 1. Compila esempio
cargo run -- test_ops.sigma

# 2. Converti
./convert_to_rars.sh test_ops.sigma.s

# 3. Testa in RARS
java -jar rars.jar test_ops.sigma_rars.s
# Output atteso: 11
```

## 🆘 Errori Comuni

| Errore | Causa | Fix |
|--------|-------|-----|
| `_start not found` | File non convertito | Usa `convert_to_rars.sh` |
| `segfault` | Offset stack errati | Usa versione aggiornata |
| `invalid instruction` | Toolchain sbagliato | Esegui `brew link riscv-gnu-toolchain` |
| `syscall error` | Syscall Linux in RARS | Converti con script |

## 📖 Link Utili

- **RARS**: https://github.com/TheThirdOne/rars/releases
- **RISC-V**: https://riscv.org
- **Homebrew RISC-V**: https://github.com/riscv/homebrew-riscv

## 💡 Pro Tips

1. **Usa RARS per debug**, non per produzione
2. **Frame pointer `s0`** è fisso, `sp` può muoversi
3. **Stack cresce verso il basso** (indirizzi decrescenti)
4. **Offset positivi** da `s0` per variabili locali
5. **Profondità espressioni** determina spazio temporanei

---

Per guida completa: vedi `TESTING_RARS.md` e `example.txt` 🚀
