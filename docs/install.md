---
icon: lucide/file-terminal
---

# Installation

## Pre-compiled binaries

Download from [:simple-github: GitHub Releases](https://github.com/aNNiMON/effy/releases)

## Packages


=== ":simple-archlinux: effy"

    ```bash
    paru -S effy
    ```
    ```bash
    yay -S effy
    ```

=== ":simple-archlinux: effy-bin"

    ```bash
    paru -S effy-bin
    ```
    ```bash
    yay -S effy-bin
    ```

## Install using `cargo`

```bash
cargo install --locked effy
```

## Compile from source

1. Install [:simple-rust: Rustup](https://rustup.rs/)
2. Clone the repository:
   ```bash
   git clone https://github.com/aNNiMON/effy
   cd effy
   ```
3. Build:
   ```bash
   cargo build --release
   ```
4. Navigate to the `./target/release` directory and look for the `effy` binary


