use std::collections::HashMap;
use crate::tokenize::{Token, TokenType};
use std::vec;

#[derive(Debug)]
pub enum AbstractSyntaxTreeSymbol {
    AbstractSyntaxTreeSymbolEntry,
    AbstractSyntaxTreeSymbolExit(Expr),
    AbstractSyntaxTreeSymbolVariableDeclaration {
        name: String,
        type_: Type,
        value: Expr,
    },
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
    ParseTreeSymbolNodeVariable,
    ParseTreeSymbolNodeType,
    ParseTreeSymbolTerminalExit,
    ParseTreeSymbolTerminalSemicolon,
    ParseTreeSymbolTerminalIntegerLiteral,
    ParseTreeSymbolTerminalEquals,
    ParseTreeSymbolTerminalI32S,
    ParseTreeSymbolTerminalIdentifier,
}

#[derive(Debug)]
pub struct ParseTreeNode {
    symbol: ParseTreeSymbol,
    children: Vec<ParseTreeNode>,
    value: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Type {
    I32S,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i32),
    Ident(String),
}

struct VarEntry {
    var_type: Type,
    var_value: Expr,
}

pub struct Parser {
    tokens: Vec<Token>,
    token_index: usize,
    symbol_table: HashMap<String, VarEntry>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            token_index: 0,
            symbol_table: HashMap::new(),
        }
    }

    // Assume the tokens are given to us starting from the entry point
    pub fn parse(&mut self) -> ParseTreeNode {
        self.parse_entry()
    }

    pub fn print_tree(&mut self, node: &ParseTreeNode, indent: usize) {
        for _i in 0..indent {
            print!("    ");
        }
        println!("{:?}", node.symbol);

        for _i in 0..indent {
            print!("    ");
        }
        println!("{:?}", node.value);

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

/*    fn peek(&mut self) -> Option<&Token> {
        self.peek_ahead(1)
    }

    fn peek_ahead(&mut self, ahead: usize) -> Option<&Token> {
        if self.token_index + ahead >= self.tokens.len() {
            None
        } else {
            Some(&self.tokens[self.token_index + ahead])
        }
    }*/

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
            TokenType::TokenTypeTypeI32S => {
                statement_node.children.push(self.parse_variable()?);
                Ok(statement_node)
            }
            _ => Err(format!(
                "ParseError: unrecognized token type: {:?}",
                token.token_type
            )),
        }
    }

    fn parse_exit(&mut self) -> Result<ParseTreeNode, String> {
        let exit_terminal = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalExit,
            children: Vec::new(),
            value: None,
        };
        self.consume();

        let expr_node = self.parse_expression()?;

        let semi_terminal = if self.current().map_or(false, |t| t.token_type == TokenType::TokenTypeSemicolon) {
            let node = ParseTreeNode {
                symbol: ParseTreeSymbol::ParseTreeSymbolTerminalSemicolon,
                children: Vec::new(),
                value: None,
            };
            self.consume();
            node
        } else {
            return Err(format!(
                "MissingTokenError: expected Semicolon, found: {:?}",
                self.current().map(|t| &t.token_type)
            ));
        };

        Ok(ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolNodeExit,
            children: vec![exit_terminal, expr_node, semi_terminal],
            value: None,
        })
    }

    fn parse_expression(&mut self) -> Result<ParseTreeNode, String> {
        let token = self.current().ok_or("ParseError: Unexpected end of input in expression")?;

        let child = match token.token_type {
            TokenType::TokenTypeIntegerLiteral => ParseTreeNode {
                symbol: ParseTreeSymbol::ParseTreeSymbolTerminalIntegerLiteral,
                children: Vec::new(),
                value: token.value.clone(),
            },

            TokenType::TokenTypeIdentifier => ParseTreeNode {
                symbol: ParseTreeSymbol::ParseTreeSymbolTerminalIdentifier,
                children: Vec::new(),
                value: token.value.clone(),
            },

            _ => {
                return Err(format!(
                    "Unexpected token in expression: {:?}",
                    token.token_type
                ));
            }
        };

        self.consume();
        Ok(ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolNodeExpression,
            children: vec![child],
            value: None,
        })
    }

    fn parse_variable(&mut self) -> Result<ParseTreeNode, String> {
        let type_node = self.parse_type()?;

        let ident_token = self.current().ok_or("ParseError: Expected identifier, found end of input")?;
        if ident_token.token_type != TokenType::TokenTypeIdentifier {
            return Err(format!("ParseError: Expected identifier, found {:?}", ident_token.token_type));
        }
        let ident_terminal = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalIdentifier,
            children: vec![],
            value: ident_token.value.clone(),
        };
        self.consume();

        let equals_token = self.current().ok_or("ParseError: Expected '=', found end of input")?;
        if equals_token.token_type != TokenType::TokenTypeEquals {
            return Err(format!("ParseError: Expected '=', found {:?}", equals_token.token_type));
        }
        let equals_terminal = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalEquals,
            children: vec![],
            value: None,
        };
        self.consume();

        let expr_node = self.parse_expression()?;

        let semi_token = self.current().ok_or("ParseError: Expected semicolon, found end of input")?;
        if semi_token.token_type != TokenType::TokenTypeSemicolon {
            return Err(format!("ParseError: Expected semicolon, found {:?}", semi_token.token_type));
        }
        let semi_terminal = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalSemicolon,
            children: vec![],
            value: None,
        };
        self.consume();

        self.add_var_to_map(&ident_terminal, &type_node, &expr_node);

        Ok(ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolNodeVariable,
            children: vec![
                type_node,
                ident_terminal,
                equals_terminal,
                expr_node,
                semi_terminal,
            ],
            value: None,
        })
    }

    fn parse_type(&mut self) -> Result<ParseTreeNode, String> {
        if self.current() != None
            && self.current().unwrap().token_type == TokenType::TokenTypeTypeI32S
        {
            let node = ParseTreeNode {
                symbol: ParseTreeSymbol::ParseTreeSymbolNodeType,
                children: vec![ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolTerminalI32S,
                    children: Vec::new(),
                    value: None,
                }],
                value: None,
            };
            self.consume();
            Ok(node)
        } else {
            Err(format!(
                "MissingTokenError: expected Type, found: {:?}",
                self.current().unwrap().token_type
            ))
        }
    }

    fn add_var_to_map(&mut self, node_terminal_id: &ParseTreeNode, node_type: &ParseTreeNode, node_expr: &ParseTreeNode, ) {
        let name = node_terminal_id.value.as_ref().expect("Identifier should have a value").clone();

        let var_type = self.add_var_to_map_type_helper(node_type);
        let var_value = self.add_var_to_map_expression_helper(node_expr);

        self.symbol_table.insert(name, VarEntry { var_type, var_value,
        });
    }

    fn add_var_to_map_type_helper(&mut self, node: &ParseTreeNode) -> Type {
        match node.children.first().unwrap().symbol {
            ParseTreeSymbol::ParseTreeSymbolTerminalI32S => Type::I32S,
            _ => panic!("Unsupported type node"),
        }
    }

    fn add_var_to_map_expression_helper(&mut self, node: &ParseTreeNode) -> Expr {
        let child = node.children.first().unwrap();
        match child.symbol {
            ParseTreeSymbol::ParseTreeSymbolTerminalIntegerLiteral => {
                let value = child.value.as_ref().unwrap().parse::<i32>().unwrap();
                Expr::Int(value)
            },
            ParseTreeSymbol::ParseTreeSymbolTerminalIdentifier => {
                let ident = child.value.as_ref().unwrap().clone();
                Expr::Ident(ident)
            }
            _ => panic!("Unsupported expression type"),
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
                    if let Some(value_child_node) = expr_node.children.first() {
                        
                        let expr = match value_child_node.symbol {
                            ParseTreeSymbol::ParseTreeSymbolTerminalIntegerLiteral => {
                                let v = value_child_node.value.as_ref().unwrap().parse::<i32>().unwrap();
                                Expr::Int(v)
                            }
                            ParseTreeSymbol::ParseTreeSymbolTerminalIdentifier => {
                                let name = value_child_node.value.as_ref().unwrap().to_string();
                                Expr::Ident(name)
                            }
                            _ => panic!("Invalid expression in exit"),
                        };

                        AbstractSyntaxTreeNode {
                            symbol: AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolExit(expr),
                            children: Vec::new(),
                        }
                    } else {
                        panic!("Expression node has no integer literal child");
                    }
                } else {
                    panic!("Exit statement has no expression child");
                }
            }

            ParseTreeSymbol::ParseTreeSymbolNodeVariable => {
                if let Some(terminal_id_node) = parse_tree.children.iter().
                    find(|c| c.symbol == ParseTreeSymbol::ParseTreeSymbolTerminalIdentifier) {
                    let name = terminal_id_node.value.as_ref().expect("Missing terminal");
                    let entry = self.symbol_table.get(name).unwrap();

                    AbstractSyntaxTreeNode {
                        symbol: AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolVariableDeclaration {
                            name: name.to_string(),
                            type_: entry.var_type.clone(),
                            value: entry.var_value.clone(),
                        },
                        children: Vec::new(),
                    }
                } else {
                    panic!("Variable node has no terminal identifier");
                }
            }

            _ => {
                panic!("Unexpected parse tree node: {:?}", parse_tree.symbol);
            }
        }
    }
}
