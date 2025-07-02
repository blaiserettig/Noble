use crate::tokenize::{Token, TokenType};

#[derive(Debug)]
pub enum GrammarSymbols {
    GrammarSymbolNodeEntryPoint,
    GrammarSymbolNodeStatement,
    GrammarSymbolNodeExpression,
    GrammarSymbolNodeExit,
    GrammarSymbolTerminalExit,
    GrammarSymbolTerminalSemicolon,
    GrammarSymbolTerminalIntegerLiteral,
}

pub struct ParseTreeNode {
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
        self.parse_entry()
    }

    pub fn print_tree(&mut self, node: &ParseTreeNode, indent: usize) {
        for _i in 0..indent {
            print!("  ");
        }
        println!("{:?}", node.symbol);

        for child in &node.children {
            self.print_tree(child, indent + 1);
        }
    }

    fn is_at_end(&self) -> bool {
        self.token_index >= self.tokens.len()
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.token_index)
    }

    fn peek(&mut self) -> Option<&Token> {
        self.peek_ahead(1)
    }

    fn peek_ahead(&mut self, ahead: usize) -> Option<&Token> {
        if self.token_index + ahead >= self.tokens.len() {
            None
        } else {
            Some(&self.tokens[self.token_index + ahead])
        }
    }

    fn consume(&mut self) -> &Token {
        let token = &self.tokens[self.token_index];
        self.token_index += 1;
        token
    }

    fn parse_entry(&mut self) -> ParseTreeNode {
        self.consume();

        let mut entry_node: ParseTreeNode = ParseTreeNode { symbol: GrammarSymbols::GrammarSymbolNodeEntryPoint, children: Vec::new() };

        while !self.is_at_end() {
            entry_node.children.push(self.parse_statement().expect("ParseError: Could not parse Statement"));
        }

        if !self.is_at_end() {
            eprintln!("ParseError: Unexpected tokens after end of entry point");
        }

        entry_node
    }

    fn parse_statement(&mut self) -> Option<ParseTreeNode> {
        let token = &self.current().unwrap();

        let mut statement_node = ParseTreeNode { symbol: GrammarSymbols::GrammarSymbolNodeStatement, children: Vec::new() };

        match token.token_type {
            TokenType::TokenTypeExit => {
                statement_node.children.push(self.parse_exit()?);
                Option::from(statement_node)
            }
            _ => {
                eprintln!("ParseError: Unrecognized token type: {:?}", token.token_type);
                None
            }
        }
    }

    fn parse_exit(&mut self) -> Option<ParseTreeNode> {
        if self.peek() != None && self.peek().unwrap().token_type == TokenType::TokenTypeIntegerLiteral {
            if self.peek_ahead(2) != None && self.peek_ahead(2).unwrap().token_type == TokenType::TokenTypeSemicolon {
                let exit_terminal = ParseTreeNode { symbol: GrammarSymbols::GrammarSymbolTerminalExit, children: Vec::new() };
                self.consume();

                let expr_node = self.parse_expression()?;

                let semi_terminal = ParseTreeNode { symbol: GrammarSymbols::GrammarSymbolTerminalSemicolon, children: Vec::new() };
                self.consume();

                Some(ParseTreeNode { symbol: GrammarSymbols::GrammarSymbolNodeExit, children: vec![exit_terminal, expr_node, semi_terminal] })
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
        if self.current() != None && self.current().unwrap().token_type == TokenType::TokenTypeIntegerLiteral {
            let node = ParseTreeNode {
                symbol: GrammarSymbols::GrammarSymbolNodeExpression,
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