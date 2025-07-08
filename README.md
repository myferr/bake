<p align="center">
<img width="212" height="212" alt="Image" src="https://github.com/user-attachments/assets/7274288a-11c6-4c01-8fd0-960a7931cdb4" />
</p>

<h1 align="center">bake</h1>

<p align="center">
<img src="https://img.shields.io/github/license/myferr/bake" alt="License">
<img src="https://img.shields.io/crates/v/bake-tool" alt="Crates.io">
<img src="https://img.shields.io/badge/Made%20with-Rust-orange?logo=rust&amp;logoColor=white" alt="Rust">
<img src="https://tokei.rs/b1/github/myferr/bake" alt="Lines of Code">
<img src="https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=flat" alt="Contributions welcome">
</p>

<p align="center">
CI
<br />
<img src="https://img.shields.io/github/actions/workflow/status/myferr/bake/crates.yml?label=crates" alt="Build">
<img src="https://img.shields.io/github/actions/workflow/status/myferr/bake/releases.yml?label=releases" alt="Build">
</p>

---

`bake` is a minimal and fast alternative to Make, written in Rust made for developers to improve efficency. `bake` uses it's own "Bakefile" system with a YAML-like syntax, but also supports simple Makefiles.

# Installation

## Remote ([install.c](https://raw.githubusercontent.com/myferr/bake/main/install.c))
**Linux/macOS:**

```bash
curl -sL https://raw.githubusercontent.com/myferr/bake/main/install.c | cc -xc -o install - && sudo ./install
```

**Windows (PowerShell):**

```bash
Invoke-WebRequest -Uri https://raw.githubusercontent.com/myferr/bake/main/install.c -OutFile install.c; cl install.c; .\install.exe
```

## Crates

**Cross-platform (Cargo):**
```bash
cargo install bake-tool
```
