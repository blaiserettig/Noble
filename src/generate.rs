use std::collections::HashSet;
use std::io::Write;
use std::fs::File;
use std::io::BufWriter;
use crate::parse::{AbstractSyntaxTreeNode, AbstractSyntaxTreeSymbol, Expr};

pub struct Generator {
    declared_vars: HashSet<String>,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            declared_vars: HashSet::new(),
        }
    }
    
    pub fn generate_boilerplate(&mut self, writer: &mut BufWriter<&File>) {
        write!(writer, "{}", "bits 64\ndefault rel\n\nsegment .text\nglobal mainCRTStartup\n\nmainCRTStartup:\n").expect("Unable to write to file.");
    }

    pub fn generate_x64(&mut self, ast_root: &AbstractSyntaxTreeNode, writer: &mut BufWriter<&File>) {
        match &ast_root.symbol {
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolEntry => {
                ast_root.children.iter().map(|child| self.generate_x64(child, writer)).for_each(drop);

                writeln!(writer, "    ret").unwrap();

                if !self.declared_vars.is_empty() {
                    writeln!(writer, "\nsegment .bss").unwrap();
                    for var in &self.declared_vars {
                        writeln!(writer, "{} resd 1", var).unwrap();
                    }
                }
            }
            
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolExit(expr) => {
                match expr {
                    Expr::Int(i) => {
                        writeln!(writer, "    mov eax, {}", i).unwrap();
                    }
                    Expr::Ident(j) => {
                        writeln!(writer, "    mov eax, dword [{}]", j).expect("Idek");
                    }
                }
            }

            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolVariableDeclaration { name, type_: _type_, value } => {
                self.declared_vars.insert(name.clone());
                match value {
                    Expr::Int(i) => {
                        writeln!(writer, "    mov dword [{}], {}", name, i).unwrap();
                    }
                    Expr::Ident(ident) => {
                        writeln!(writer, "    mov eax, dword [{}]", ident).unwrap();
                        writeln!(writer, "    mov dword [{}], eax", name).unwrap();
                    }
                }
            }
        }
    }
}