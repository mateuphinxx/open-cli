# OpenCLI

Command-line interface (CLI) tool untuk [Open.MP](https://open.mp/) Manajemen Server dan Pawn project building dengan sistem package management.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Test](https://github.com/mxp96/open-cli/actions/workflows/test.yml/badge.svg)](https://github.com/mxp96/open-cli/actions/workflows/test.yml)

## Dokumentasi

- [Wiki Home](https://github.com/mxp96/open-cli/wiki)
- [Package Management](https://github.com/mxp96/open-cli/wiki)
- [Compiler Options](https://github.com/mxp96/open-cli/wiki/Compiler-Options)
- [Docker Guide](docs/DOCKER.md)
- [Contributing](docs/CONTRIBUTING.md)

## Fitur-Fitur

- **Package Management** - Installasi pustaka seperti sscanf, mysql dari GitHub
- **Compiler Management** - Compiler otomatis download dan pengelola penyimpanan sementara
- **Security First** - Verifikasi integritas dengan hash Argon2
- **Progress Tracking** - Real-time download dan build monitoring
- **Build Performance** - Lihat seberapa cepat proyek Kamu dikompilasi
- **Comprehensive Logging** - Log aktivitas lengkap untuk debugging

## Installasi

### dari Release

Download binary terbaru untuk platform Anda dari [Releases](https://github.com/mxp96/open-cli/releases).

**Linux/macOS:**
```bash
tar -xzf opencli-*.tar.gz
sudo mv opencli /usr/local/bin/
opencli --version
```

**Windows:**
Extract the ZIP and add to PATH.

### From Source

```bash
git clone https://github.com/mxp96/open-cli
cd open-cli
cargo build --release
```

Binary will be in `target/release/opencli`.

### Using Docker

```bash
docker pull ghcr.io/mxp96/open-cli:latest
docker run --rm -v $(pwd):/workspace ghcr.io/mxp96/open-cli:latest --help
```

## Quick Start

```bash
# Setup new project
opencli setup

# Install Pawn compiler
opencli install compiler

# Install packages
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

# With version constraints
opencli package install "Y-Less/sscanf=^2.13.7"
```

### Manage Packages

```bash
# List installed packages
opencli package list

# Remove package
opencli package remove Y-Less/sscanf

# Update packages
opencli package update Y-Less/sscanf
opencli package update --all

# Check integrity
opencli package check
```

### Version Constraints

```toml
[packages]
"owner/repo" = "^x.y.z"              # Compatible updates
"owner/repo" = "~x.y.z"              # Patch updates only
"owner/repo" = ">=x.y.z, <a.b.c"     # Range constraint
"owner/repo" = "latest"              # Always latest
"owner/repo" = "x.y.z"               # Exact version
```

## Configuration

Create `opencli.toml` with `opencli setup`:

```toml
[build]
entry_file = "gamemodes/gamemode.pwn"
output_file = "gamemodes/gamemode.amx"
compiler_version = "v3.10.11"

[build.includes]
paths = ["include"]

[build.args]
args = ["-d3", "-;+", "-(+", "-\\+", "-Z+"]

[packages]
"Y-Less/sscanf" = { version = "^2.13.8", target = "components" }
```

## Building

```bash
# Default build
opencli build

# Verbose output
opencli build --verbose

# Force compiler re-download
opencli build --force-download

# Update compiler config
opencli build --update-config
```

## Development

```bash
# Format code
cargo fmt --all
make docker-format  # Using Docker

# Run linter
cargo clippy --all-targets --all-features

# Run tests
cargo test --release

# Docker development
docker compose up dev
```

See [CONTRIBUTING.md](docs/CONTRIBUTING.md) for more details.

## Requirements

- Rust 1.89.0+ (for building from source)
- Internet connection (first time setup)
- open.mp server binary (for running servers)

## Contributors

Thanks to all contributors who made this project possible:

[![Contributors](https://contrib.rocks/image?repo=mxp96/open-cli)](https://github.com/mxp96/open-cli/graphs/contributors)

<!-- CONTRIBUTORS-LIST:START -->
Made with [contrib.rocks](https://contrib.rocks).
<!-- CONTRIBUTORS-LIST:END -->

## License

MIT License - Copyright (c) 2025 Matthias Theodore "mxp96" Bartholomew.
See [LICENSE](LICENSE) for details.

> Inspired by [sampctl](https://github.com/Southclaws/sampctl)
> Indonesian/ID mantainer: klantle. 
