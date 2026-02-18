# rfetch 🦀
> A blazing fast, highly customizable system info tool written in Rust.

`rfetch` is a modern alternative to Neofetch, rewritten in Rust for speed. It maintains **100% features** of the original Neofetch, including all 261 distro logos and 28 info fields, but runs instantly.

![Screenshot](https://i.imgur.com/example_screenshot.png) *<!-- Add your own screenshot later -->*

## 🚀 Features
- **⚡ Blazing Fast**: Written in Rust for maximum performance (sub-5ms runtime).
- **🎨 261+ Logos**: Every single logo from Neofetch has been ported.
- **📦 Zero Dependencies**: Single binary distribution. No `bash`, no `python`, no `awk`.
- **🔧 Highly Configurable**: Custom logos, colors, and info layout.
- **🖥️ Cross-Platform**: Works on Linux, macOS, and BSD.

## 📦 Installation

### From Source
```bash
git clone https://github.com/SAMIR897/rfetch.git
cd rfetch
cargo install --path .
```

## 🛠️ Usage
Just run:
```bash
rfetch
```

## 📜 Full Info Fields
`rfetch` displays all 28 fields found in Neofetch:
- OS, Host, Kernel, Uptime
- Packages (brew, pacman, dpkg, rpm, snap, flatpak)
- Shell, Resolution, DE, WM
- Theme, Icons, Terminal, Terminal Font
- CPU, GPU, Memory, Disk, Battery
- Local IP, Users, Song (Spotify/Music)

## 🤝 Contributing
PRs are welcome! We are especially looking for:
- More distro logos (if we missed any!)
- Benchmarks vs fastfetch/neofetch
- Packaging for Arch (AUR), Debian (.deb), and Homebrew

## 📝 License
MIT
