use crate::tokenize::{Token, TokenType};

pub enum GrammarSymbols {
    GrammarSymbolNodeEntryPoint,
    GrammarSymbolNodeStatement,
    GrammarSymbolNodeExpression,
    GrammarSymbolNodeExit,
    GrammarSymbolTerminalExit,
    GrammarSymbolTerminalSemicolon,
    GrammarSymbolTerminalIntegerLiteral,
}

struct ParseTreeNode {
    symbol: GrammarSymbols,
    children: Vec<ParseTreeNode>,
}

pub struct Parser {
    tokens: Vec<Token>,
    token_index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            token_index: 0,
        }
    }

    // Assume the tokens are given to us starting from the entry point
    pub fn parse(&mut self) -> ParseTreeNode {
        let mut entry_node: ParseTreeNode = ParseTreeNode { symbol: GrammarSymbols::GrammarSymbolNodeEntryPoint, children: Vec::new() };

        while !self.is_at_end() {
            let token: &Token = self.current().expect("ParseError: Could not acquire token");

            match token.token_type {
                TokenType::TokenTypeExit => {
                    entry_node.children.push(self.parse_exit().expect("ParseError: Could not parse TokenTypeExit"));
                }
                TokenType::TokenTypeIntegerLiteral => {}
                TokenType::TokenTypeSemicolon => {}
            }
        }
        entry_node
    }

    pub fn is_at_end(&self) -> bool {
        self.token_index >= self.tokens.len()
    }

    pub fn current(&self) -> Option<&Token> {
        self.tokens.get(self.token_index)
    }

    pub fn peek(&mut self) -> Option<&Token> {
        self.peek_ahead(1)
    }

    pub fn peek_ahead(&mut self, ahead: usize) -> Option<&Token> {
        if self.token_index + ahead >= self.tokens.len() {
            None
        } else {
            Some(&self.tokens[self.token_index + ahead])
        }
    }

    pub fn consume(&mut self) -> &Token {
        let token = &self.tokens[self.token_index];
        self.token_index += 1;
        token
    }

    fn parse_exit(&mut self) -> Option<ParseTreeNode> {
        if self.peek() != None && self.peek().unwrap().token_type == TokenType::TokenTypeIntegerLiteral {
            if self.peek_ahead(2) != None && self.peek_ahead(2).unwrap().token_type == TokenType::TokenTypeSemicolon {

                let exit_terminal = ParseTreeNode {symbol: GrammarSymbols::GrammarSymbolTerminalExit, children: Vec::new()};
                self.consume();

                let expr_node = self.parse_expression()?;

                let semi_terminal = ParseTreeNode {symbol: GrammarSymbols::GrammarSymbolTerminalSemicolon, children: Vec::new()};
                self.consume();

                Some(ParseTreeNode {symbol: GrammarSymbols::GrammarSymbolNodeExit, children: vec![exit_terminal, expr_node, semi_terminal]})

            } else {
                eprintln!("MissingTokenError: Expected Semicolon, found None");
                None
            }
        } else {
            eprintln!("MissingTokenError: Expected IntegerLiteral, found None");
            None
        }
    }

    fn parse_expression(&mut self) -> Option<ParseTreeNode> {
        if self.peek() != None && self.peek().unwrap().token_type == TokenType::TokenTypeIntegerLiteral {
            let node = ParseTreeNode { symbol: GrammarSymbols::GrammarSymbolNodeExpression,
                children: vec![ParseTreeNode {
                    symbol: GrammarSymbols::GrammarSymbolTerminalIntegerLiteral,
                    children: Vec::new(),
                }]
            };
            self.consume();
            Some(node)
        } else {
            eprintln!("MissingTokenError: Expected IntegerLiteral from expression, found None");
            None
        }
    }
}