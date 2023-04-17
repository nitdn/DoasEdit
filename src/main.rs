mod file_ops;
use file_ops::process_file;
use std::{env, io};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: DoasEdit <file_path>");
        std::process::exit(1);
    }

    let file_path = &args[1];
    process_file(file_path)
}
