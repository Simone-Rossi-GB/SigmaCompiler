# ============================================================================
# ESEMPIO COMPLETO PER RARS - Pronto all'uso!
# ============================================================================
# Questo è un esempio completo di codice Sigma convertito per RARS.
# Calcola: x = 5 + 3 * 2 = 11
#          y = 10 - 4 / 2 = 8
# Poi stampa x (dovrebbe stampare 11)
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
# Funzione principale: sigma
# ============================================================================

sigma:
    # Prologo - alloca 44 byte sullo stack
    addi sp, sp, -44
    mv s0, sp       # s0 = frame pointer (punta alla base del frame)
    sw ra, 40(s0)   # salva return address

    # ========================================================================
    # VarDecl: based x = 5 + 3 * 2
    # ========================================================================
    # Calcola 5 + (3 * 2) seguendo la precedenza degli operatori

    # Push 5 sullo stack temporaneo
    li a0, 5
    addi sp, sp, -4
    sw a0, 0(sp)

    # Push 3 sullo stack temporaneo
    li a0, 3
    addi sp, sp, -4
    sw a0, 0(sp)

    # Carica 2 e moltiplica: 3 * 2 = 6
    li a0, 2
    lw a1, 0(sp)     # a1 = 3
    addi sp, sp, 4   # pop
    mul a0, a1, a0   # a0 = 3 * 2 = 6

    # Somma: 5 + 6 = 11
    lw a1, 0(sp)     # a1 = 5
    addi sp, sp, 4   # pop
    add a0, a1, a0   # a0 = 5 + 6 = 11

    # Salva x nel frame (offset 0)
    sw a0, 0(s0)     # x = 11

    # ========================================================================
    # VarDecl: based y = 10 - 4 / 2
    # ========================================================================
    # Calcola 10 - (4 / 2) seguendo la precedenza degli operatori

    # Push 10 sullo stack temporaneo
    li a0, 10
    addi sp, sp, -4
    sw a0, 0(sp)

    # Push 4 sullo stack temporaneo
    li a0, 4
    addi sp, sp, -4
    sw a0, 0(sp)

    # Carica 2 e dividi: 4 / 2 = 2
    li a0, 2
    lw a1, 0(sp)     # a1 = 4
    addi sp, sp, 4   # pop
    div a0, a1, a0   # a0 = 4 / 2 = 2

    # Sottrai: 10 - 2 = 8
    lw a1, 0(sp)     # a1 = 10
    addi sp, sp, 4   # pop
    sub a0, a1, a0   # a0 = 10 - 2 = 8

    # Salva y nel frame (offset 4)
    sw a0, 4(s0)     # y = 8

    # ========================================================================
    # VarDecl: based bigger = x > y
    # ========================================================================
    # Confronta se 11 > 8 (dovrebbe essere 0 = false, perché slt è "less than")

    # Carica x
    lw a0, 0(s0)     # a0 = x = 11
    addi sp, sp, -4
    sw a0, 0(sp)

    # Carica y e confronta
    lw a0, 4(s0)     # a0 = y = 8
    lw a1, 0(sp)     # a1 = x = 11
    addi sp, sp, 4   # pop
    slt a0, a0, a1   # a0 = (y < x) = (8 < 11) = 1 (true)

    # Salva bigger nel frame (offset 8)
    sw a0, 8(s0)     # bigger = 1

    # ========================================================================
    # VarDecl: based equal = x == 10
    # ========================================================================
    # Verifica se x == 10 (dovrebbe essere 0 = false, perché 11 != 10)

    # Carica x
    lw a0, 0(s0)     # a0 = x = 11
    addi sp, sp, -4
    sw a0, 0(sp)

    # Confronta con 10
    li a0, 10
    lw a1, 0(sp)     # a1 = x = 11
    addi sp, sp, 4   # pop
    sub a0, a1, a0   # a0 = 11 - 10 = 1
    seqz a0, a0      # a0 = (1 == 0) = 0 (false)

    # Salva equal nel frame (offset 12)
    sw a0, 12(s0)    # equal = 0

    # ========================================================================
    # Print x (dovrebbe stampare 11)
    # ========================================================================
    lw a0, 0(s0)     # a0 = x = 11
    call print_int   # Stampa: 11

    # ========================================================================
    # Return x
    # ========================================================================
    lw a0, 0(s0)     # a0 = x = 11 (valore di ritorno)

    # Epilogo - ripristina stack e ritorna
    lw ra, 40(s0)    # ripristina return address
    addi sp, sp, 44  # dealloca stack frame
    ret              # ritorna al chiamante (main)

# ============================================================================
# Data Section
# ============================================================================

.data
.Lnewline: .byte 10

# ============================================================================
# OUTPUT ATTESO IN RARS
# ============================================================================
# Quando esegui questo programma in RARS, dovresti vedere:
#
# 11
#
# (Il valore di x viene stampato)
# ============================================================================
