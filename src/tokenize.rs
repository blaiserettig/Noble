use std::process::exit;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    TokenTypeExit,
    TokenTypeIntegerLiteral,
    TokenTypeSemicolon,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<String>,
}

pub struct Tokenizer {
    chars: Vec<char>,
    index: usize,
}

impl Tokenizer {
    pub fn new(input_string: String) -> Self {
        Self {
            chars: input_string.chars().collect(),
            index: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut buffer: Vec<char> = Vec::new();
        
        while !self.is_at_end() {
            println!("{:?}", self.current().unwrap());
            if self.current().unwrap().is_ascii_alphabetic() {
                buffer.push(self.consume());
                while self.current() != None && self.current().unwrap().is_ascii_alphanumeric() {
                    buffer.push(self.consume());
                }
                if buffer == ['e', 'x', 'i', 't'] {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeExit,
                        value: None,
                    });
                } else {
                    eprintln!("{:?}", "Tokenization Error!");
                    exit(1);
                }
            } else if self.current().unwrap().is_ascii_digit() {
                buffer.push(self.consume());
                while self.current() != None && self.current().unwrap().is_ascii_digit() {
                    buffer.push(self.consume());
                }
                tokens.push(Token {
                    token_type: TokenType::TokenTypeIntegerLiteral,
                    value: Some(buffer.iter().collect()),
                });
            } else if self.current().unwrap() == ';' {
                self.consume();
                tokens.push(Token {
                    token_type: TokenType::TokenTypeSemicolon,
                    value: None,
                });
            } else if self.chars[self.index].is_ascii_whitespace() {
                self.consume();
            } else {
                eprintln!("{:?}", "Tokenization Error!");
                exit(1);
            }
            buffer.clear();
        }
        tokens
    }

    pub fn current(&mut self) -> Option<char> {
        if self.index < self.chars.len() {
            Some(self.chars[self.index])
        } else {
            None
        }
    }
    
    pub fn is_at_end(&self) -> bool {
        self.index >= self.chars.len()
    }
    
    pub fn peek(&mut self) -> Option<char> {
        self.peek_ahead(1)
    }

    pub fn peek_ahead(&mut self, ahead: usize) -> Option<char> {
        if self.index + ahead >= self.chars.len() {
            None
        } else {
            Some(self.chars[self.index + ahead])
        }
    }

    pub fn consume(&mut self) -> char {
        let c: char = self.chars[self.index];
        self.index += 1;
        c
    }
}