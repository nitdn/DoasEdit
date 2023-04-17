use diff::lines;
use duct::cmd;
use std::{fs, io};

pub fn compare_and_copy(file_path: &str, temp_file: &tempfile::NamedTempFile) -> io::Result<()> {
    let source_content = fs::read_to_string(file_path)?;
    let temp_content = fs::read_to_string(temp_file)?;

    let diff = lines(&source_content, &temp_content);

    if diff
        .into_iter()
        .any(|line| !matches!(line, diff::Result::Both(_, _)))
    {
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
