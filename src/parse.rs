use crate::tokenize::Token;

pub enum GrammarSymbols {
    GrammarSymbolEntryPoint,
    GrammarSymbolStatement,
    GrammarSymbolExpression,
    GrammarSymbolExit,
    GrammarSymbolSemicolon,
}

struct ParseTreeNode {
    symbol: GrammarSymbols,
    children: Vec<ParseTreeNode>,
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            index: 0,
        }
    }

    // Assume the tokens are given to us starting from the entry point
    pub fn parse(&mut self) -> Option<ParseTreeNode> {
        let mut entry_node: ParseTreeNode = ParseTreeNode { symbol: GrammarSymbols::GrammarSymbolEntryPoint, children: Vec::new() };

        while !self.is_at_end() {
            
        }

        return if entry_node.children.is_empty() {
            None
        } else {
            Some(entry_node)
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.index >= self.tokens.len()
    }

    pub fn current(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }
}