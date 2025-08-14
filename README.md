
## Noble Programming Language

[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)


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
exit y;             // Exit program with return code
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

## Implementation Details

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
i32s x = 1;
i32s y = x;
exit y;
```

**Generated Assembly** (`out.asm`):
```asm
bits 64
default rel

segment .text
global mainCRTStartup

mainCRTStartup:
    mov dword [x], 1
    mov eax, dword [x]
    mov dword [y], eax
    mov eax, dword [y]
    ret

segment .bss
y resd 1
x resd 1
```

**Intermediate Steps** (Tokenization):
```tokens
Token { token_type: TokenTypeEntryPoint, value: None }
Token { token_type: TokenTypeTypeI32S, value: None }
Token { token_type: TokenTypeIdentifier, value: Some("x") }
Token { token_type: TokenTypeEquals, value: None }
Token { token_type: TokenTypeIntegerLiteral, value: Some("1") }
Token { token_type: TokenTypeSemicolon, value: None }
Token { token_type: TokenTypeTypeI32S, value: None }
Token { token_type: TokenTypeIdentifier, value: Some("y") }
Token { token_type: TokenTypeEquals, value: None }
Token { token_type: TokenTypeIdentifier, value: Some("x") }
Token { token_type: TokenTypeSemicolon, value: None }
Token { token_type: TokenTypeExit, value: None }
Token { token_type: TokenTypeIdentifier, value: Some("y") }
Token { token_type: TokenTypeSemicolon, value: None }
```

**Intermediate Steps** (Parsing):
```parse tree
ParseTreeSymbolNodeEntryPoint
None
    ParseTreeSymbolNodeStatement
    None
        ParseTreeSymbolNodeVariable
        None
            ParseTreeSymbolNodeType
            None
                ParseTreeSymbolTerminalI32S
                None
            ParseTreeSymbolTerminalIdentifier
            Some("x")
            ParseTreeSymbolTerminalEquals
            None
            ParseTreeSymbolNodeExpression
            None
                ParseTreeSymbolTerminalIntegerLiteral
                Some("1")
            ParseTreeSymbolTerminalSemicolon
            None
    ParseTreeSymbolNodeStatement
    None
        ParseTreeSymbolNodeVariable
        None
            ParseTreeSymbolNodeType
            None
                ParseTreeSymbolTerminalI32S
                None
            ParseTreeSymbolTerminalIdentifier
            Some("y")
            ParseTreeSymbolTerminalEquals
            None
            ParseTreeSymbolNodeExpression
            None
                ParseTreeSymbolTerminalIdentifier
                Some("x")
            ParseTreeSymbolTerminalSemicolon
            None
    ParseTreeSymbolNodeStatement
    None
        ParseTreeSymbolNodeExit
        None
            ParseTreeSymbolTerminalExit
            None
            ParseTreeSymbolNodeExpression
            None
                ParseTreeSymbolTerminalIdentifier
                Some("y")
            ParseTreeSymbolTerminalSemicolon
            None
```

**Intermediate Steps** (Abstract Syntax Tree):
```ast
AbstractSyntaxTreeSymbolEntry
  AbstractSyntaxTreeSymbolVariableDeclaration { name: "x", type_: I32S, value: Int(1) }
  AbstractSyntaxTreeSymbolVariableDeclaration { name: "y", type_: I32S, value: Ident("x") }
  AbstractSyntaxTreeSymbolExit(Ident("y"))
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
- [Compiler Construction CSU Sacramento Lecture Series](https://www.youtube.com/@ghassanshobakicomputerscie9478/playlists)

## License

MIT License - see [LICENSE](LICENSE) for details.
