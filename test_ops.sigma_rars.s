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
  addi sp, sp, -44
  mv s0, sp       # s0 = frame pointer
  sw ra, 40(s0)
   # VarDecl: based x
  li a0, 5
   addi sp, sp, -4
   sw a0, 0(sp)
  li a0, 3
   addi sp, sp, -4
   sw a0, 0(sp)
  li a0, 2
   lw a1, 0(sp)
   addi sp, sp, 4
    mul a0, a1, a0
   lw a1, 0(sp)
   addi sp, sp, 4
    add a0, a1, a0
  sw a0, 0(s0)   # salva in stack x
   # VarDecl: based y
  li a0, 10
   addi sp, sp, -4
   sw a0, 0(sp)
  li a0, 4
   addi sp, sp, -4
   sw a0, 0(sp)
  li a0, 2
   lw a1, 0(sp)
   addi sp, sp, 4
    div a0, a1, a0
   lw a1, 0(sp)
   addi sp, sp, 4
    sub a0, a1, a0
  sw a0, 4(s0)   # salva in stack y
   # VarDecl: based bigger
  lw a0, 0(s0)   # load x
   addi sp, sp, -4
   sw a0, 0(sp)
  lw a0, 4(s0)   # load y
   lw a1, 0(sp)
   addi sp, sp, 4
    slt a0, a0, a1
  sw a0, 8(s0)   # salva in stack bigger
   # VarDecl: based equal
  lw a0, 0(s0)   # load x
   addi sp, sp, -4
   sw a0, 0(sp)
  li a0, 10
   lw a1, 0(sp)
   addi sp, sp, 4
   sub a0, a1, a0
   seqz a0, a0
  sw a0, 12(s0)   # salva in stack equal
   # Print
  lw a0, 0(s0)   # load x
   call print_int
   # Return
  lw a0, 0(s0)   # load x
   # Epilogo
  lw ra, 40(s0)
  addi sp, sp, 44
   ret



# ============================================================================
# Data Section
# ============================================================================

.data
.Lnewline: .byte 10