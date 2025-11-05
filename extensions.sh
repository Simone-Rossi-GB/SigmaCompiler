#!/bin/bash

echo "🚀 Installing VS Code Extensions for Rust Development & Enhanced Writing Experience"
echo "==============================================================================="

# Array of all extensions
extensions=(
    # 🦀 Rust Essentials
    "rust-lang.rust-analyzer"
    "serayuzgur.crates"
    "tamasfe.even-better-toml"
    "usernamehvt.errorlens"
    "vadimcn.vscode-lldb"
    
    # 🎨 Themes & Icons (scegli uno, io installo tutti e decidi tu)
    "zhuangtongfa.material-theme"          # One Dark Pro
    "PKief.material-icon-theme"            # Material Icons (MIGLIORE)
    "dracula-theme.theme-dracula"          # Dracula
    "enkia.tokyo-night"                    # Tokyo Night
    "Catppuccin.catppuccin-vsc"           # Catppuccin
    
    # 💻 AI & Productivity
    "GitHub.copilot"                       # GitHub Copilot (a pagamento)
    "GitHub.copilot-chat"                  # Copilot Chat
    "VisualStudioExptTeam.vscodeintellicode" # IntelliCode (gratis)
    
    # 📝 Writing & Auto-completion
    "formulahendry.auto-rename-tag"        # Auto Rename Tag
    "formulahendry.auto-close-tag"         # Auto Close Tag
    "2gua.rainbow-brackets"                # Rainbow Brackets
    "brunnerh.insert-unicode"              # Insert Unicode symbols
    "wware.snippet-creator"                # Snippet Creator
    "rebornix.project-snippets"            # Project Snippets
    "ChakrounAnas.turbo-console-log"       # Turbo Console Log
    
    # ✏️ Advanced Editing
    "wmaurer.change-case"                  # Change Case
    "Tyriar.sort-lines"                    # Sort Lines
    "albymor.increment-selection"          # Increment Selection
    "BriteSnow.vscode-toggle-quotes"       # Toggle Quotes
    "letrieu.expand-region"                # Expand Region
    "wwm.better-align"                     # Better Align (OTTIMO per Rust)
    "stkb.rewrap"                          # Rewrap comments
    "shardulm94.trailing-spaces"           # Trailing Spaces
    
    # 🎯 Navigation & Selection
    "wmaurer.vscode-jumpy"                 # Jumpy
    "metaseed.metago"                      # MetaGo
    
    # 🌈 Visual & Highlighting
    "oderwat.indent-rainbow"               # Indent Rainbow
    "Gruntfuggly.todo-tree"               # TODO Tree (MEGLIO di TODO Highlight)
    "aaron-bond.better-comments"           # Better Comments
    "vincaslt.highlight-matching-tag"      # Highlight Matching Tag
    "naumovs.color-highlight"              # Color Highlight
    "IBM.output-colorizer"                 # Output Colorizer
    
    # 🔍 Git & Version Control
    "eamodio.gitlens"                      # GitLens (ESSENZIALE)
    "mhutchie.git-graph"                   # Git Graph
    "donjayamanne.githistory"              # Git History
    "GitHub.vscode-pull-request-github"    # GitHub Pull Requests
    
    # 📚 Documentation
    "yzhang.markdown-all-in-one"          # Markdown All in One (MIGLIORE)
    "DavidAnson.vscode-markdownlint"      # markdownlint
    
    # 🛠️ Utilities
    "ms-vscode-remote.remote-ssh"          # Remote SSH
    "ms-azuretools.vscode-docker"          # Docker
    "rangav.vscode-thunder-client"         # Thunder Client (MIGLIORE di REST Client)
    "qwtel.sqlite-viewer"                  # SQLite Viewer
    "ms-vscode.hexeditor"                  # Hex Editor
    "streetsidesoftware.code-spell-checker" # Code Spell Checker
    "johnpapa.vscode-peacock"              # Peacock
    
    # 🚀 Code Execution & Testing
    "formulahendry.code-runner"            # Code Runner
    "hdevalke.rust-test-lens"             # Rust Test Lens
    
    # 📊 Quality & Analysis
    "ryanluker.vscode-coverage-gutters"    # Coverage Gutters
    "SonarSource.sonarlint-vscode"         # SonarLint
    
    # 🎁 Bonus Quality of Life
    "adpyke.codesnap"                      # CodeSnap (MIGLIORE di Polacode)
    "WakaTime.vscode-wakatime"            # WakaTime
    "chrmarti.regex"                       # Regex Previewer
    "quicktype.quicktype"                  # Paste JSON as Code
    "christian-kohler.path-intellisense"   # Path Intellisense
    
    # 🦀 Rust Specific
    "dunstontc.vscode-rust-syntax"         # Rust Syntax
    "fill-labs.dependi"                    # Dependi
    
    # 🖥️ Terminal
    "Tyriar.terminal-tabs"                 # Terminal Tabs

    # 📸 Screenshots
    "adpyke.codesnap"                      # CodeSnap - bellissimi screenshot
    
    # 📊 Productivity Tracking
    "WakaTime.vscode-wakatime"            # WakaTime - traccia tempo coding
    
    # 🔧 Development Tools
    "chrmarti.regex"                       # Regex Previewer
    "quicktype.quicktype"                  # Paste JSON as Code
    
    # 🎨 UI Enhancements
    "johnpapa.vscode-peacock"              # Peacock - colora workspace
    "vincaslt.highlight-matching-tag"      # Highlight Matching Tag
    "naumovs.color-highlight"              # Color Highlight
    
    # 📝 Advanced Editing
    "wwm.better-align"                     # Better Align
    "wmaurer.vscode-jumpy"                 # Jumpy navigation
    "metaseed.metago"                      # MetaGo navigation
    "letrieu.expand-region"                # Expand Region
    "BriteSnow.vscode-toggle-quotes"       # Toggle Quotes
    
    # 🔤 Text Manipulation
    "wmaurer.change-case"                  # Change Case
    "Tyriar.sort-lines"                    # Sort Lines
    "albymor.increment-selection"          # Increment Selection
    "stkb.rewrap"                          # Rewrap
    
    # 🌟 Symbols & Unicode
    "brunnerh.insert-unicode"              # Insert Unicode
    
    # 🗂️ Project Management
    "alefragnani.Bookmarks"                # Bookmarks
    "alefragnani.project-manager"          # Project Manager
    
    # 📦 Package Management
    "fill-labs.dependi"                    # Dependi - dependency manager
    
    # 🎵 Fun (optional)
    "rokoroku.vscode-theme-darcula"        # Darcula theme from IntelliJ
    "sdras.night-owl"                      # Night Owl theme
    "wesbos.theme-cobalt2"                 # Cobalt2 theme
)

# Counter for installed extensions
count=0
total=${#extensions[@]}

echo ""
echo "📦 Installing $total extensions..."
echo ""

# Install each extension
for ext in "${extensions[@]}"; do
    ((count++))
    echo "[$count/$total] Installing $ext..."
    code --install-extension "$ext" --force
    
    # Check if installation was successful
    if [ $? -eq 0 ]; then
        echo "✅ Successfully installed $ext"
    else
        echo "❌ Failed to install $ext"
    fi
    echo ""
done

echo ""
echo "=============================================================================="
echo "🎉 Installation complete! Installed $total extensions."
echo ""
echo "📝 Next steps:"
echo "1. Restart VS Code"
echo "2. Choose your preferred theme (Ctrl+K Ctrl+T)"
echo "3. Configure settings with the recommended settings.json"
echo "4. Install a font with ligatures (Fira Code, JetBrains Mono, Cascadia Code)"
echo ""
echo "🔧 Optional: Run 'code --list-extensions' to verify all extensions"
echo "=============================================================================="