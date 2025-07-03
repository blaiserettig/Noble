use std::io::Write;
use std::fs::File;
use std::io::BufWriter;
use crate::parse::{AbstractSyntaxTreeNode, AbstractSyntaxTreeSymbol};

pub struct Generator {

}

impl Generator {
    pub fn new() -> Self {
        Self {

        }
    }
    
    pub fn generate_boilerplate(&mut self, writer: &mut BufWriter<&File>) {
        write!(writer, "{}", "bits 64\ndefault rel\n\nsegment .text\nglobal mainCRTStartup\n\nmainCRTStartup:\n").expect("Unable to write to file.");
    }

    pub fn generate_x64(&mut self, ast_root: &AbstractSyntaxTreeNode, writer: &mut BufWriter<&File>) {
        match ast_root.symbol {
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolEntry => {
                ast_root.children.iter().map(|child| self.generate_x64(child, writer)).for_each(drop);
            }
            
            AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolExit(code) => {
                writeln!(writer, "    mov eax, {}", code).unwrap();
                writeln!(writer, "    ret").unwrap();
            }
        }
    }
}