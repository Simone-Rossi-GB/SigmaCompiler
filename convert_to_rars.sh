#!/bin/bash
# Script per convertire codice RISC-V Sigma in formato RARS
# Uso: ./convert_to_rars.sh <file.s>

set -e

if [ $# -eq 0 ]; then
    echo "❌ Errore: Specificare un file .s da convertire"
    echo "Uso: ./convert_to_rars.sh <file.s>"
    echo ""
    echo "Esempio:"
    echo "  ./convert_to_rars.sh test_ops.sigma.s"
    exit 1
fi

INPUT="$1"
OUTPUT="${INPUT%.s}_rars.s"

if [ ! -f "$INPUT" ]; then
    echo "❌ Errore: File '$INPUT' non trovato"
    exit 1
fi

echo "🔄 Conversione da bare-metal Linux a RARS..."
echo "   Input:  $INPUT"
echo "   Output: $OUTPUT"
echo ""

# Crea il file di output con header RARS
cat > "$OUTPUT" << 'RARS_TEMPLATE'
# ============================================================================
# Convertito automaticamente per RARS (RISC-V Assembler and Runtime Simulator)
# Generato da: Sigma Manny Compiler
# ============================================================================

.text
.globl main

# Entry point per RARS
main:
    call sigma
    # Exit con valore di ritorno in a0
    li a7, 10      # syscall Exit
    ecall

# ============================================================================
# Helper Functions (RARS syscall version)
# ============================================================================

# Stampa intero in a0
print_int:
    # RARS PrintInt: stampa direttamente il valore in a0
    li a7, 1       # syscall PrintInt
    ecall

    # Stampa newline
    li a0, 10      # '\n'
    li a7, 11      # syscall PrintChar
    ecall
    ret

# Stampa stringa (indirizzo in a0)
print_string:
    # RARS PrintString: a0 contiene l'indirizzo della stringa
    li a7, 4       # syscall PrintString
    ecall

    # Stampa newline
    li a0, 10      # '\n'
    li a7, 11      # syscall PrintChar
    ecall
    ret

# ============================================================================
# User Functions (generato dal compilatore Sigma)
# ============================================================================

RARS_TEMPLATE

# Estrai la funzione sigma (e altre funzioni utente)
# Cerca da "# Funzione" fino a ".data" (escludendo .data stesso)
echo "📝 Estraendo funzioni utente..."

# Usa awk per estrarre tutto tra "# Funzione" e ".data"
awk '/^# Funzione /,/^\.data$/ {
    if ($0 !~ /^\.data$/) print
}' "$INPUT" >> "$OUTPUT"

# Aggiungi sezione .data se presente
if grep -q "^\.data" "$INPUT"; then
    echo "" >> "$OUTPUT"
    echo "# ============================================================================" >> "$OUTPUT"
    echo "# Data Section" >> "$OUTPUT"
    echo "# ============================================================================" >> "$OUTPUT"
    echo "" >> "$OUTPUT"

    # Estrai sezione .data
    sed -n '/^\.data$/,/^$/p' "$INPUT" >> "$OUTPUT"
    echo "📦 Sezione .data inclusa"
fi

echo ""
echo "✅ Conversione completata con successo!"
echo ""
echo "📁 File generato: $OUTPUT"
echo ""
echo "🚀 Per eseguire in RARS:"
echo "   1. Scarica RARS da: https://github.com/TheThirdOne/rars/releases"
echo "   2. Esegui: java -jar rars.jar"
echo "   3. Apri il file: $OUTPUT"
echo "   4. Premi 'Assemble' (F3) poi 'Run' (F5)"
echo ""
echo "💡 Per info dettagliate, vedi: example.txt"
echo ""
