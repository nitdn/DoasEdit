use duct::cmd;
use std::{env, io};

pub fn edit_temp_file(temp_file: &tempfile::NamedTempFile) -> io::Result<()> {
    let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());

    // Use doas if needed
    if let Err(e) = cmd!(editor, temp_file.path().as_os_str()).run() {
        eprintln!("Failed to open editor: {:?}", e);
        std::process::exit(1);
    }

    Ok(())
}
