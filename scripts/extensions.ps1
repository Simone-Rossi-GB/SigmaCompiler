# ========================================
# VS Code Extensions Installer for Windows
# ========================================

Write-Host "🚀 Installing VS Code Extensions for Rust Development" -ForegroundColor Cyan
Write-Host "======================================================" -ForegroundColor Cyan
Write-Host ""

# Array of all extensions
$extensions = @(
    # 🦀 Rust Essentials
    "rust-lang.rust-analyzer",
    "serayuzgur.crates",
    "tamasfe.even-better-toml",
    "usernamehvt.errorlens",
    "vadimcn.vscode-lldb",
    
    # 🎨 Themes & Icons
    "zhuangtongfa.material-theme",
    "PKief.material-icon-theme",
    "dracula-theme.theme-dracula",
    "enkia.tokyo-night",
    "Catppuccin.catppuccin-vsc",
    
    # 💻 AI & Productivity
    "GitHub.copilot",
    "GitHub.copilot-chat",
    "VisualStudioExptTeam.vscodeintellicode",
    
    # 📝 Writing & Auto-completion
    "formulahendry.auto-rename-tag",
    "formulahendry.auto-close-tag",
    "2gua.rainbow-brackets",
    "brunnerh.insert-unicode",
    "wware.snippet-creator",
    "rebornix.project-snippets",
    "ChakrounAnas.turbo-console-log",
    
    # ✏️ Advanced Editing
    "wmaurer.change-case",
    "Tyriar.sort-lines",
    "albymor.increment-selection",
    "BriteSnow.vscode-toggle-quotes",
    "letrieu.expand-region",
    "wwm.better-align",
    "stkb.rewrap",
    "shardulm94.trailing-spaces",
    
    # 🎯 Navigation & Selection
    "wmaurer.vscode-jumpy",
    "metaseed.metago",
    
    # 🌈 Visual & Highlighting
    "oderwat.indent-rainbow",
    "Gruntfuggly.todo-tree",
    "aaron-bond.better-comments",
    "vincaslt.highlight-matching-tag",
    "naumovs.color-highlight",
    "IBM.output-colorizer",
    
    # 🔍 Git & Version Control
    "eamodio.gitlens",
    "mhutchie.git-graph",
    "donjayamanne.githistory",
    "GitHub.vscode-pull-request-github",
    
    # 📚 Documentation
    "yzhang.markdown-all-in-one",
    "DavidAnson.vscode-markdownlint",
    
    # 🛠️ Utilities
    "ms-vscode-remote.remote-ssh",
    "ms-azuretools.vscode-docker",
    "rangav.vscode-thunder-client",
    "qwtel.sqlite-viewer",
    "ms-vscode.hexeditor",
    "streetsidesoftware.code-spell-checker",
    "johnpapa.vscode-peacock",
    
    # 🚀 Code Execution & Testing
    "formulahendry.code-runner",
    "hdevalke.rust-test-lens",
    
    # 📊 Quality & Analysis
    "ryanluker.vscode-coverage-gutters",
    "SonarSource.sonarlint-vscode",
    
    # 🎁 Bonus Quality of Life
    "adpyke.codesnap",
    "WakaTime.vscode-wakatime",
    "chrmarti.regex",
    "quicktype.quicktype",
    "christian-kohler.path-intellisense",
    
    # 🦀 Rust Specific
    "dunstontc.vscode-rust-syntax",
    "fill-labs.dependi",
    
    # 🖥️ Terminal
    "Tyriar.terminal-tabs",

    # 📸 Screenshots
    "adpyke.codesnap",
    
    # 📊 Productivity Tracking
    "WakaTime.vscode-wakatime",
    
    # 🔧 Development Tools
    "chrmarti.regex",
    "quicktype.quicktype",
    
    # 🎨 UI Enhancements
    "johnpapa.vscode-peacock",
    "vincaslt.highlight-matching-tag",
    "naumovs.color-highlight",
    
    # 📝 Advanced Editing
    "wwm.better-align",
    "wmaurer.vscode-jumpy",
    "metaseed.metago",
    "letrieu.expand-region",
    "BriteSnow.vscode-toggle-quotes",
    
    # 🔤 Text Manipulation
    "wmaurer.change-case",
    "Tyriar.sort-lines",
    "albymor.increment-selection",
    "stkb.rewrap",
    
    # 🌟 Symbols & Unicode
    "brunnerh.insert-unicode",
    
    # 🗂️ Project Management
    "alefragnani.Bookmarks",
    "alefragnani.project-manager",
    
    # 📦 Package Management
    "fill-labs.dependi",
    
    # 🎵 Extra Themes
    "rokoroku.vscode-theme-darcula",
    "sdras.night-owl",
    "wesbos.theme-cobalt2"
)

# Counter
$count = 0
$total = $extensions.Count

Write-Host "📦 Installing $total extensions..." -ForegroundColor Yellow
Write-Host ""

# Install each extension
foreach ($ext in $extensions) {
    $count++
    Write-Host "[$count/$total] Installing $ext..." -ForegroundColor White
    
    # Run code command
    $result = code --install-extension $ext --force 2>&1
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Successfully installed $ext" -ForegroundColor Green
    } else {
        Write-Host "❌ Failed to install $ext" -ForegroundColor Red
    }
    Write-Host ""
}

Write-Host ""
Write-Host "======================================================" -ForegroundColor Cyan
Write-Host "🎉 Installation complete! Installed $total extensions." -ForegroundColor Green
Write-Host "" -ForegroundColor Cyan
Write-Host "📝 Next steps:" -ForegroundColor Yellow
Write-Host "1. Restart VS Code" -ForegroundColor White
Write-Host "2. Choose your preferred theme (Ctrl+K Ctrl+T)" -ForegroundColor White
Write-Host "3. Install a font with ligatures (Fira Code recommended)" -ForegroundColor White
Write-Host "4. Copy settings.json to: $env:APPDATA\Code\User\settings.json" -ForegroundColor White
Write-Host "5. Copy keybindings.json to: $env:APPDATA\Code\User\keybindings.json" -ForegroundColor White
Write-Host "" -ForegroundColor Cyan
Write-Host "🔧 Verify with: code --list-extensions" -ForegroundColor Yellow
Write-Host "======================================================" -ForegroundColor Cyan

# Pause to see results
Write-Host ""
Write-Host "Press any key to exit..." -ForegroundColor Cyan
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")