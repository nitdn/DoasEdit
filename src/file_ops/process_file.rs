

use std::{fs, io, os::unix::fs::PermissionsExt, process::Command};
use crate::file_ops::{copy_to_temp, edit_temp_file, compare_and_copy}; // Add this line to import the functions

pub fn process_file(file_path: &str) -> io::Result<()> {
    let file_metadata = fs::metadata(file_path);

    if let Err(e) = file_metadata {
        if e.kind() == io::ErrorKind::NotFound {
            // Create the file if it doesn't exist
            fs::write(file_path, "").or_else(|e| match e.kind() {
                io::ErrorKind::PermissionDenied => {
                    Command::new("doas")
                        .arg("touch")
                        .arg(file_path)
                        .status()
                        .map_err(|_| {
                            io::Error::new(
                                io::ErrorKind::PermissionDenied,
                                format!("Failed to create file {} with doas: {}", file_path, e),
                            )
                        })?;
                    Ok(()) // Add this line to return Ok(()) on success
                }
                _ => Err(e),
            })?;
        } else {
            return Err(e);
        }
    }

    // Check file permissions
    let perm = fs::metadata(file_path)?.permissions();
    if perm.mode() & 0o200 == 0 {
        // If no write permission, use doas
        Command::new("doas")
            .arg("touch")
            .arg(file_path)
            .status()
            .map_err(|e| {
                io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    format!("Failed to change permissions with doas: {}", e),
                )
            })?;
    }

    let temp_file = copy_to_temp(file_path)?;
    edit_temp_file(&temp_file)?;
    compare_and_copy(file_path, &temp_file)?;

    Ok(())
}
