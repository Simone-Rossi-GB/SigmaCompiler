@echo off
echo ========================================
echo   Sigma Compiler - Setup Toolchain
echo ========================================
echo.

echo Questo script ti aiuta a configurare il toolchain RISC-V
echo.

echo Opzioni disponibili:
echo   1. Installa con MSYS2 (Raccomandato)
echo   2. Download binari precompilati
echo   3. Usa WSL (Windows Subsystem for Linux)
echo   4. Verifica installazione esistente
echo   5. Esci
echo.

set /p choice="Scegli un'opzione (1-5): "

if "%choice%"=="1" goto msys2
if "%choice%"=="2" goto prebuilt
if "%choice%"=="3" goto wsl
if "%choice%"=="4" goto verify
if "%choice%"=="5" goto end

echo Scelta non valida!
pause
goto end

:msys2
echo.
echo === Installazione con MSYS2 ===
echo.
echo 1. Scarica MSYS2 da: https://www.msys2.org/
echo 2. Installa MSYS2
echo 3. Apri "MSYS2 MINGW64" dal menu Start
echo 4. Esegui: pacman -S mingw-w64-x86_64-riscv64-unknown-elf-gcc
echo 5. Aggiungi al PATH: C:\msys64\mingw64\bin
echo.
echo Vuoi aprire il sito di MSYS2? (s/n)
set /p open="Risposta: "
if /i "%open%"=="s" start https://www.msys2.org/
pause
goto end

:prebuilt
echo.
echo === Download binari precompilati ===
echo.
echo 1. Apri: https://github.com/stnolting/riscv-gcc-prebuilt
echo 2. Scarica l'ultima release per Windows
echo 3. Estrai in C:\riscv
echo 4. Aggiungi C:\riscv\bin al PATH
echo.
echo Vuoi aprire la pagina GitHub? (s/n)
set /p open="Risposta: "
if /i "%open%"=="s" start https://github.com/stnolting/riscv-gcc-prebuilt
pause
goto end

:wsl
echo.
echo === Installazione con WSL (Raccomandato!) ===
echo.
echo WSL ti permette di usare Linux dentro Windows.
echo E' l'opzione piu' semplice e affidabile!
echo.
echo 1. Apri PowerShell come Amministratore
echo 2. Esegui: wsl --install
echo 3. Riavvia il PC
echo 4. Apri "Ubuntu" dal menu Start
echo 5. Esegui: sudo apt update
echo 6. Esegui: sudo apt install gcc-riscv64-linux-gnu qemu-user
echo 7. Fatto! Ora puoi usare 'wsl' prima dei comandi
echo.
echo Esempio: wsl cargo run -- program.sgm -x
echo.
pause
goto end

:verify
echo.
echo === Verifica Installazione ===
echo.

echo Cercando riscv64-unknown-elf-gcc...
where riscv64-unknown-elf-gcc >nul 2>&1
if %ERRORLEVEL% EQU 0 (
    echo [OK] Trovato: riscv64-unknown-elf-gcc
    riscv64-unknown-elf-gcc --version | findstr "gcc"
) else (
    echo [X] Non trovato: riscv64-unknown-elf-gcc
)

echo.
echo Cercando riscv64-linux-gnu-gcc...
where riscv64-linux-gnu-gcc >nul 2>&1
if %ERRORLEVEL% EQU 0 (
    echo [OK] Trovato: riscv64-linux-gnu-gcc
    riscv64-linux-gnu-gcc --version | findstr "gcc"
) else (
    echo [X] Non trovato: riscv64-linux-gnu-gcc
)

echo.
echo Cercando qemu-riscv32...
where qemu-riscv32 >nul 2>&1
if %ERRORLEVEL% EQU 0 (
    echo [OK] Trovato: qemu-riscv32
    qemu-riscv32 --version | findstr "version"
) else (
    echo [X] Non trovato: qemu-riscv32
)

echo.
echo Se tutti i componenti sono presenti, puoi usare:
echo   cargo run -- program.sgm -x
echo.
pause
goto end

:end
