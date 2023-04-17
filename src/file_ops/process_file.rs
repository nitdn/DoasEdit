use std::{fs, io, os::unix::fs::PermissionsExt};

pub fn process_file(file_path: &str) -> io::Result<()> {
    let file_metadata = fs::metadata(file_path);

    if let Err(e) = file_metadata {
        if e.kind() == io::ErrorKind::NotFound {
            // Create the file if it doesn't exist
            fs::write(file_path, "")?;
        } else {
            return Err(e);
        }
    }

    // Check file permissions
    let perm = fs::metadata(file_path)?.permissions();
    if perm.mode() & 0o200 == 0 {
        // If no write permission, use doas
        cmd!("doas", "touch", file_path).run()?;
    }

    let temp_file = copy_to_temp(file_path)?;
    edit_temp_file(&temp_file)?;
    compare_and_copy(file_path, &temp_file)?;

    Ok(())
}
