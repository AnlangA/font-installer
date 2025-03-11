use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use winreg::enums::*;
use winreg::RegKey;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the path from command line arguments or use the current directory
    let font_dir = match env::args().nth(1) {
        Some(path) => PathBuf::from(path),
        None => {
            println!("No directory specified. Using current directory.");
            env::current_dir()?
        }
    };

    if !font_dir.is_dir() {
        return Err(format!("{} is not a valid directory", font_dir.display()).into());
    }

    println!("Scanning for font files in: {}", font_dir.display());

    // Get Windows Fonts directory from registry
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let fonts_reg_key = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Fonts")?;
    
    let windows_reg = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion")?;
    let system_root: String = windows_reg.get_value("SystemRoot")?;
    let fonts_dir = Path::new(&system_root).join("Fonts");

    println!("Windows Fonts directory: {}", fonts_dir.display());

    // Track installation stats
    let mut installed_count = 0;
    let mut skipped_count = 0;
    let mut failed_count = 0;

    // Process each file in the directory
    for entry in fs::read_dir(&font_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        // Skip directories and non-font files
        if path.is_dir() {
            continue;
        }
        
        // Check if it's a font file by extension
        let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
        if !is_font_extension(extension) {
            continue;
        }
        
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
        println!("Processing font: {}", file_name);
        
        // Check if font is already installed
        let font_name = get_font_name(&file_name);
        let is_installed = fonts_reg_key.get_value::<String, _>(&font_name).is_ok();
        
        if is_installed {
            println!("  Font already installed: {}", font_name);
            skipped_count += 1;
            continue;
        }
        
        // Copy font file to Windows Fonts directory
        let dest_path = fonts_dir.join(&file_name);
        match fs::copy(&path, &dest_path) {
            Ok(_) => {
                println!("  Copied to Windows Fonts directory");
                
                // Add font to registry using Windows API
                if let Err(e) = add_font_to_registry(&file_name) {
                    println!("  Failed to add to registry: {}", e);
                    failed_count += 1;
                    // Try to clean up the copied file
                    let _ = fs::remove_file(&dest_path);
                } else {
                    println!("  Successfully installed: {}", font_name);
                    installed_count += 1;
                }
            }
            Err(e) => {
                println!("  Failed to copy font file: {}", e);
                failed_count += 1;
            }
        }
    }
    
    println!("\nInstallation complete!");
    println!("  Fonts installed: {}", installed_count);
    println!("  Fonts skipped (already installed): {}", skipped_count);
    println!("  Fonts failed: {}", failed_count);
    
    Ok(())
}

// Check if file extension is a font extension
fn is_font_extension(extension: &str) -> bool {
    matches!(extension.to_lowercase().as_str(), "ttf" | "otf" | "ttc" | "pfb" | "pfm")
}

// Extract font name from filename
fn get_font_name(filename: &str) -> String {
    filename.split('.').next().unwrap_or(filename).to_string()
}

// Add font to Windows registry using Windows shell commands
fn add_font_to_registry(font_filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Use PowerShell to add font to registry
    let ps_command = format!(
        "$fonts = (New-Object -ComObject Shell.Application).Namespace(0x14); \
         $fonts.CopyHere('C:\\Windows\\Fonts\\{}'); \
         [System.Runtime.Interopservices.Marshal]::ReleaseComObject($fonts) | Out-Null",
        font_filename
    );
    
    let output = Command::new("powershell")
        .args(&["-Command", &ps_command])
        .output()?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("PowerShell command failed: {}", error).into());
    }
    
    Ok(())
}