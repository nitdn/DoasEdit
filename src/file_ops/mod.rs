mod compare_and_copy;
mod copy_to_temp;
mod edit_temp_file;
mod process_file; // Add this line to include the new process_file.rs file

pub use compare_and_copy::compare_and_copy;
pub use copy_to_temp::copy_to_temp;
pub use edit_temp_file::edit_temp_file;
pub use process_file::process_file; // Add this line to re-export the process_file function
