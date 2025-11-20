# Noble Formal Grammar
***
```
"Entry Point"   → Stmt*
Stmt            → Exit | VariableDec | VariableAsm | For | If
VariableDec     → Type Ident "=" Expr ";"
VariableAsm     → Ident "=" Expr ";"
For             → "for" Ident "in" Int_Lit "to" Int_Lit Block
If              → "if" Expr Block Else
Else            → "else" If | else" Block | ε
Block           → "{" Stmt* "}"
Type            → i32s | f32s | bool
Ident           → *user-defined non-keyword*
Exit            → "exit" Expr ";"
Expr            → Equality
Equality        → Comparison (("==" | "!=") Comparison)*
Comparison      → Add (("<" | "<=" | ">" | ">=") Add)*
Add             → Primary (("+" | "-") Primary)*
Primary         → Int_Lit | Float_Lit | Bool_Lit | Ident | "(" Expr ")"
Int_Lit         → *integer literal*
Int_Lit         → *floating point literal*
Int_Lit         → *boolean point literal*
```