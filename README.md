# linuxftools

A Rust implementation of Linux filesystem-related tools.

## Overview

This project provides a collection of filesystem utilities implemented in Rust, offering functionality for memory-mapped file operations, file allocation, cache status inspection, and more.

## Available Tools

### fallocate
Pre-allocates space for files.

```bash
fallocate <file> <length>
```

- `file`: Target file path
- `length`: Size to allocate (in bytes)

### fincore
Shows which parts of files are cached in memory.

```bash
fincore [OPTIONS] <files>...
```

Options:
- `-s, --summarize`: Print a summary report when comparing multiple files
- `-p, --pages`: Print pages that are cached
- `-o, --only-cached`: Only print stats for files that are actually in cache

### fadvise
Provides file access pattern advice to the kernel.

```bash
fadvise [OPTIONS] <filename> <mode>
```

Options:
- `-o, --offset <offset>`: Offset position
- `-l, --length <length>`: Length

## Building

```bash
cargo build --release
```

## Installation

```bash
cargo install --path .
```

## Usage Examples

### File Pre-allocation
```bash
fallocate test.dat 1048576  # Pre-allocate 1MB file
```

### Check File Cache Status
```bash
fincore /path/to/file          # Check cache status of a file
fincore -s /path/to/files/*    # Summary view of multiple files
fincore -p /path/to/file       # Show cached pages
```

### File Access Advice
```bash
fadvise /path/to/file sequential    # Advise sequential access
fadvise /path/to/file random        # Advise random access
```

## Dependencies

- `clap`: Command-line argument parsing
- `libc`: System call interface

## Supported Platforms

- Linux
- macOS (with some limitations)

## License

This project is licensed under the MIT License.

## Contributing

Bug reports and feature requests are welcome via GitHub Issues. Pull requests are also appreciated.
