use crate::tokenize::{Token, TokenType};
use std::vec;

#[derive(Debug)]
pub enum AbstractSyntaxTreeSymbol {
    AbstractSyntaxTreeSymbolEntry,
    AbstractSyntaxTreeSymbolExit(i32),
}

#[derive(Debug)]
pub struct AbstractSyntaxTreeNode {
    pub symbol: AbstractSyntaxTreeSymbol,
    pub children: Vec<AbstractSyntaxTreeNode>,
}

#[derive(Debug, PartialEq)]
pub enum ParseTreeSymbol {
    ParseTreeSymbolNodeEntryPoint,
    ParseTreeSymbolNodeStatement,
    ParseTreeSymbolNodeExpression,
    ParseTreeSymbolNodeExit,
    ParseTreeSymbolTerminalExit,
    ParseTreeSymbolTerminalSemicolon,
    ParseTreeSymbolTerminalIntegerLiteral,
}

pub struct ParseTreeNode {
    symbol: ParseTreeSymbol,
    children: Vec<ParseTreeNode>,
    value: Option<String>,
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

        let mut entry_node = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolNodeEntryPoint,
            children: Vec::new(),
            value: None,
        };

        while !self.is_at_end() {
            match self.parse_statement() {
                Ok(stmt) => entry_node.children.push(stmt),
                Err(e) => {
                    eprintln!("Fatal -- {}", e);
                    break;
                }
            }
        }

        /*        if !self.is_at_end() {
            eprintln!("ParseError: Unexpected tokens after end of entry point");
        }*/

        entry_node
    }

    fn parse_statement(&mut self) -> Result<ParseTreeNode, String> {
        let token = &self.current().unwrap();

        let mut statement_node = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolNodeStatement,
            children: Vec::new(),
            value: None,
        };

        match token.token_type {
            TokenType::TokenTypeExit => {
                statement_node.children.push(self.parse_exit()?);
                Ok(statement_node)
            }
            _ => Err(format!(
                "ParseError: unrecognized token type: {:?}",
                token.token_type
            )),
        }
    }

    fn parse_exit(&mut self) -> Result<ParseTreeNode, String> {
        if self.peek() != None
            && self.peek().unwrap().token_type == TokenType::TokenTypeIntegerLiteral
        {
            if self.peek_ahead(2) != None
                && self.peek_ahead(2).unwrap().token_type == TokenType::TokenTypeSemicolon
            {
                let exit_terminal = ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolTerminalExit,
                    children: Vec::new(),
                    value: None,
                };
                self.consume();

                let expr_node = self.parse_expression()?;

                let semi_terminal = ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolTerminalSemicolon,
                    children: Vec::new(),
                    value: None,
                };
                self.consume();

                Ok(ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolNodeExit,
                    children: vec![exit_terminal, expr_node, semi_terminal],
                    value: None,
                })
            } else {
                Err("MissingTokenError: expected Semicolon, found None"
                    .parse()
                    .unwrap())
            }
        } else {
            Err("MissingTokenError: expected IntegerLiteral, found None"
                .parse()
                .unwrap())
        }
    }

    fn parse_expression(&mut self) -> Result<ParseTreeNode, String> {
        if self.current() != None
            && self.current().unwrap().token_type == TokenType::TokenTypeIntegerLiteral
        {
            let node = ParseTreeNode {
                symbol: ParseTreeSymbol::ParseTreeSymbolNodeExpression,
                children: vec![ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolTerminalIntegerLiteral,
                    children: Vec::new(),
                    value: self.current().unwrap().value.clone(),
                }],
                value: None,
            };
            self.consume();
            Ok(node)
        } else {
            Err(
                "MissingTokenError: expected IntegerLiteral from expression, found None"
                    .parse()
                    .unwrap(),
            )
        }
    }

    pub fn print_ast(&mut self, node: &AbstractSyntaxTreeNode, indent: usize) {
        for _i in 0..indent {
            print!("  ");
        }
        println!("{:?}", node.symbol);

        for child in &node.children {
            self.print_ast(child, indent + 1);
        }
    }

    pub fn build_ast(&self, parse_tree: &ParseTreeNode) -> AbstractSyntaxTreeNode {
        match parse_tree.symbol {
            ParseTreeSymbol::ParseTreeSymbolNodeEntryPoint => {
                let entry_node = AbstractSyntaxTreeNode {
                    symbol: AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolEntry,
                    children: parse_tree
                        .children
                        .iter()
                        .filter_map(|child| match child.symbol {
                            ParseTreeSymbol::ParseTreeSymbolNodeStatement => {
                                Some(self.build_ast(child))
                            }
                            _ => None,
                        })
                        .collect(),
                };
                entry_node
            }

            ParseTreeSymbol::ParseTreeSymbolNodeStatement => {
                if let Some(first_child) = parse_tree.children.first() {
                    self.build_ast(first_child)
                } else {
                    panic!("Statement node has no children");
                }
            }

            ParseTreeSymbol::ParseTreeSymbolNodeExit => {
                // [exit, expression, semicolon]
                if let Some(expr_node) = parse_tree
                    .children
                    .iter()
                    .find(|c| c.symbol == ParseTreeSymbol::ParseTreeSymbolNodeExpression)
                {
                    if let Some(int_literal_node) = expr_node.children.first() {
                        let value_str = int_literal_node
                            .value
                            .as_ref()
                            .expect("Missing integer literal value");
                        let int_value = value_str.parse::<i32>().expect("Invalid integer literal");

                        AbstractSyntaxTreeNode {
                            symbol: AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolExit(
                                int_value,
                            ),
                            children: Vec::new(),
                        }
                    } else {
                        panic!("Expression node has no integer literal child");
                    }
                } else {
                    panic!("Exit statement has no expression child");
                }
            }

            _ => {
                panic!("Unexpected parse tree node: {:?}", parse_tree.symbol);
            }
        }
    }
}
