use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage: ./d [filename]");
        return;
    }

    let file_path: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join(&args[1]);

    let file_contents: String = read_file(file_path);

    println!("{:?}", file_contents);
}

fn read_file(file_path: PathBuf) -> String {
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    contents
}