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

# Funzione sigma
sigma:
   # Prologo 
  addi sp, sp, -36
  mv s0, sp       # s0 = frame pointer
  sw ra, 32(s0)
   # VarDecl: vibes msg
  la a0, str_0
  sw a0, 0(s0)   # salva in stack msg
   # Print
  lw a0, 0(s0)   # load msg
   call print_string
   # VarDecl: based x
  li a0, 42
  sw a0, 4(s0)   # salva in stack x
   # Print
  lw a0, 4(s0)   # load x
   call print_int
   # VarDecl: based y
  lw a0, 4(s0)   # load x
   addi sp, sp, -4
   sw a0, 0(sp)
  li a0, 10
   lw a1, 0(sp)
   addi sp, sp, 4
    add a0, a1, a0
  sw a0, 8(s0)   # salva in stack y
   # Print
  lw a0, 8(s0)   # load y
   call print_int
   # Epilogo
  li a0, 0        # funzione ghost ritorna 0
  lw ra, 32(s0)
  addi sp, sp, 36
   ret



# ============================================================================
# Data Section
# ============================================================================

.data
str_0: .asciz "Hello World!"
.Lnewline: .byte 10

