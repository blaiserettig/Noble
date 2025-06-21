use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;

#[derive(Debug)]
enum TokenType {
    TokenTypeReturn,
    TokenTypeIntegerLiteral,
    TokenTypeSemicolon,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    value: Option<String>,
}

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

    let tokens: Vec<Token> = tokenize(file_contents);
    for  token in tokens {
        println!("{:?}", token);
    }
}

fn read_file(file_path: PathBuf) -> String {
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    contents
}

fn tokenize(input_string: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut buffer: Vec<char> = Vec::new();

    let chars: Vec<char> = input_string.chars().collect();
    let mut index: usize = 0;
    
    while index < chars.len() {
        if chars[index].is_ascii_alphabetic() {
            buffer.push(chars[index]);
            index += 1;
            while chars[index].is_ascii_alphanumeric() {
                buffer.push(chars[index]);
                index += 1;
            }
            if buffer == ['r', 'e', 't', 'u', 'r', 'n'] {
                tokens.push(Token {
                    token_type: TokenType::TokenTypeReturn,
                    value: None,
                });
            } else {
                
            }
        } else if chars[index].is_ascii_digit() {
            buffer.push(chars[index]);
            index += 1;
            while chars[index].is_ascii_digit() {
                buffer.push(chars[index]);
                index += 1;
            }
            tokens.push(Token {
                token_type: TokenType::TokenTypeIntegerLiteral,
                value: Some(buffer.iter().collect()),
            });
        } else if chars[index] == ';' {
            tokens.push(Token {
                token_type: TokenType::TokenTypeSemicolon,
                value: None,
            });
            index += 1;
        } else if chars[index].is_ascii_whitespace() {
            index += 1;
        } else {
            eprintln!("{:?}", "Tokenization Error!");
            exit(1);
        }
        buffer.clear();
    }
    tokens
}
