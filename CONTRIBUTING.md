# Contributing to rfetch 🦀

First off, thank you for considering contributing to rfetch! It's people like you that make this tool great.

## 🔰 How to Contribute

### 1. Reporting Bugs
- Check if the issue already exists in the [Issue Tracker](https://github.com/SAMIR897/rfetch/issues).
- Open a new issue with a clear title and description.
- Include your OS, Shell, and Terminal version.
- Paste the output of `rfetch --version` (once implemented).

### 2. Suggesting Features
- We love new ideas! Open an issue with the `enhancement` label.
- Explain *why* this feature would be useful.

### 3. Adding Support for a New Distro
If your distro is missing (unlikely, we have 261!), here's how to add it:
1. Fork the repo.
2. Add the ASCII art text file to `src/logos/<distro_name>.txt`.
3. Use `${c1}`, `${c2}` placeholders for colors.
4. Add the entry to `src/logo.rs` in the `build_logo_db` function.
5. Create a Pull Request!

## 🧪 Running Tests

Before submitting a PR, please run the tests to ensure everything is solid:

```bash
cargo test
```

## 📜 Code Style
- We use standard Rust formatting.
- Run `cargo fmt` before committing.
- Ensure no warnings with `cargo check`.

## ⚖️ License
By contributing, you agree that your contributions will be licensed under its MIT License.
