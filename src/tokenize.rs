use std::process::exit;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    TokenTypeEntryPoint,
    TokenTypeExit,
    TokenTypeIntegerLiteral,
    TokenTypeSemicolon,
    TokenTypeEquals,
    TokenTypeIdentifier,
    TokenTypeTypeI32S,
    TokenTypeTypeF32S,
    TokenTypeTypeBool,
    TokenTypeFloatLiteral,
    TokenTypeBooleanLiteral,
    TokenTypeFor,
    TokenTypeForIn,
    TokenTypeForTo,
    TokenTypeLeftCurlyBrace,
    TokenTypeRightCurlyBrace,
    TokenTypePlus,
    TokenTypeMinus,
    TokenTypeMultiply,
    TokenTypeDivide,
    TokenTypeLessThan,
    TokenTypeLessThanOrEqual,
    TokenTypeGreaterThan,
    TokenTypeGreaterThanOrEqual,
    TokenTypeEqualsEquals,
    TokenTypeNotEquals,
    TokenTypeLeftParen,
    TokenTypeRightParen,
}

#[derive(Debug, PartialEq)]
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

        tokens.push(Token {
            token_type: TokenType::TokenTypeEntryPoint,
            value: None,
        });

        while !self.is_at_end() {
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
                } else if buffer == ['i', '3', '2', 's'] {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeTypeI32S,
                        value: None,
                    });
                } else if buffer == ['f', '3', '2', 's'] {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeTypeF32S,
                        value: None,
                    });
                } else if buffer == ['b', 'o', 'o', 'l'] {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeTypeBool,
                        value: None,
                    });
                } else if buffer == ['t', 'r', 'u', 'e'] {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeBooleanLiteral,
                        value: Some("true".to_string()),
                    });
                } else if buffer == ['f', 'a', 'l', 's', 'e'] {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeBooleanLiteral,
                        value: Some("false".to_string()),
                    });
                } else if buffer == ['f', 'o', 'r'] {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeFor,
                        value: None,
                    })
                } else if buffer == ['i', 'n'] {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeForIn,
                        value: None,
                    })
                } else if buffer == ['t', 'o'] {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeForTo,
                        value: None,
                    })
                } else {
                    // If not a keyword, it is an identifier
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeIdentifier,
                        value: Some(buffer.iter().collect()),
                    });
                }
            } else if self.current().unwrap().is_ascii_digit() {
                buffer.push(self.consume());
                while self.current() != None && self.current().unwrap().is_ascii_digit() {
                    buffer.push(self.consume());
                }
                if self.current() != None && self.current().unwrap() == '.' {
                    buffer.push(self.consume());
                    while self.current() != None && self.current().unwrap().is_ascii_digit() {
                        buffer.push(self.consume());
                    }
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeFloatLiteral,
                        value: Some(buffer.iter().collect()),
                    });
                } else {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeIntegerLiteral,
                        value: Some(buffer.iter().collect()),
                    });
                }
            } else if self.current().unwrap() == ';' {
                self.consume();
                tokens.push(Token {
                    token_type: TokenType::TokenTypeSemicolon,
                    value: None,
                });
            } else if self.current().unwrap() == '=' {
                self.consume();
                if self.current() == Some('=') {
                    self.consume();
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeEqualsEquals,
                        value: None,
                    });
                } else {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeEquals,
                        value: None,
                    });
                }
            } else if self.current().unwrap() == '!' {
                self.consume();
                if self.current() == Some('=') {
                    self.consume();
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeNotEquals,
                        value: None,
                    });
                } else {
                    eprintln!("{:?}", "Tokenization Error: '!' must be followed by '='");
                    exit(1);
                }
            } else if self.current().unwrap() == '<' {
                self.consume();
                if self.current() == Some('=') {
                    self.consume();
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeLessThanOrEqual,
                        value: None,
                    });
                } else {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeLessThan,
                        value: None,
                    });
                }
            } else if self.current().unwrap() == '>' {
                self.consume();
                if self.current() == Some('=') {
                    self.consume();
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeGreaterThanOrEqual,
                        value: None,
                    });
                } else {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeGreaterThan,
                        value: None,
                    });
                }
            } else if self.current().unwrap() == '+' {
                self.consume();
                tokens.push(Token {
                    token_type: TokenType::TokenTypePlus,
                    value: None,
                });
            } else if self.current().unwrap() == '-' {
                self.consume();
                tokens.push(Token {
                    token_type: TokenType::TokenTypeMinus,
                    value: None,
                });
            } else if self.current().unwrap() == '*' {
                self.consume();
                tokens.push(Token {
                    token_type: TokenType::TokenTypeMultiply,
                    value: None,
                });
            } else if self.current().unwrap() == '/' {
                self.consume();
                tokens.push(Token {
                    token_type: TokenType::TokenTypeDivide,
                    value: None,
                });
            } else if self.current().unwrap() == '(' {
                self.consume();
                tokens.push(Token {
                    token_type: TokenType::TokenTypeLeftParen,
                    value: None,
                });
            } else if self.current().unwrap() == ')' {
                self.consume();
                tokens.push(Token {
                    token_type: TokenType::TokenTypeRightParen,
                    value: None,
                });
            } else if self.current().unwrap() == '{' {
                self.consume();
                tokens.push(Token {
                    token_type: TokenType::TokenTypeLeftCurlyBrace,
                    value: None,
                });
            } else if self.current().unwrap() == '}' {
                self.consume();
                tokens.push(Token {
                    token_type: TokenType::TokenTypeRightCurlyBrace,
                    value: None,
                });
            } else if self.current().unwrap().is_ascii_whitespace() {
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

    pub fn consume(&mut self) -> char {
        let c: char = self.chars[self.index];
        self.index += 1;
        c
    }
}
