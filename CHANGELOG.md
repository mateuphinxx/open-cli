# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- compiler management with automatic download and caching
- Argon2 hash verification for compiler integrity
- Real-time progress tracking for downloads and builds
- Build timing display
- Comprehensive logging system
- Security scanning with CodeQL and cargo-audit
- Cross-platform support (Windows, Linux, macOS)

### Changed
- Improved error handling and user feedback
- Optimized binary size with release profiles
- Enhanced configuration management

### Fixed
- Removed deprecated include.p fallback logic
- Improved cross-platform path handling

### Security
- Added supply chain security checks
- Implemented dependency vulnerability scanning
- Added license compliance checking

## [0.1.0] - 2025-01-XX

### Added
- Initial release
- Basic CLI interface for open.mp server management
- Project building functionality
- Compiler installation and management
- Configuration setup
- Server execution capabilities
