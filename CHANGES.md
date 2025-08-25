# Changes

## v0.2.0 (2025-08-25)

### Added
- **fincore**: Added `-g, --graph` option to print a visual graph of each file's cached page distribution
- **Documentation**: Added comprehensive installation instructions in README.md with pre-built binary download and installation steps
- **Development**: Added cargo-husky for automated pre-commit hooks (test, fmt, check, clippy)

### Changed
- **Dependencies**: Added `termsize` dependency (v0.1.9) for terminal size detection in graph display
- **README**: Updated with detailed installation section including GitHub Releases download instructions
- **Version**: Bumped version to 0.2.0

### Fixed
- **Build**: Committed Cargo.lock file for reproducible builds
- **Code Quality**: Applied cargo fmt formatting across codebase
- **Permissions**: Fixed file permissions issues
