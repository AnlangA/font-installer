# Windows Font Installer

[![CI](https://github.com/AnlangA/font-installer/actions/workflows/ci.yml/badge.svg)](https://github.com/AnlangA/font-installer/actions/workflows/ci.yml)
[![Release](https://github.com/AnlangA/font-installer/actions/workflows/release.yml/badge.svg)](https://github.com/AnlangA/font-installer/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A simple Rust utility for batch installing font files on Windows systems.

## Overview

This program automates the process of installing font files to the Windows operating system. It scans a specified directory (including all subdirectories) for font files, copies them to the Windows Fonts directory, and registers them in the system.

## Features

- **Recursive directory scanning**: Searches for fonts in all subdirectories
- Scans directories for common font file formats (TTF, OTF, TTC, PFB, PFM)
- Detects and skips already installed fonts
- Provides detailed installation statistics
- Supports OpenType and TrueType fonts
- Simple command-line interface
- Optimized release builds with LTO and binary stripping

## Download

Download the latest release from the [Releases page](https://github.com/AnlangA/font-installer/releases/latest).

### Quick Download

| Platform | Download |
|----------|----------|
| Windows x64 | [font-installer-windows-x64.zip](https://github.com/AnlangA/font-installer/releases/latest/download/font-installer-windows-x64.zip) |

## Requirements

- Windows operating system
- Administrator privileges (for installing fonts)

## Installation

### From Release (Recommended)

1. Download the latest release from the [Releases page](https://github.com/AnlangA/font-installer/releases/latest)
2. Extract the zip file
3. Run `font-installer.exe` with administrator privileges

### From Source

1. Clone or download this repository
2. Navigate to the project directory
3. Build the program using Cargo:

```bash
cargo build --release
```

The compiled executable will be available in the `target/release` directory.

## Usage

Run the program with optional path to the directory containing font files:

```bash
font-installer.exe "C:\path\to\fonts"
```

If no directory is specified, the program will use the current directory:

```bash
font-installer.exe
```

### Command Line Options

```
Usage: font-installer [OPTIONS] [DIRECTORY]

Arguments:
  [DIRECTORY]  Path to directory containing font files (default: current directory)

Options:
  -h, --help     Print this help message
  -v, --version  Print version information
```

### Example Output

```
Scanning for font files in: C:\path\to\fonts (including subdirectories)
Windows Fonts directory: C:\Windows\Fonts
Found 20 font file(s).
Processing font: HKGrotesk-Black.otf
  Copied to Windows Fonts directory
  Successfully installed: HKGrotesk-Black
Processing font: HKGrotesk-Bold.otf
  Font already installed: HKGrotesk-Bold
[...]

Installation complete!
  Fonts installed: 15
  Fonts skipped (already installed): 5
  Fonts failed: 0
```

## Supported Font Formats

- OpenType fonts (.otf)
- TrueType fonts (.ttf)
- TrueType Collection (.ttc)
- Type 1 fonts (.pfb, .pfm)

## Notes

- Running the program with administrator privileges is recommended to ensure proper access to the Windows Fonts directory
- The program automatically skips fonts that are already installed
- For successful installation, font files should be valid and not corrupted

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions, issues, and feature requests are welcome!
