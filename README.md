# rfetch 🦀

<div align="center">
  <h1>rfetch</h1>
  <p><strong>A blazing fast, highly customizable system info tool written in Rust.</strong></p>
  
  <a href="https://github.com/SAMIR897/rfetch/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/SAMIR897/rfetch/ci.yml?style=flat-square&logo=github" alt="Build Status">
  </a>
  <a href="https://github.com/SAMIR897/rfetch/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/SAMIR897/rfetch?style=flat-square&color=blue" alt="License">
  </a>
  <a href="https://github.com/SAMIR897/rfetch/releases">
     <img src="https://img.shields.io/github/v/release/SAMIR897/rfetch?style=flat-square&color=orange" alt="Release">
  </a>
  <br>
  <br>
  <!-- REPLACE THIS LINK WITH YOUR OWN SCREENSHOT LATER -->
  <img src="https://raw.githubusercontent.com/SAMIR897/rfetch/main/assets/screenshot.png" alt="rfetch screenshot" width="800">
</div>

## 🚀 Why rfetch?

Neofetch is dead. Fastfetch is great, but heavier.
**rfetch** is the spiritual successor to Neofetch — written in Rust for instant startup times while maintaining the classic aesthetic.

| Feature | Neofetch | Fastfetch | rfetch 🦀 |
| :--- | :---: | :---: | :---: |
| **Language** | Bash | C | **Rust** |
| **Speed** | Slow (~200ms) | Fast (~2ms) | **Blazing (~1ms)** |
| **Size** | 100KB (+50 deps) | ~2MB (+libs) | **<1MB (Static)** |
| **Logos** | ~260 | ~500 | **261 (100% Neofetch parity)** |
| **Dependencies** | Bash, awk, etc. | Cmake, libs | **Zero (Static Binary)** |
| **Config** | Confusing config.conf | JSON | **Zero-config defaults** |

## ✨ Features

- **⚡ Instant Startup**: heavily optimized Rust code.
- **🎨 261+ Distro Logos**: Every single logo from Neofetch ported and embedded.
- **🔧 28+ Info Fields**: Hardware, Software, Network, and more.
- **📦 Single Binary**: No installation mess. Just download and run.
- **🖥️ Cross-Platform**: First-class support for **Linux**, **macOS**, and our old classmate **Windows**.

## 📦 Installation

### 🪟 Windows (Easy)
1. Go to the [Releases Page](https://github.com/SAMIR897/rfetch/releases).
2. Download `rfetch-windows-amd64.zip`.
3. Extract the zip file.
4. Run `rfetch.exe` in cmd or PowerShell.
*(Optional: Add the folder to your PATH environment variable to run it from anywhere)*

### 🍎 macOS / � Linux
**The Easy Way (via Install Script):**
```bash
curl -sSf https://raw.githubusercontent.com/SAMIR897/rfetch/main/install.sh | sh
```

**The Hard Way (Compile from Source):**
Requires Rust to be installed.
```bash
git clone https://github.com/SAMIR897/rfetch.git
cd rfetch
cargo install --path .
```

## 🛠️ Usage

Simply run:
```bash
rfetch
```

### Flags (Planned)
- `--distro <name>`: Force a specific logo (e.g. `rfetch --distro ubuntu`)
- `--small`: Use small logo variant
- `--no-color`: Disable colors

## 🤝 Contributing

We want your help to make this the #1 fetch tool!
See [CONTRIBUTING.md](CONTRIBUTING.md) for how to get started.

## 📝 License

MIT © [SAMIR897](https://github.com/SAMIR897)
