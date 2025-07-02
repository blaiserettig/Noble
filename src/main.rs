mod tokenize;
mod parse;

use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use crate::tokenize::{Token, TokenType, Tokenizer};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage: ./d [filename]");
        return;
    }

    let input_file_path: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join(&args[1]);

    let file_contents: String = read_file(input_file_path);

    println!("{:?}", file_contents);

    let mut tokenizer = Tokenizer::new(file_contents);
    let tokens: Vec<Token> = tokenizer.tokenize();
    for token in &tokens {
        println!("{:?}", token);
    }

    let output_file_path: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/out.asm");

    assemble(tokens, output_file_path);
}

fn read_file(file_path: PathBuf) -> String {
    let contents: String =
        fs::read_to_string(file_path).expect("Unable to read file.");
    contents
}

fn assemble(tokens: Vec<Token>, file_path: PathBuf) {
    let output_file = File::create(file_path).expect("Unable to create file.");
    let mut writer = BufWriter::new(&output_file);

    write!(&mut writer, "{}", "bits 64\ndefault rel\n\nsegment .text\nglobal mainCRTStartup\n\nmainCRTStartup:\n").expect("Unable to write to file.");

    for i in 0..tokens.len() {
        let token = &tokens[i];
        if token.token_type == TokenType::TokenTypeExit {
            if i + 1 < tokens.len() && tokens[i + 1].token_type == TokenType::TokenTypeIntegerLiteral {
                if i + 2 < tokens.len() && tokens[i + 2].token_type == TokenType::TokenTypeSemicolon {
                    write!(&mut writer, "{}", format!("    mov eax, {}\n    ret", tokens[i + 1].value.as_ref().unwrap())).expect("Unable to write to file.");
                }
            }
        }
    }
}