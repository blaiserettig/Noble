use crate::tokenize::{Token, TokenType};
use std::collections::HashMap;
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
    AbstractSyntaxTreeSymbolVariableAssignment {
        name: String,
        value: Expr,
    },
    AbstractSyntaxTreeSymbolFor {
        iterator_name: String,
        iterator_begin: Expr,
        iterator_end: Expr,
        body: Vec<AbstractSyntaxTreeNode>,
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
    ParseTreeSymbolNodeVariableDeclaration,
    ParseTreeSymbolNodeVariableAssignment,
    ParseTreeSymbolNodeType,
    ParseTreeSymbolNodeFor,
    ParseTreeSymbolNodeBlock,
    ParseTreeSymbolNodeEquality,
    ParseTreeSymbolNodeComparison,
    ParseTreeSymbolNodeAdd,
    ParseTreeSymbolNodePrimary,
    ParseTreeSymbolTerminalExit,
    ParseTreeSymbolTerminalSemicolon,
    ParseTreeSymbolTerminalIntegerLiteral,
    ParseTreeSymbolTerminalEquals,
    ParseTreeSymbolTerminalI32S,
    ParseTreeSymbolTerminalF32S,
    ParseTreeSymbolTerminalBool,
    ParseTreeSymbolTerminalFloatLiteral,
    ParseTreeSymbolTerminalBooleanLiteral,
    ParseTreeSymbolTerminalIdentifier,
    ParseTreeSymbolTerminalFor,
    ParseTreeSymbolTerminalForIn,
    ParseTreeSymbolTerminalForTo,
    ParseTreeSymbolTerminalLeftCurlyBrace,
    ParseTreeSymbolTerminalRightCurlyBrace,
    ParseTreeSymbolTerminalPlus,
    ParseTreeSymbolTerminalMinus,
    ParseTreeSymbolTerminalLessThan,
    ParseTreeSymbolTerminalLessThanOrEqual,
    ParseTreeSymbolTerminalGreaterThan,
    ParseTreeSymbolTerminalGreaterThanOrEqual,
    ParseTreeSymbolTerminalEqualsEquals,
    ParseTreeSymbolTerminalNotEquals,
    ParseTreeSymbolTerminalLeftParen,
    ParseTreeSymbolTerminalRightParen,
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
    F32S,
    Bool,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i32),
    Float(f32),
    Bool(bool),
    Ident(String),
    BinaryOp {
        left: Box<Expr>,
        op: BinOpType,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum BinOpType {
    Add,
    Subtract,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

struct VarEntry {
    var_type: Type,
    var_value: Expr,
}

pub struct Parser {
    tokens: Vec<Token>,
    token_index: usize,
    scopes: Vec<HashMap<String, VarEntry>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            token_index: 0,
            scopes: vec![HashMap::new()],
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
                statement_node
                    .children
                    .push(self.parse_variable_declaration()?);
                Ok(statement_node)
            }
            TokenType::TokenTypeTypeF32S => {
                statement_node
                    .children
                    .push(self.parse_variable_declaration()?);
                Ok(statement_node)
            }
            TokenType::TokenTypeTypeBool => {
                statement_node
                    .children
                    .push(self.parse_variable_declaration()?);
                Ok(statement_node)
            }
            TokenType::TokenTypeIdentifier => {
                statement_node
                    .children
                    .push(self.parse_variable_assignment()?);
                Ok(statement_node)
            }
            TokenType::TokenTypeFor => {
                statement_node.children.push(self.parse_for()?);
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

        let semi_terminal = if self
            .current()
            .map_or(false, |t| t.token_type == TokenType::TokenTypeSemicolon)
        {
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
        let expr_content = self.parse_equality()?;
        Ok(ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolNodeExpression,
            children: vec![expr_content],
            value: None,
        })
    }

    // Equality → Comparison (("==" | "!=") Comparison)*
    fn parse_equality(&mut self) -> Result<ParseTreeNode, String> {
        let mut left = self.parse_comparison()?;

        while let Some(token) = self.current() {
            match token.token_type {
                TokenType::TokenTypeEqualsEquals | TokenType::TokenTypeNotEquals => {
                    let op_type = token.token_type;
                    let op_terminal = ParseTreeNode {
                        symbol: match op_type {
                            TokenType::TokenTypeEqualsEquals => {
                                ParseTreeSymbol::ParseTreeSymbolTerminalEqualsEquals
                            }
                            TokenType::TokenTypeNotEquals => {
                                ParseTreeSymbol::ParseTreeSymbolTerminalNotEquals
                            }
                            _ => unreachable!(),
                        },
                        children: Vec::new(),
                        value: None,
                    };
                    self.consume();

                    let right = self.parse_comparison()?;

                    left = ParseTreeNode {
                        symbol: ParseTreeSymbol::ParseTreeSymbolNodeEquality,
                        children: vec![left, op_terminal, right],
                        value: None,
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    // Comparison → Add (("<" | "<=" | ">" | ">=") Add)*
    fn parse_comparison(&mut self) -> Result<ParseTreeNode, String> {
        let mut left = self.parse_add()?;

        while let Some(token) = self.current() {
            match token.token_type {
                TokenType::TokenTypeLessThan
                | TokenType::TokenTypeLessThanOrEqual
                | TokenType::TokenTypeGreaterThan
                | TokenType::TokenTypeGreaterThanOrEqual => {
                    let op_type = token.token_type;
                    let op_terminal = ParseTreeNode {
                        symbol: match op_type {
                            TokenType::TokenTypeLessThan => {
                                ParseTreeSymbol::ParseTreeSymbolTerminalLessThan
                            }
                            TokenType::TokenTypeLessThanOrEqual => {
                                ParseTreeSymbol::ParseTreeSymbolTerminalLessThanOrEqual
                            }
                            TokenType::TokenTypeGreaterThan => {
                                ParseTreeSymbol::ParseTreeSymbolTerminalGreaterThan
                            }
                            TokenType::TokenTypeGreaterThanOrEqual => {
                                ParseTreeSymbol::ParseTreeSymbolTerminalGreaterThanOrEqual
                            }
                            _ => unreachable!(),
                        },
                        children: Vec::new(),
                        value: None,
                    };
                    self.consume();

                    let right = self.parse_add()?;

                    left = ParseTreeNode {
                        symbol: ParseTreeSymbol::ParseTreeSymbolNodeComparison,
                        children: vec![left, op_terminal, right],
                        value: None,
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    // Add → Primary (("+" | "-") Primary)*
    fn parse_add(&mut self) -> Result<ParseTreeNode, String> {
        let mut left = self.parse_primary()?;

        while let Some(token) = self.current() {
            match token.token_type {
                TokenType::TokenTypePlus | TokenType::TokenTypeMinus => {
                    let op_type = token.token_type;
                    let op_terminal = ParseTreeNode {
                        symbol: match op_type {
                            TokenType::TokenTypePlus => {
                                ParseTreeSymbol::ParseTreeSymbolTerminalPlus
                            }
                            TokenType::TokenTypeMinus => {
                                ParseTreeSymbol::ParseTreeSymbolTerminalMinus
                            }
                            _ => unreachable!(),
                        },
                        children: Vec::new(),
                        value: None,
                    };
                    self.consume();

                    let right = self.parse_primary()?;

                    left = ParseTreeNode {
                        symbol: ParseTreeSymbol::ParseTreeSymbolNodeAdd,
                        children: vec![left, op_terminal, right],
                        value: None,
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    // Primary → Int_Lit | Float_Lit | Bool_Lit | Ident | "(" Expr ")"
    fn parse_primary(&mut self) -> Result<ParseTreeNode, String> {
        let token = self
            .current()
            .ok_or("ParseError: Unexpected end of input in primary expression")?;

        match token.token_type {
            TokenType::TokenTypeIntegerLiteral => {
                let child = ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolTerminalIntegerLiteral,
                    children: Vec::new(),
                    value: token.value.clone(),
                };
                self.consume();
                Ok(ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolNodePrimary,
                    children: vec![child],
                    value: None,
                })
            }

            TokenType::TokenTypeFloatLiteral => {
                let child = ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolTerminalFloatLiteral,
                    children: Vec::new(),
                    value: token.value.clone(),
                };
                self.consume();
                Ok(ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolNodePrimary,
                    children: vec![child],
                    value: None,
                })
            }

            TokenType::TokenTypeBooleanLiteral => {
                let child = ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolTerminalBooleanLiteral,
                    children: Vec::new(),
                    value: token.value.clone(),
                };
                self.consume();
                Ok(ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolNodePrimary,
                    children: vec![child],
                    value: None,
                })
            }

            TokenType::TokenTypeIdentifier => {
                let child = ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolTerminalIdentifier,
                    children: Vec::new(),
                    value: token.value.clone(),
                };
                self.consume();
                Ok(ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolNodePrimary,
                    children: vec![child],
                    value: None,
                })
            }

            TokenType::TokenTypeLeftParen => {
                let left_paren = ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolTerminalLeftParen,
                    children: Vec::new(),
                    value: None,
                };
                self.consume();

                // Recursively parse the expression inside parentheses
                let expr_content = self.parse_equality()?;
                // Wrap it in an Expression node
                let expr = ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolNodeExpression,
                    children: vec![expr_content],
                    value: None,
                };

                let right_paren_token = self
                    .current()
                    .ok_or("ParseError: Expected ')', found end of input")?;
                if right_paren_token.token_type != TokenType::TokenTypeRightParen {
                    return Err(format!(
                        "ParseError: Expected ')', found {:?}",
                        right_paren_token.token_type
                    ));
                }
                let right_paren = ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolTerminalRightParen,
                    children: Vec::new(),
                    value: None,
                };
                self.consume();

                Ok(ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolNodePrimary,
                    children: vec![left_paren, expr, right_paren],
                    value: None,
                })
            }

            _ => Err(format!(
                "Unexpected token in primary expression: {:?}",
                token.token_type
            )),
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<ParseTreeNode, String> {
        let type_node = self.parse_type()?;

        let ident_terminal = self.parse_expression()?;

        let equals_token = self
            .current()
            .ok_or("ParseError: Expected '=', found end of input")?;
        if equals_token.token_type != TokenType::TokenTypeEquals {
            return Err(format!(
                "ParseError: Expected '=', found {:?}",
                equals_token.token_type
            ));
        }
        let equals_terminal = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalEquals,
            children: vec![],
            value: None,
        };
        self.consume();

        let expr_node = self.parse_expression()?;

        let semi_token = self
            .current()
            .ok_or("ParseError: Expected semicolon, found end of input")?;
        if semi_token.token_type != TokenType::TokenTypeSemicolon {
            return Err(format!(
                "ParseError: Expected semicolon, found {:?}",
                semi_token.token_type
            ));
        }
        let semi_terminal = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalSemicolon,
            children: vec![],
            value: None,
        };
        self.consume();

        let var_name = self
            .find_terminal(&ident_terminal)
            .value
            .as_ref()
            .expect("Identifier should have a value")
            .clone();
        
        let var_type = self.match_type_in_scope(&type_node);
        let var_value = self.match_expression_in_scope(&expr_node);
        self.insert_in_scope(
            var_name,
            VarEntry {
                var_type,
                var_value,
            },
        );

        Ok(ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolNodeVariableDeclaration,
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

    fn parse_variable_assignment(&mut self) -> Result<ParseTreeNode, String> {
        let ident_token = self
            .current()
            .ok_or("ParseError: Expected identifier, found end of input")?;
        if ident_token.token_type != TokenType::TokenTypeIdentifier {
            return Err(format!(
                "ParseError: Expected identifier, found {:?}",
                ident_token.token_type
            ));
        }
        let ident_terminal = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalIdentifier,
            children: vec![],
            value: ident_token.value.clone(),
        };
        self.consume();

        let equals_token = self
            .current()
            .ok_or("ParseError: Expected '=', found end of input")?;
        if equals_token.token_type != TokenType::TokenTypeEquals {
            return Err(format!(
                "ParseError: Expected '=', found {:?}",
                equals_token.token_type
            ));
        }
        let equals_terminal = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalEquals,
            children: vec![],
            value: None,
        };
        self.consume();

        let expr_node = self.parse_expression()?;

        let semi_token = self
            .current()
            .ok_or("ParseError: Expected semicolon, found end of input")?;
        if semi_token.token_type != TokenType::TokenTypeSemicolon {
            return Err(format!(
                "ParseError: Expected semicolon, found {:?}",
                semi_token.token_type
            ));
        }
        let semi_terminal = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalSemicolon,
            children: vec![],
            value: None,
        };
        self.consume();
        
        let var_name = ident_terminal
            .value
            .as_ref()
            .expect("Identifier should have a value")
            .clone();
        let var_value = self.match_expression_in_scope(&expr_node).clone();
        if self.lookup_in_scope(&var_name).is_none() {
            return Err(format!("Undefined variable {}", var_name));
        }
        self.update_in_scope(&var_name, var_value)?;

        Ok(ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolNodeVariableAssignment,
            children: vec![ident_terminal, equals_terminal, expr_node, semi_terminal],
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
        } else if self.current() != None
            && self.current().unwrap().token_type == TokenType::TokenTypeTypeF32S
        {
            let node = ParseTreeNode {
                symbol: ParseTreeSymbol::ParseTreeSymbolNodeType,
                children: vec![ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolTerminalF32S,
                    children: Vec::new(),
                    value: None,
                }],
                value: None,
            };
            self.consume();
            Ok(node)
        } else if self.current() != None
            && self.current().unwrap().token_type == TokenType::TokenTypeTypeBool
        {
            let node = ParseTreeNode {
                symbol: ParseTreeSymbol::ParseTreeSymbolNodeType,
                children: vec![ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolTerminalBool,
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

    fn parse_for(&mut self) -> Result<ParseTreeNode, String> {
        if self.current().unwrap().token_type != TokenType::TokenTypeFor {
            return Err(format!(
                "MissingTokenError: Expected 'for', found: {:?}",
                self.current().unwrap().token_type
            ));
        }
        let terminal_for = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalFor,
            children: vec![],
            value: None,
        };
        self.consume();

        let ident_node = self.parse_expression()?;

        if self.current().unwrap().token_type != TokenType::TokenTypeForIn {
            return Err(format!(
                "MissingTokenError: Expected 'for_in', found: {:?}",
                self.current().unwrap().token_type
            ));
        }
        let terminal_for_in = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalForIn,
            children: vec![],
            value: None,
        };
        self.consume();

        let lower_bound_node = self.parse_expression()?;

        if self.current().unwrap().token_type != TokenType::TokenTypeForTo {
            return Err(format!(
                "MissingTokenError: Expected 'for_dot', found: {:?}",
                self.current().unwrap().token_type
            ));
        }
        let terminal_for_dot = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalForTo,
            children: vec![],
            value: None,
        };
        self.consume();

        let upper_bound_node = self.parse_expression()?;

        if self.current().unwrap().token_type != TokenType::TokenTypeLeftCurlyBrace {
            return Err(format!(
                "MissingTokenError: Expected 'left_curly_brace', found: {:?}",
                self.current().unwrap().token_type
            ));
        }
        
        self.push_scope();

        // push iterator while inside the new scope
        let var_name = self
            .find_terminal(&ident_node)
            .value
            .as_ref()
            .expect("Identifier should have a value")
            .clone();

        let var_type = Type::I32S;
        let var_value = self.match_expression_in_scope(&lower_bound_node);
        self.insert_in_scope(
            var_name,
            VarEntry {
                var_type,
                var_value,
            },
        );
        
        let block_node = self.parse_block()?;
        self.pop_scope();
        
        Ok(ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolNodeFor,
            children: vec![
                terminal_for,
                ident_node,
                terminal_for_in,
                lower_bound_node,
                terminal_for_dot,
                upper_bound_node,
                block_node,
            ],
            value: None,
        })
    }
    
    fn parse_block(&mut self) -> Result<ParseTreeNode, String> {
        if self.current().unwrap().token_type != TokenType::TokenTypeLeftCurlyBrace {
            return Err(format!(
                "MissingTokenError: Expected 'left_curly_brace', found: {:?}",
                self.current().unwrap().token_type
            ));
        }
        let left_bracket_terminal = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalLeftCurlyBrace,
            children: vec![],
            value: None,
        };
        self.consume();
        
        let statement_node = self.parse_statement()?;

        if self.current().unwrap().token_type != TokenType::TokenTypeRightCurlyBrace {
            return Err(format!(
                "MissingTokenError: Expected 'right_curly_brace', found: {:?}",
                self.current().unwrap().token_type
            ));
        }
        let right_bracket_terminal = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalRightCurlyBrace,
            children: vec![],
            value: None,
        };
        self.consume();
        
        Ok(ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolNodeBlock,
            children: vec![
                left_bracket_terminal,
                statement_node,
                right_bracket_terminal,
            ],
            value: None,
        })
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

    pub fn build_ast(&mut self, parse_tree: &ParseTreeNode) -> AbstractSyntaxTreeNode {
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
                    let value_child_node = self.find_terminal(&expr_node);
                    let expr = match value_child_node.symbol {
                        ParseTreeSymbol::ParseTreeSymbolTerminalIntegerLiteral => {
                            let v = value_child_node
                                .value
                                .as_ref()
                                .unwrap()
                                .parse::<i32>()
                                .unwrap();
                            Expr::Int(v)
                        }
                        ParseTreeSymbol::ParseTreeSymbolTerminalFloatLiteral => {
                            let v = value_child_node
                                .value
                                .as_ref()
                                .unwrap()
                                .parse::<f32>()
                                .unwrap();
                            Expr::Float(v)
                        }
                        ParseTreeSymbol::ParseTreeSymbolTerminalBooleanLiteral => {
                            let v = value_child_node
                                .value
                                .as_ref()
                                .unwrap()
                                .parse::<bool>()
                                .unwrap();
                            Expr::Bool(v)
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
                    panic!("Exit statement has no expression child");
                }
            }

            ParseTreeSymbol::ParseTreeSymbolNodeVariableDeclaration => {
                if let Some(node_expr) = parse_tree
                    .children
                    .iter()
                    .find(|c| c.symbol == ParseTreeSymbol::ParseTreeSymbolNodeExpression)
                {
                    let terminal_id_node = self.find_terminal(node_expr);
                    
                    let name = terminal_id_node.value.as_ref().expect("Missing terminal");
                    let entry = self.lookup_in_scope(name).unwrap();

                    AbstractSyntaxTreeNode {
                        symbol: AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolVariableDeclaration {
                            name: name.to_string(),
                            type_: entry.var_type.clone(),
                            value: entry.var_value.clone(),
                        },
                        children: Vec::new(),
                    }
                } else {
                    panic!("Variable declaration node has no expression child");
                }
            }

            ParseTreeSymbol::ParseTreeSymbolNodeVariableAssignment => {
                if let Some(terminal_id_node) = parse_tree
                    .children
                    .iter()
                    .find(|c| c.symbol == ParseTreeSymbol::ParseTreeSymbolTerminalIdentifier)
                {
                    let name = terminal_id_node.value.as_ref().expect("Missing terminal");
                    let entry = self.lookup_in_scope(name).unwrap();

                    AbstractSyntaxTreeNode {
                        symbol:
                            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolVariableAssignment {
                                name: name.to_string(),
                                value: entry.var_value.clone(),
                            },
                        children: Vec::new(),
                    }
                } else {
                    panic!("Variable node has no terminal identifier");
                }
            }

            ParseTreeSymbol::ParseTreeSymbolNodeFor => {
                let mut expr_nodes = parse_tree
                    .children
                    .iter()
                    .filter(|c| c.symbol == ParseTreeSymbol::ParseTreeSymbolNodeExpression);

                let id_expr = expr_nodes.next().expect("Missing iterator expression");
                let begin_expr = expr_nodes.next().expect("Missing begin expression");
                let end_expr = expr_nodes.next().expect("Missing end expression");

                let iterator_name = self.find_terminal(&id_expr)                    
                    .value
                    .as_ref()
                    .unwrap()
                    .clone();

                let iterator_begin = {
                    let lit = self.find_terminal(&begin_expr);
                    Expr::Int(lit.value.as_ref().unwrap().parse().unwrap())
                };

                let iterator_end = {
                    let lit = self.find_terminal(&end_expr);
                    Expr::Int(lit.value.as_ref().unwrap().parse().unwrap())
                };

                let mut stmt_nodes = Vec::new();
                self.find_statements(parse_tree, &mut stmt_nodes);

                let body: Vec<AbstractSyntaxTreeNode> =
                    stmt_nodes.into_iter()
                        .map(|stmt| self.build_ast(stmt))
                        .collect();

                AbstractSyntaxTreeNode {
                    symbol: AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolFor {
                        iterator_name,
                        iterator_begin,
                        iterator_end,
                        body,
                    },
                    children: vec![],
                }
            }

            _ => {
                panic!("Unexpected parse tree node: {:?}", parse_tree.symbol);
            }
        }
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn lookup_in_scope(&self, name: &str) -> Option<&VarEntry> {
        for scope in self.scopes.iter().rev() {
            if let Some(v) = scope.get(name) {
                return Some(v);
            }
        }
        None
    }

    fn insert_in_scope(&mut self, name: String, entry: VarEntry) {
        self.scopes.last_mut().unwrap().insert(name, entry);
    }

    fn update_in_scope(&mut self, name: &str, value: Expr) -> Result<(), String> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(var) = scope.get_mut(name) {
                var.var_value = value;
                return Ok(());
            }
        }
        Err(format!("Undefined variable {}", name))
    }

    fn match_type_in_scope(&mut self, node: &ParseTreeNode) -> Type {
        match node.children.first().unwrap().symbol {
            ParseTreeSymbol::ParseTreeSymbolTerminalI32S => Type::I32S,
            ParseTreeSymbol::ParseTreeSymbolTerminalF32S => Type::F32S,
            ParseTreeSymbol::ParseTreeSymbolTerminalBool => Type::Bool,
            _ => panic!("Unsupported type node"),
        }
    }

    fn match_expression_in_scope(&mut self, node: &ParseTreeNode) -> Expr {
        let child = self.find_terminal(node);
        match child.symbol {
            ParseTreeSymbol::ParseTreeSymbolTerminalIntegerLiteral => {
                let value = child.value.as_ref().unwrap().parse::<i32>().unwrap();
                Expr::Int(value)
            }
            ParseTreeSymbol::ParseTreeSymbolTerminalFloatLiteral => {
                let value = child.value.as_ref().unwrap().parse::<f32>().unwrap();
                Expr::Float(value)
            }
            ParseTreeSymbol::ParseTreeSymbolTerminalBooleanLiteral => {
                let value = child.value.as_ref().unwrap().parse::<bool>().unwrap();
                Expr::Bool(value)
            }
            ParseTreeSymbol::ParseTreeSymbolTerminalIdentifier => {
                let ident = child.value.as_ref().unwrap().clone();
                if self.lookup_in_scope(&ident).is_none() {
                    panic!("Undefined identifier {}", ident);
                }
                Expr::Ident(ident)
            }
            _ => panic!("Unsupported expression type"),
        }
    }
    
    fn find_terminal<'a>(&mut self, node: &'a ParseTreeNode) -> &'a ParseTreeNode {
        match node.symbol {
            ParseTreeSymbol::ParseTreeSymbolTerminalIntegerLiteral |
            ParseTreeSymbol::ParseTreeSymbolTerminalFloatLiteral |
            ParseTreeSymbol::ParseTreeSymbolTerminalBooleanLiteral |
            ParseTreeSymbol::ParseTreeSymbolTerminalIdentifier => return node,

            _ => {}
        }

        for child in &node.children {
            let result = self.find_terminal(child);
            return result; 
        }

        panic!("No terminal node found in subtree");
    }

    fn find_statements<'a>(&self, node: &'a ParseTreeNode, out: &mut Vec<&'a ParseTreeNode>) {
        if node.symbol == ParseTreeSymbol::ParseTreeSymbolNodeStatement {
            out.push(node);
        }
        for child in &node.children {
            self.find_statements(child, out);
        }
    }
}
