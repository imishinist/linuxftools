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
- `-s, --summarize`: When comparing multiple files, print a summary report
- `-p, --pages`: Print pages that are cached
- `-o, --only-cached`: Only print stats for files that are actually in cache
- `-g, --graph`: Print a visual graph of each file's cached page distribution
- `-S, --min-size <N>`: Require that each files size be larger than N bytes
- `-C, --min-cached-size <N>`: Require that each files cached size be larger than N bytes
- `-P, --min-perc-cached <N>`: Require percentage of a file that must be cached
- `-L, --vertical`: Print the output of this script vertically
- `-v, --verbose`: Enable verbose output

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

### From GitHub Releases

Download the latest pre-built binary from the [Releases page](https://github.com/imishinist/linuxftools/releases):

```bash
# Download the latest release
curl -L -o linuxftools-Linux-x86_64-musl.tar.gz https://github.com/imishinist/linuxftools/releases/latest/download/linuxftools-Linux-x86_64-musl.tar.gz

# Extract the archive
tar -xzf linuxftools-Linux-x86_64-musl.tar.gz

# Move binaries to your PATH (e.g., /usr/local/bin)
sudo cp linuxftools-*/* /usr/local/bin/

# Or install to ~/.local/bin (make sure it's in your PATH)
mkdir -p ~/.local/bin
cp linuxftools-*/* ~/.local/bin/

# Clean up
rm -rf linuxftools-* linuxftools-Linux-x86_64-musl.tar.gz
```

### From Source

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
