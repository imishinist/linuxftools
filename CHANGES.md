# Changes

## 2025-08-24

### Added
- **fincore**: Memory-mapped file cache inspection tool
  - Shows which parts of files are cached in memory
  - `-s, --summarize`: Print summary report for multiple files
  - `-p, --pages`: Print cached pages information
  - `-o, --only-cached`: Show only files that are in cache

- **fallocate**: File space pre-allocation utility
  - Pre-allocates disk space for files: `fallocate <file> <length>`

- **fadvise**: File access pattern advisor
  - Provides kernel hints for file access patterns: `fadvise <filename> <mode>`
  - `-o, --offset`: Specify offset position
  - `-l, --length`: Specify length

## 2025-08-23

### Initial Release
- Project structure and basic command-line interfaces established
- Cross-platform support for Linux and macOS
