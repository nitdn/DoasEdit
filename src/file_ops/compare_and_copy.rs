use duct::cmd;
use std::{fs, io, process::Command};

pub fn compare_and_copy(file_path: &str, temp_file: &tempfile::NamedTempFile) -> io::Result<()> {
    let output = Command::new("diff")
        .arg("-q")
        .arg(file_path)
        .arg(temp_file.path())
        .output()?;

    if !output.stdout.is_empty() {
        // diff found differences
        if let Err(e) = fs::copy(temp_file.path(), file_path) {
            if e.kind() == io::ErrorKind::PermissionDenied {
                // Use doas if permission denied
                cmd!("doas", "cp", temp_file.path(), file_path).run()?;
            } else {
                return Err(e);
            }
        }
    }

    Ok(())
}
