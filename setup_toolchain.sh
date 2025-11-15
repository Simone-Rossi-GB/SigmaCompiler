#!/bin/bash

echo "========================================"
echo "  Sigma Compiler - Setup Toolchain"
echo "========================================"
echo ""

# Detect OS
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    OS="mac"
else
    OS="unknown"
fi

echo "Sistema operativo rilevato: $OS"
echo ""

# Check if running in WSL
if grep -qi microsoft /proc/version 2>/dev/null; then
    echo "🎉 WSL rilevato!"
    echo ""
fi

echo "Opzioni disponibili:"
echo "  1. Installa toolchain RISC-V"
echo "  2. Installa QEMU"
echo "  3. Installa tutto (RISC-V + QEMU)"
echo "  4. Verifica installazione"
echo "  5. Esci"
echo ""

read -p "Scegli un'opzione (1-5): " choice

case $choice in
    1)
        echo ""
        echo "=== Installazione RISC-V Toolchain ==="
        echo ""
        if [[ "$OS" == "linux" ]]; then
            if command -v apt &> /dev/null; then
                echo "Usando apt (Debian/Ubuntu)..."
                sudo apt update
                sudo apt install -y gcc-riscv64-linux-gnu
            elif command -v dnf &> /dev/null; then
                echo "Usando dnf (Fedora)..."
                sudo dnf install -y gcc-riscv64-linux-gnu
            elif command -v pacman &> /dev/null; then
                echo "Usando pacman (Arch)..."
                sudo pacman -S riscv64-linux-gnu-gcc
            else
                echo "Package manager non riconosciuto!"
                echo "Installa manualmente: gcc-riscv64-linux-gnu"
            fi
        elif [[ "$OS" == "mac" ]]; then
            echo "Usando Homebrew..."
            if ! command -v brew &> /dev/null; then
                echo "Homebrew non trovato! Installalo da https://brew.sh"
                exit 1
            fi
            brew tap riscv/riscv
            brew install riscv-tools
        fi
        echo ""
        echo "✓ Installazione completata!"
        ;;

    2)
        echo ""
        echo "=== Installazione QEMU ==="
        echo ""
        if [[ "$OS" == "linux" ]]; then
            if command -v apt &> /dev/null; then
                echo "Usando apt (Debian/Ubuntu)..."
                sudo apt update
                sudo apt install -y qemu-user qemu-user-static
            elif command -v dnf &> /dev/null; then
                echo "Usando dnf (Fedora)..."
                sudo dnf install -y qemu-user
            elif command -v pacman &> /dev/null; then
                echo "Usando pacman (Arch)..."
                sudo pacman -S qemu-user
            else
                echo "Package manager non riconosciuto!"
                echo "Installa manualmente: qemu-user"
            fi
        elif [[ "$OS" == "mac" ]]; then
            echo "Usando Homebrew..."
            if ! command -v brew &> /dev/null; then
                echo "Homebrew non trovato! Installalo da https://brew.sh"
                exit 1
            fi
            brew install qemu
        fi
        echo ""
        echo "✓ Installazione completata!"
        ;;

    3)
        echo ""
        echo "=== Installazione Completa ==="
        echo ""
        if [[ "$OS" == "linux" ]]; then
            if command -v apt &> /dev/null; then
                sudo apt update
                sudo apt install -y gcc-riscv64-linux-gnu qemu-user qemu-user-static
            elif command -v dnf &> /dev/null; then
                sudo dnf install -y gcc-riscv64-linux-gnu qemu-user
            elif command -v pacman &> /dev/null; then
                sudo pacman -S riscv64-linux-gnu-gcc qemu-user
            fi
        elif [[ "$OS" == "mac" ]]; then
            if command -v brew &> /dev/null; then
                brew tap riscv/riscv
                brew install riscv-tools qemu
            fi
        fi
        echo ""
        echo "✓ Installazione completata!"
        ;;

    4)
        echo ""
        echo "=== Verifica Installazione ==="
        echo ""

        # Check GCC
        if command -v riscv64-unknown-elf-gcc &> /dev/null; then
            echo "✓ riscv64-unknown-elf-gcc trovato"
            riscv64-unknown-elf-gcc --version | head -n1
        elif command -v riscv64-linux-gnu-gcc &> /dev/null; then
            echo "✓ riscv64-linux-gnu-gcc trovato"
            riscv64-linux-gnu-gcc --version | head -n1
        else
            echo "✗ Compilatore RISC-V non trovato"
        fi

        echo ""

        # Check QEMU
        if command -v qemu-riscv32 &> /dev/null; then
            echo "✓ qemu-riscv32 trovato"
            qemu-riscv32 --version | head -n1
        else
            echo "✗ QEMU non trovato"
        fi

        echo ""

        if command -v riscv64-linux-gnu-gcc &> /dev/null && command -v qemu-riscv32 &> /dev/null; then
            echo "🎉 Tutto configurato correttamente!"
            echo ""
            echo "Puoi ora eseguire:"
            echo "  cargo run -- program.sgm -x"
        else
            echo "⚠️  Alcuni componenti mancano. Installa con le opzioni 1-3."
        fi
        ;;

    5)
        echo "Uscita..."
        exit 0
        ;;

    *)
        echo "Scelta non valida!"
        exit 1
        ;;
esac

echo ""
read -p "Premi INVIO per continuare..."
