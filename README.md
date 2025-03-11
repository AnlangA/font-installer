# Windows Font Installer

A simple Rust utility for batch installing font files on Windows systems.

## Overview

This program automates the process of installing font files to the Windows operating system. It scans a specified directory for font files, copies them to the Windows Fonts directory, and registers them in the system.

## Features

- Scans directories for common font file formats (TTF, OTF, TTC, PFB, PFM)
- Detects and skips already installed fonts
- Provides detailed installation statistics
- Supports OpenType and TrueType fonts
- Simple command-line interface

## Requirements

- Windows operating system
- Rust compiler (for building from source)
- Administrator privileges (for installing fonts)

## Installation

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

### Example Output

```
Scanning for font files in: C:\path\to\fonts
Windows Fonts directory: C:\Windows\Fonts
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

This project is open-source software.

## Contributing

Contributions, issues, and feature requests are welcome!
