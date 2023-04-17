use duct::cmd;
use std::{fs, io};
use tempfile::NamedTempFile;

pub fn copy_to_temp(file_path: &str) -> io::Result<NamedTempFile> {
    let temp_file = NamedTempFile::new()?;

    if let Err(e) = fs::copy(file_path, temp_file.path()) {
        if e.kind() == io::ErrorKind::PermissionDenied {
            // Use doas if permission denied
            cmd!("doas", "cp", file_path, temp_file.path()).run()?;
        } else {
            return Err(e);
        }
    }

    Ok(temp_file)
}
