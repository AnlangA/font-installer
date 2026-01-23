//! Windows Font Installer
//!
//! A simple utility for batch installing font files on Windows systems.
//! Supports TTF, OTF, TTC, PFB, and PFM font formats.

#![cfg(windows)]

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use winreg::enums::*;
use winreg::RegKey;

/// Supported font file extensions
const FONT_EXTENSIONS: &[&str] = &["ttf", "otf", "ttc", "pfb", "pfm"];

/// Installation statistics
#[derive(Default)]
struct InstallStats {
    installed: u32,
    skipped: u32,
    failed: u32,
}

impl InstallStats {
    fn print_summary(&self) {
        println!("\nInstallation complete!");
        println!("  Fonts installed: {}", self.installed);
        println!("  Fonts skipped (already installed): {}", self.skipped);
        println!("  Fonts failed: {}", self.failed);
    }
}

/// Font installer configuration and state
struct FontInstaller {
    source_dir: PathBuf,
    fonts_dir: PathBuf,
    fonts_reg_key: RegKey,
    stats: InstallStats,
}

impl FontInstaller {
    /// Create a new FontInstaller instance
    fn new(source_dir: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        if !source_dir.is_dir() {
            return Err(format!("{} is not a valid directory", source_dir.display()).into());
        }

        // Get Windows Fonts directory from registry
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let fonts_reg_key =
            hklm.open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Fonts")?;

        let windows_reg = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion")?;
        let system_root: String = windows_reg.get_value("SystemRoot")?;
        let fonts_dir = Path::new(&system_root).join("Fonts");

        Ok(Self {
            source_dir,
            fonts_dir,
            fonts_reg_key,
            stats: InstallStats::default(),
        })
    }

    /// Run the font installation process
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!(
            "Scanning for font files in: {} (including subdirectories)",
            self.source_dir.display()
        );
        println!("Windows Fonts directory: {}", self.fonts_dir.display());

        // Collect all font files recursively
        let font_files = collect_font_files(&self.source_dir);

        if font_files.is_empty() {
            println!("No font files found.");
        } else {
            println!("Found {} font file(s).", font_files.len());
            for path in font_files {
                self.process_font(&path);
            }
        }

        self.stats.print_summary();
        Ok(())
    }

    /// Process a single font file
    fn process_font(&mut self, path: &Path) {
        let file_name = match path.file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => return,
        };

        println!("Processing font: {}", file_name);

        let font_name = get_font_name(&file_name);

        // Check if font is already installed
        if self
            .fonts_reg_key
            .get_value::<String, _>(&font_name)
            .is_ok()
        {
            println!("  Font already installed: {}", font_name);
            self.stats.skipped += 1;
            return;
        }

        // Copy and register font
        let dest_path = self.fonts_dir.join(&file_name);
        match fs::copy(path, &dest_path) {
            Ok(_) => {
                println!("  Copied to Windows Fonts directory");

                if let Err(e) = add_font_to_registry(&file_name) {
                    println!("  Failed to add to registry: {}", e);
                    self.stats.failed += 1;
                    let _ = fs::remove_file(&dest_path);
                } else {
                    println!("  Successfully installed: {}", font_name);
                    self.stats.installed += 1;
                }
            }
            Err(e) => {
                println!("  Failed to copy font file: {}", e);
                self.stats.failed += 1;
            }
        }
    }
}

/// Check if file extension is a font extension
fn is_font_extension(extension: &str) -> bool {
    FONT_EXTENSIONS.contains(&extension.to_lowercase().as_str())
}

/// Recursively collect all font files from a directory and its subdirectories
fn collect_font_files(dir: &Path) -> Vec<PathBuf> {
    let mut font_files = Vec::new();
    collect_font_files_recursive(dir, &mut font_files);
    font_files
}

/// Helper function to recursively traverse directories and collect font files
fn collect_font_files_recursive(dir: &Path, font_files: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_dir() {
                // Recursively search subdirectories
                collect_font_files_recursive(&path, font_files);
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if is_font_extension(ext) {
                    font_files.push(path);
                }
            }
        }
    }
}

/// Extract font name from filename (without extension)
fn get_font_name(filename: &str) -> String {
    Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(filename)
        .to_string()
}

/// Add font to Windows registry using PowerShell Shell.Application
fn add_font_to_registry(font_filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let ps_command = format!(
        "$fonts = (New-Object -ComObject Shell.Application).Namespace(0x14); \
         $fonts.CopyHere('C:\\Windows\\Fonts\\{}'); \
         [System.Runtime.Interopservices.Marshal]::ReleaseComObject($fonts) | Out-Null",
        font_filename
    );

    let output = Command::new("powershell")
        .args(["-Command", &ps_command])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("PowerShell command failed: {}", error).into());
    }

    Ok(())
}

fn print_usage() {
    println!("Windows Font Installer v{}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("Usage: font-installer [OPTIONS] [DIRECTORY]");
    println!();
    println!("Arguments:");
    println!("  [DIRECTORY]  Path to directory containing font files (default: current directory)");
    println!();
    println!("Options:");
    println!("  -h, --help     Print this help message");
    println!("  -v, --version  Print version information");
    println!();
    println!("Supported formats: TTF, OTF, TTC, PFB, PFM");
}

fn print_version() {
    println!("font-installer {}", env!("CARGO_PKG_VERSION"));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // Handle flags
    if args.len() > 1 {
        match args[1].as_str() {
            "-h" | "--help" => {
                print_usage();
                return Ok(());
            }
            "-v" | "--version" => {
                print_version();
                return Ok(());
            }
            _ => {}
        }
    }

    // Get the source directory
    let font_dir = if args.len() > 1 && !args[1].starts_with('-') {
        PathBuf::from(&args[1])
    } else {
        println!("No directory specified. Using current directory.");
        env::current_dir()?
    };

    let mut installer = FontInstaller::new(font_dir)?;
    installer.run()
}
