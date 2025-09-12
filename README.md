# OpenCLI
CLI tool for open.mp server management and Pawn project building with package management system.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

*Inspired by [sampctl](https://github.com/Southclaws/sampctl)*

## Features

- **Package Management** - Install libraries like sscanf, mysql from GitHub
- **Compiler Management** - Automatic compiler download and caching
- **Security First** - Integrity verification with Argon2 hash  
- **Progress Tracking** - Real-time download and build monitoring
- **Build Performance** - See how fast your projects compile
- **Comprehensive Logging** - Complete activity logs for debugging

## Installation

```bash
git clone https://github.com/mateuphinxx/open-cli
cd open-cli
cargo build --release
```

## Quick Start

```bash
# Setup new project
opencli setup

# Install Pawn compiler
opencli install compiler

# Install packages (example: sscanf)
opencli package install Y-Less/sscanf

# Build project
opencli build

# Run server
opencli run
```

## Package Management

### Install Packages
```bash
# Install all packages from opencli.toml
opencli package install

# Install specific package
opencli package install Y-Less/sscanf
opencli package install "Y-Less/sscanf=2.13.8"
opencli package install Y-Less/sscanf --target components

# Install with versioning
opencli package install "Y-Less/sscanf=^2.13.7"
```

### Package Management
```bash
# List installed packages
opencli package list

# Remove package
opencli package remove Y-Less/sscanf

# Update packages
opencli package update Y-Less/sscanf
opencli package update --all
```

### Version Constraints
```toml
[packages]
"owner/repo" = "^x.y.z"              # Compatible updates (>=x.y.z, <(x+1).0.0)
"owner/repo" = "~x.y.z"              # Patch updates only (>=x.y.z, <x.(y+1).0)
"owner/repo" = ">=x.y.z, <a.b.c"     # Range constraint
"owner/repo" = "latest"              # Always use latest version
"owner/repo" = "x.y.z"               # Exact version
```

### File Placement

OpenCLI automatically places files according to their type:

- **Include Files (.inc)** → Goes to folders defined in `[build.includes]`
- **Components** → `.dll`/`.so` files go to `components/` folder
- **Plugins** → Legacy files go to `plugins/` folder
- **Root Libraries** → Files containing "amx", "lib", or "log-core" (like `amxsscanf.dll`, `log-core.dll`) → root directory

**Archive Detection:**
- If archive contains `components/` folder → files are taken from there for components target
- If archive contains `plugins/` folder → files are taken from there for plugins target

## Configuration

Create `opencli.toml` with `opencli setup`:

```toml
[build]
entry_file = "gamemodes/gamemode.pwn"
output_file = "gamemodes/gamemode.amx"
compiler_version = "v3.10.11"

[build.includes]
paths = [
    "include"           # Package includes will go here
]

[build.args]
args = ["-d3", "-;+", "-(+", "-\\+", "-Z+", "-O2"]
```

## Compiler Management

```bash
# Install default compiler (v3.10.11)
opencli install compiler

# Install specific version
opencli install compiler --version v3.10.11

# Force reinstall
opencli install compiler --force
```

## Project Building

```bash
# Build with default settings
opencli build

# Build with verbose output
opencli build --verbose

# Force compiler re-download
opencli build --force-download
```

## Server Management

```bash
# Run server (auto-detect omp-server)
opencli run

# Run with custom path
opencli run --server-path "path/to/omp-server.exe"
```

## File Locations

All OpenCLI files are stored in:

- **Windows**: `%APPDATA%\opencli\`
- **Linux/macOS**: `~/.config/opencli/`

## Example Workflow

```bash
# 1. Setup new project
opencli setup

# 2. Install dependencies
opencli package install Y-Less/sscanf
opencli package install pBlueG/SA-MP-MySQL --target plugins

# 3. Build project (includes automatically available)
opencli build

# 4. Run server (binaries already in correct places)
opencli run
```

## Package Examples

```toml
[packages]
# Components plugins
"Y-Less/sscanf" = { version = "^2.13.8", target = "components" }

# Legacy Plugins
"Southclaws/pawn-requests" = { version = "latest", target = "plugins" }
```

## Requirements

- Rust 1.75+
- Internet connection (first time setup)
- open.mp server binary (for running servers)

## License

MIT License - Copyright (c) 2025 Mattias Theodore "mateuphinxx" Bartholomew

See [LICENSE](LICENSE) for details.