use crate::parse::{AbstractSyntaxTreeNode, AbstractSyntaxTreeSymbol, Expr};
use std::collections::HashSet;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

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
        write!(
            writer,
            "{}",
            "bits 64\ndefault rel\n\nsegment .text\nglobal mainCRTStartup\n\nmainCRTStartup:\n"
        )
        .expect("Unable to write to file.");
    }

    pub fn generate_x64(
        &mut self,
        ast_root: &AbstractSyntaxTreeNode,
        writer: &mut BufWriter<&File>,
    ) {
        match &ast_root.symbol {
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolEntry => {
                ast_root
                    .children
                    .iter()
                    .map(|child| self.generate_x64(child, writer))
                    .for_each(drop);

                writeln!(writer, "    ret").unwrap();

                if !self.declared_vars.is_empty() {
                    writeln!(writer, "\nsegment .bss").unwrap();
                    for var in &self.declared_vars {
                        writeln!(writer, "{} resd 1", var).unwrap();
                    }
                }
            }

            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolExit(expr) => match expr {
                Expr::Int(i) => {
                    writeln!(writer, "    mov eax, {}", i).unwrap();
                }
                Expr::Ident(j) => {
                    writeln!(writer, "    mov eax, dword [{}]", j).expect("Idek");
                }
                Expr::Float(f) => {
                    let bits = f.to_bits();
                    writeln!(writer, "    mov eax, {}", bits).unwrap();
                }
            },

            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolVariableDeclaration {
                name,
                type_: _type_,
                value,
            } => {
                self.declared_vars.insert(name.clone());
                self.match_variable_helper(name, value, writer);
            }

            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolVariableAssignment {
                name,
                value,
            } => {
                self.match_variable_helper(name, value, writer);
            }

            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolFor {
                iterator_name,
                iterator_begin,
                iterator_end,
                body,
            } => {
                self.declared_vars.insert(iterator_name.clone());

                let loop_label = format!("loop_begin_{}", iterator_name);
                let end_label = format!("loop_end_{}", iterator_name);

                self.generate_expr_into_register(iterator_begin, "eax", writer);
                writeln!(writer, "    mov dword [{}], eax", iterator_name).unwrap();

                writeln!(writer, "{}:", loop_label).unwrap();

                writeln!(writer, "    mov eax, dword [{}]", iterator_name).unwrap();
                self.generate_expr_into_register(iterator_end, "ebx", writer);
                writeln!(writer, "    cmp eax, ebx").unwrap();
                writeln!(writer, "    jg {}", end_label).unwrap();

                for stmt in body {
                    self.generate_x64(stmt, writer);
                }

                writeln!(writer, "    mov eax, dword [{}]", iterator_name).unwrap();
                writeln!(writer, "    inc eax").unwrap();
                writeln!(writer, "    mov dword [{}], eax", iterator_name).unwrap();

                writeln!(writer, "    jmp {}", loop_label).unwrap();

                writeln!(writer, "{}:", end_label).unwrap();
            }
        }
    }

    fn match_variable_helper(
        &mut self,
        name: &String,
        value: &Expr,
        writer: &mut BufWriter<&File>,
    ) {
        match value {
            Expr::Int(i) => {
                writeln!(writer, "    mov dword [{}], {}", name, i).unwrap();
            }
            Expr::Ident(ident) => {
                writeln!(writer, "    mov eax, dword [{}]", ident).unwrap();
                writeln!(writer, "    mov dword [{}], eax", name).unwrap();
            }
            Expr::Float(f) => {
                let bits = f.to_bits();
                writeln!(writer, "    mov dword [{}], {}", name, bits).unwrap();
            }
        }
    }

    fn generate_expr_into_register(
        &mut self,
        expr: &Expr,
        reg: &str,
        writer: &mut BufWriter<&File>,
    ) {
        match expr {
            Expr::Int(i) => {
                writeln!(writer, "    mov {}, {}", reg, i).unwrap();
            }
            Expr::Ident(name) => {
                writeln!(writer, "    mov {}, dword [{}]", reg, name).unwrap();
            }
            Expr::Float(f) => {
                let bits = f.to_bits();
                writeln!(writer, "    mov {}, {}", reg, bits).unwrap();
            }
        }
    }
}
