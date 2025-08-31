# OpenCLI
CLI tool for open.mp server management and Pawn project building.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

*Inspired by [sampctl](https://github.com/Southclaws/sampctl)*

## Features

- **Smart Compiler Management** - Automatic download and caching
- **Security First** - Argon2 hash verification for compiler integrity  
- **Progress Tracking** - Real-time download and build progress
- **Build Timing** - See how fast your projects compile
- **Comprehensive Logging** - Full activity logs for debugging

## Installation

```bash
git clone https://github.com/mateuphinx/open-cli
cd open-cli
cargo build --release
```

## Quick Start

```bash
# Setup your project
opencli setup

# Install Pawn compiler
opencli install compiler

# Build your project
opencli build

# Run your server
opencli run
```

## Usage

### Project Building
```bash
# Build with default settings
opencli build

# Build with verbose output
opencli build --verbose

# Force compiler re-download
opencli build --force-download
```

### Compiler Management
```bash
# Install default compiler (v3.10.11)
opencli install compiler

# Install specific version
opencli install compiler --version v3.10.12

# Force reinstall
opencli install compiler --force
```

### Server Management
```bash
# Run server (auto-detect omp-server)
opencli run

# Run with custom path
opencli run --server-path "path/to/omp-server.exe"
```

## Configuration

Run `opencli setup` to create `opencli.toml`:

```toml
[build]
entry_file = "gamemodes/gamemode.pwn"
output_file = "gamemodes/gamemode.amx"
compiler_version = "v3.10.11"

[build.includes]
paths = ["qawno/include"]

[build.args]
args = ["-d3", "-;+", "-(+", "-\\+", "-Z+", "-O2"]
```

## File Locations

All OpenCLI files are stored in your system's config directory:

- **Windows**: `%APPDATA%\opencli\`
- **Linux/macOS**: `~/.config/opencli/`

## Requirements

- Rust 1.70+
- Internet connection (first time setup)
- open.mp server binary (for running servers)

## License

MIT License - Copyright (c) 2025 Mattias Theodore "mateuphinx" Bartholomew

See [LICENSE](LICENSE) for details.