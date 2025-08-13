# Noble Programming Language

A minimal, educational programming language implemented in Rust that compiles to x86-64 assembly. Noble demonstrates the complete pipeline of language implementation, from lexical analysis to code generation.

## Features

- **Complete Compilation Pipeline**: Lexing → Parsing → AST Generation → x86-64 Code Generation
- **Type System**: Currently supports signed 32-bit integers (`i32s`)
- **Variable Declaration and Assignment**: Store and retrieve values
- **Program Exit with Return Values**: Control program termination
- **Cross-Platform Assembly Output**: Generates NASM-compatible x86-64 assembly
- **Comprehensive Error Handling**: Detailed error messages for parsing failures
- **Symbol Table Management**: Tracks variable declarations and types

## Language Syntax

Noble follows a simple, C-like syntax:

```noble
i32s x = 42;        // Variable declaration and initialization
i32s y = x;         // Variable assignment from another variable
exit 0;             // Exit program with return code
```

### Grammar

```
Entry Point → [Stmt]*
[Stmt]      → [Exit] | [Variable]
[Variable]  → [Type] [Ident] = [Expr] ;
[Type]      → i32s
[Ident]     → user-defined non-keyword
[Exit]      → exit [Expr] ;
[Expr]      → [Int_Lit] | [Ident]
```

## Architecture

### Compilation Pipeline

```
Source Code (.nbl)
       ↓
   Tokenizer (Lexer)
       ↓
   Parse Tree
       ↓
Abstract Syntax Tree
       ↓
   Code Generator
       ↓
x86-64 Assembly (.asm)
```

### Module Structure

- **`tokenize.rs`** - Lexical analysis and token generation
- **`parse.rs`** - Parsing, AST construction, and symbol table management  
- **`generate.rs`** - x86-64 assembly code generation
- **`main.rs`** - CLI interface and pipeline orchestration

## 🔧 Implementation Details

### Tokenizer
- **Character-by-character parsing** with lookahead support
- **Keyword recognition**: `exit`, `i32s`
- **Token types**: Identifiers, integer literals, operators, punctuation
- **Error handling**: Graceful failure on unrecognized characters

### Parser
- **Recursive descent parser** following the formal grammar
- **Two-phase approach**: Parse tree construction followed by AST generation
- **Symbol table**: HashMap-based variable tracking with type information
- **Error recovery**: Detailed error messages with token context

### Code Generator
- **x86-64 assembly generation** using NASM syntax
- **Memory management**: Automatic `.bss` segment generation for variables
- **Register allocation**: Strategic use of EAX register for operations
- **Boilerplate generation**: Windows-compatible entry point setup

### Data Structures

```rust
// Token representation
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<String>,
}

// AST node structure
pub struct AbstractSyntaxTreeNode {
    pub symbol: AbstractSyntaxTreeSymbol,
    pub children: Vec<AbstractSyntaxTreeNode>,
}

// Expression types
pub enum Expr {
    Int(i32),
    Ident(String),
}
```

## Getting Started

### Prerequisites
- Rust (latest stable version)
- NASM assembler (for assembling output)
- MSFT VS Linker (for creating executables)

### Installation

```bash
git clone https://github.com/blaiserettig/Noble
cd Noble
cargo build --release
```

### Usage

1. **Write a Noble program** (`example.nbl`):
```noble
i32s result = 42;
exit result;
```

2. **Compile to assembly**:
```bash
./target/release/noble example.nbl
```

3. **Assemble and link** (Windows):
```bash
nasm -f win64 src/out.asm -o out.obj
link out.obj /subsystem:console /entry:mainCRTStartup
```

## Example Compilation

**Input** (`input.nbl`):
```noble
i32s x = 0;
exit x;
```

**Generated Assembly** (`out.asm`):
```asm
bits 64
default rel

segment .text
global mainCRTStartup

mainCRTStartup:
    mov dword [x], 0
    mov eax, dword [x]
    ret

segment .bss
x resd 1
```

## Technical Highlights

- **Memory-Safe Implementation**: Written in Rust with comprehensive error handling
- **Formal Grammar**: Well-defined BNF grammar specification
- **Parse Tree Visualization**: Debug output for understanding parsing process
- **AST Transformation**: Clean separation between concrete and abstract syntax
- **Symbol Table**: Proper variable scoping and type checking foundation
- **Modular Design**: Clean separation of concerns across compilation phases

## Roadmap

### Short Term
- [ ] More primitive types (`f32`, `bool`, `char`)
- [ ] Arithmetic expressions (`+`, `-`, `*`, `/`)
- [ ] Boolean type and logical operations
- [ ] Comparison operators (`==`, `!=`, `<`, `>`)

### Medium Term  
- [ ] Arrays and basic data structures
- [ ] String literals and manipulation
- [ ] Conditional statements (`if`/`else`)
- [ ] Loops (`while`, `for`)

### Long Term
- [ ] Functions and procedure calls
- [ ] Structs and user-defined types
- [ ] Standard library functions
- [ ] Optimization passes
- [ ] LLVM backend integration

## Educational Value

This project demonstrates:
- **Compiler theory fundamentals**
- **Rust systems programming**
- **Assembly language generation**
- **Formal language design**
- **Error handling strategies**
- **Modular software architecture**

## Contributing

Contributions are welcome! Areas of interest:
- New language features
- Optimization improvements  
- Better error messages
- Additional target architectures
- Documentation

## References

- [Crafting Interpreters](https://craftinginterpreters.com/)
- [Engineering a Compiler](https://www.elsevier.com/books/engineering-a-compiler/cooper/978-0-12-815412-0)
- [x86-64 Assembly Reference](https://www.nasm.us/xdoc/2.15.05/html/nasmdoc0.html)
- [Compiler Construction CSU Sacramento Lecture Series]([https://www.nasm.us/xdoc/2.15.05/html/nasmdoc0.html](https://www.youtube.com/watch?v=W9B98S2mGGE&list=PL6KMWPQP_DM97Hh0PYNgJord-sANFTI3i))

## License

MIT License - see [LICENSE](LICENSE) for details.

---

*Noble: A small language with big aspirations. Perfect for learning the fundamentals of programming language implementation.*
