# 🧪 Testing del Compilatore Sigma con RARS

Questa guida spiega come testare il codice generato dal compilatore Sigma usando RARS (RISC-V Assembler and Runtime Simulator).

## 📋 Prerequisiti

1. **Java Runtime** (per eseguire RARS)
   ```bash
   java -version  # Verifica che Java sia installato
   ```

2. **RARS Simulator**
   - Download: https://github.com/TheThirdOne/rars/releases
   - Scarica il file JAR più recente (es. `rars1_6.jar`)

3. **Compilatore Sigma** (già presente in questo progetto)

## 🚀 Quick Start

### Metodo 1: Usa l'esempio già pronto

```bash
# Apri RARS con l'esempio già convertito
java -jar rars1_6.jar example_rars.s
```

Poi in RARS:
1. Clicca **Assemble** (o premi `F3`)
2. Clicca **Run** (o premi `F5`)
3. Dovresti vedere `11` stampato nella console

### Metodo 2: Converti il tuo codice

```bash
# 1. Compila un programma Sigma
cargo run -- test_ops.sigma

# 2. Converti il file .s generato per RARS
./convert_to_rars.sh test_ops.sigma.s

# 3. Apri in RARS
java -jar rars1_6.jar test_ops.sigma_rars.s
```

## 📝 Workflow Completo

### Step 1: Scrivi un programma Sigma

Crea un file `my_program.sigma`:
```sigma
bussin based sigma() {
    based x slay 5 + 3 * 2;
    flex x;
    yeet x;
}
```

### Step 2: Compila con Sigma Compiler

```bash
cargo run -- my_program.sigma
# Output: my_program.s (codice RISC-V RV32IM bare-metal)
```

### Step 3: Converti per RARS

```bash
./convert_to_rars.sh my_program.s
# Output: my_program_rars.s (codice RARS-compatible)
```

### Step 4: Esegui in RARS

```bash
java -jar rars1_6.jar my_program_rars.s
```

Oppure apri RARS con GUI:
```bash
java -jar rars1_6.jar
# Poi: File → Open → my_program_rars.s
```

### Step 5: Debug (opzionale)

In RARS:
1. **Assemble** il programma
2. Usa **Step** (F7) per eseguire un'istruzione alla volta
3. Guarda i **Registers** per vedere i valori nei registri
4. Guarda il **Data Segment** per vedere lo stack

## 🔧 Cosa fa lo script di conversione?

Lo script `convert_to_rars.sh` modifica automaticamente:

1. **Entry point**
   ```assembly
   # PRIMA (bare-metal):
   .global _start
   _start:
       call sigma
       li a7, 93    # syscall exit
       ecall

   # DOPO (RARS):
   .globl main
   main:
       call sigma
       li a7, 10    # syscall Exit
       ecall
   ```

2. **Helper functions**
   ```assembly
   # PRIMA (bare-metal - codice lungo):
   print_int:
       addi sp, sp, -20
       # ... 40+ linee di codice per convertire int→string
       li a7, 64    # syscall write
       ecall
       ret

   # DOPO (RARS - semplice):
   print_int:
       li a7, 1     # syscall PrintInt
       ecall
       li a0, 10    # '\n'
       li a7, 11    # syscall PrintChar
       ecall
       ret
   ```

3. **Lascia intatte** le funzioni utente (sigma, ecc.)

## 📚 File nel Progetto

| File | Descrizione |
|------|-------------|
| `example.txt` | Guida dettagliata alla conversione manuale |
| `convert_to_rars.sh` | Script automatico di conversione |
| `example_rars.s` | Esempio completo pronto per RARS |
| `TESTING_RARS.md` | Questa guida |
| `*.sigma` | Programmi sorgente Sigma |
| `*.s` | Assembly generato (bare-metal) |
| `*_rars.s` | Assembly convertito per RARS |

## 🐛 Debugging Tips

### 1. Visualizza lo Stack

In RARS, vai al pannello **Data Segment** e cerca l'area intorno a `sp` (stack pointer):
- `s0` = frame pointer (base del frame corrente)
- Variabili locali: `0(s0)`, `4(s0)`, `8(s0)`, ecc.

### 2. Breakpoint

Clicca sul numero di riga nel codice assembly per impostare un breakpoint.

### 3. Watch Registers

Nel pannello **Registers**, osserva:
- `a0` = primo argomento / valore di ritorno
- `sp` = stack pointer
- `s0` = frame pointer
- `ra` = return address

### 4. Single Step

Usa **Step** (F7) per eseguire una istruzione alla volta e vedere come cambiano i registri.

## ⚠️ Limitazioni RARS

RARS è un simulatore **educativo**, quindi:

1. ❌ Non supporta tutte le feature del bare-metal
2. ❌ Non ha un vero kernel Linux
3. ❌ Syscall limitate (no file I/O complesso, no network, ecc.)
4. ✅ Perfetto per testare logica e algoritmi
5. ✅ Ottimo per debug visuale

## 🔄 Conversione Manuale (se lo script non funziona)

Se lo script automatico ha problemi, vedi `example.txt` per le istruzioni manuali dettagliate.

### Passi principali:

1. Sostituisci `_start` con `main`
2. Sostituisci le funzioni `print_int` e `print_string` con versioni RARS
3. Cambia syscall 93 (exit) in syscall 10
4. Copia la funzione `sigma` e la sezione `.data` senza modifiche

## 📊 Syscall RARS vs Linux

| Operazione | Linux (a7) | RARS (a7) | Note |
|------------|-----------|-----------|------|
| Exit | 93 | 10 | |
| Print Int | 64 (write) | 1 | RARS più semplice |
| Print String | 64 (write) | 4 | RARS più semplice |
| Print Char | 64 (write) | 11 | |
| Read Int | - | 5 | Solo RARS |

## 🎯 Output Atteso

Per `test_ops.sigma`:
```
11
```
(Stampa il valore di `x = 5 + 3 * 2 = 11`)

## 📖 Risorse

- **RARS GitHub**: https://github.com/TheThirdOne/rars
- **RARS Wiki**: https://github.com/TheThirdOne/rars/wiki
- **RISC-V Specs**: https://riscv.org/technical/specifications/
- **RISC-V Green Card**: https://www.cl.cam.ac.uk/teaching/1617/ECAD+Arch/files/docs/RISCVGreenCardv8-20151013.pdf

## 🆘 Troubleshooting

### Problema: "Error in line X: Runtime exception"
**Soluzione**: Verifica che tutte le variabili siano inizializzate prima dell'uso.

### Problema: "Invalid or unimplemented instruction"
**Soluzione**: RARS potrebbe non supportare alcune istruzioni. Verifica che usi solo RV32IM base.

### Problema: "Null pointer exception"
**Soluzione**: Verifica che le stringhe abbiano label corrette nella sezione `.data`.

### Problema: Lo script fallisce
**Soluzione**: Converti manualmente seguendo `example.txt`.

## ✅ Checklist

Prima di aprire in RARS:

- [ ] Il file ha `.globl main` (non `_start`)
- [ ] `print_int` usa syscall 1 (non 64)
- [ ] `print_string` usa syscall 4 (non 64)
- [ ] Exit usa syscall 10 (non 93)
- [ ] La sezione `.data` è presente
- [ ] Le funzioni utente (sigma) non sono state modificate

## 🎉 Successo!

Se vedi l'output corretto in RARS, il tuo compilatore Sigma funziona! 🚀

Puoi ora:
1. Testare programmi più complessi
2. Aggiungere nuove feature al compilatore
3. Fare debug visuale dello stack e dei registri
4. Sperimentare con ottimizzazioni

Happy coding! 💻✨
