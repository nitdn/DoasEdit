mod compare_and_copy;
mod copy_to_temp;
mod edit_temp_file;

use std::io;

pub fn process_file(file_path: &str) -> io::Result<()> {
    // Check file, create if not exists
    // Handle permission, use doas if needed
    let temp_file = copy_to_temp::copy_to_temp(file_path)?;
    edit_temp_file::edit_temp_file(&temp_file)?;
    compare_and_copy::compare_and_copy(file_path, &temp_file)
}
