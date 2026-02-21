<div align="center">
  <h1>rfetch 🦀</h1>
  <p><strong>A blazing fast, cross-platform system information tool written in Rust.</strong></p>
  <p>The modern, zero-dependency successor to Neofetch.</p>
  
  <a href="https://github.com/SAMIR897/rfetch/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/SAMIR897/rfetch/ci.yml?style=flat-square&logo=github&label=CI" alt="Build Status">
  </a>
  <a href="https://github.com/SAMIR897/rfetch/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/SAMIR897/rfetch?style=flat-square&color=blue" alt="License">
  </a>
  <a href="https://github.com/SAMIR897/rfetch/releases">
    <img src="https://img.shields.io/github/v/release/SAMIR897/rfetch?style=flat-square&color=orange" alt="Release">
  </a>
  <a href="https://github.com/SAMIR897/rfetch/releases">
    <img src="https://img.shields.io/github/downloads/SAMIR897/rfetch/total?style=flat-square&color=green&label=downloads" alt="Downloads">
  </a>
  <br><br>
  <img src="https://raw.githubusercontent.com/SAMIR897/rfetch/main/assets/screenshot.png" alt="rfetch screenshot" width="800">
</div>

---

## 🚀 Why rfetch?

[Neofetch](https://github.com/dylanaraps/neofetch) is no longer maintained. [Fastfetch](https://github.com/fastfetch-cli/fastfetch) is great, but heavier.
**rfetch** is the spiritual successor to Neofetch — written in pure Rust for instant startup times, zero external dependencies, and a single static binary.

| Feature | Neofetch | Fastfetch | **rfetch** 🦀 |
| :--- | :---: | :---: | :---: |
| **Language** | Bash | C | **Rust** |
| **Speed** | ~200ms | ~2ms | **~1ms** |
| **Binary Size** | 100KB (+50 deps) | ~2MB (+libs) | **<1MB (Static)** |
| **Distro Logos** | ~260 | ~500 | **263+ (Neofetch parity)** |
| **Dependencies** | Bash, awk, etc. | CMake, libs | **Zero** |
| **Config** | config.conf | JSON | **Zero-config defaults** |

---

## ✨ Features

### System Information
rfetch displays **28+ system info fields** out of the box:

| Category | Fields |
|---|---|
| **System** | OS, Host, Kernel, Uptime |
| **Software** | Packages, Shell, DE, WM, WM Theme, Theme, Icons |
| **Display** | Resolution, Terminal, Terminal Font |
| **Hardware** | CPU, GPU, GPU Driver, Memory, Disk, Battery |
| **Network** | Local IP, Global IP, Network Speed |
| **Users** | Logged-in users |
| **Visual** | ANSI color bar |

### Highlights

- **⚡ Instant Startup** — Heavily optimized Rust code with ~1ms cold start
- **🎨 263+ Distro Logos** — Every logo from Neofetch, plus version-specific variants (e.g., Windows 7/10/11)
- **� Live Mode** — Real-time system monitoring with in-place terminal updates (`--live`)
- **📦 Single Static Binary** — No dependencies, no installation mess. Just download and run
- **🖥️ Cross-Platform** — First-class support for **Linux**, **macOS**, and **Windows**
- **🪶 Lightweight** — Minimal resource usage with zero background processes

---

## 📦 Installation

### Quick Install (macOS / Linux)

```bash
curl -sSf https://raw.githubusercontent.com/SAMIR897/rfetch/main/install.sh | sh
```

This will automatically detect your OS and architecture, download the latest release, and install it to `/usr/local/bin`.

### Windows

1. Go to the [Releases Page](https://github.com/SAMIR897/rfetch/releases)
2. Download `rfetch-windows-amd64.zip`
3. Extract and run `rfetch.exe` in cmd or PowerShell
4. *(Optional)* Add the folder to your `PATH` to run `rfetch` from anywhere

### Build from Source

Requires [Rust](https://rustup.rs/) (1.85+):

```bash
git clone https://github.com/SAMIR897/rfetch.git
cd rfetch
cargo install --path .
```

---

## 🛠️ Usage

```bash
# Standard fetch
rfetch

# Live mode — auto-refreshing system stats
rfetch --live
```

### Flags

| Flag | Description |
|---|---|
| `--live` | Enable live mode with in-place terminal updates |
| `--help` | Show help information |

### Planned Flags

| Flag | Description |
|---|---|
| `--distro <name>` | Force a specific distro logo |
| `--small` | Use smaller logo variant |
| `--no-color` | Disable colored output |
| `--block` | High-fidelity block-art logos (coming soon) |

---
## 🏗️ Project Structure

```
rfetch/
├── src/
│   ├── main.rs          # Entry point, rendering engine
│   ├── logo.rs          # Logo database and color rendering
│   ├── logos/           # 263+ ASCII art logo files
│   └── info/            # System info modules
├── install.sh           # One-line installer
├── Cargo.toml           # Rust dependencies
├── LICENSE              # MIT License
└── CONTRIBUTING.md      # Contribution guide
```

## 📦 Changelog

### [v1.0.2](https://github.com/SAMIR897/rfetch/releases/tag/v1.0.2) — *Windows Logo Update & Live Mode*
- **Windows Logos 🪟** — Version-specific ASCII logos for Windows 7, 8/10, and 11
- **Live Mode 📺** — `--live` flag for real-time, in-place terminal updates
- **Net Speed** — Added upload/download network speed monitoring
- **Fix** — Layout alignment corrected by stripping ANSI codes before padding

### [v1.0.1](https://github.com/SAMIR897/rfetch/releases/tag/v1.0.1) — *The Cross-Platform Update*
- **Windows Support 🪟** — Runs natively on Windows with `wmic`, `powershell`, and `cmd` detection; supports `winget`, `choco`, and `scoop` package counts
- **Repo Optimization 📉** — Removed build artifacts from history; size dropped from 31MB → 272KB, clone time < 1s
- **Fix** — GPU detection fallback on macOS

### [v1.0.0](https://github.com/SAMIR897/rfetch/releases/tag/v1.0.0) — *Initial Release*
- **Core**: 261+ Neofetch-parity distro logos, 28+ system info fields (OS, CPU, GPU, Memory, Disk, Battery, Network)
- **Distribution**: Single static binary, zero dependencies, one-line installer for Linux & macOS

---

## 🤝 Contributing

We'd love your help making rfetch the #1 fetch tool! Whether it's adding a missing distro logo, fixing a bug, or improving performance — all contributions are welcome.

See **[CONTRIBUTING.md](CONTRIBUTING.md)** for guidelines on getting started.

### Quick Start for Contributors

```bash
git clone https://github.com/SAMIR897/rfetch.git
cd rfetch
cargo test     # Run test suite
cargo run      # Test locally
cargo fmt      # Format code
```

---

## � License

MIT © [SAMIR897](https://github.com/SAMIR897)

---

<div align="center">
  <p>Built with 🦀 and 💜</p>
  <p>If you find rfetch useful, please consider giving it a ⭐ on GitHub!</p>
</div>
