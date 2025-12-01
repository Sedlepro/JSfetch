# JSfetch
Sed and Jess’s fetch

## Overview
JSfetch is a minimal neofetch-like system information tool written in Rust. It displays essential system details for Linux and Raspberry Pi environments.

## Features
JSfetch automatically detects:

- User
- OS / Distribution
- Kernel
- Active Shell
- Terminal
- CPU
- GPU (Raspberry Pi + PC, clean output)
- Memory (used / total)
- Uptime
- JSfetch ASCII Art

---

## Installation

### 1. Clone the repository
```bash
git clone https://github.com/<YOUR-NAME>/JSfetch
cd JSfetch
```

### 2. Build the project
```bash
cargo build --release
```

The binary will be located at:
```
target/release/jsfetch
```

### 3. (Optional) Install globally
```bash
sudo cp target/release/jsfetch /usr/local/bin/
```

Run with:
```bash
jsfetch
```

---

## Auto-run on Terminal Startup

### Bash
Edit:
```bash
nano ~/.bashrc
```
Add at the end:
```bash
jsfetch
```
Reload:
```bash
source ~/.bashrc
```

### Zsh
```bash
nano ~/.zshrc
```
Add:
```bash
jsfetch
```
Reload:
```bash
source ~/.zshrc
```

### Fish
```bash
nano ~/.config/fish/config.fish
```
Add:
```fish
jsfetch
```

### Recommended: Only run in interactive terminals
```bash
if [ -t 1 ]; then
    jsfetch
fi
```

---

## ASCII Preview
```
      ██╗███████╗███████╗███████╗████████╗██╗  ██╗
      ██║██╔════╝██╔════╝██╔════╝╚══██╔══╝██║  ██║
      ██║███████╗█████╗  ███████╗   ██║   ███████║
 ██   ██║╚════██║██╔══╝  ╚════██║   ██║   ██╔══██║
 ╚█████╔╝███████║███████╗███████║   ██║   ██║  ██║
  ╚════╝ ╚══════╝╚══════╝╚══════╝   ╚═╝   ╚═╝  ╚═╝
                    JSfetch
```
---

## License
This project is released under the MIT License.

## Contributors
- Sedrick
- Jessica
