# Noble Formal Grammar
***

|               |    |                                      |
|--------------:|:--:|-------------------------------------:|
| *Entry Point* | ⟶  |                              [Stmt]* |
|        [Stmt] | ⟶  | [Exit], [VariableDec], [VariableAsm] |
| [VariableDec] | ⟶  |        [Type] [Ident] *=* [Expr] *;* |
| [VariableAsm] | ⟶  |               [Ident] *=* [Expr] *;* |
|        [Type] | ⟶  |                               *i32s* |
|       [Ident] | ⟶  |         **user-defined non-keyword** |
|        [Exit] | ⟶  |                    *exit* [Expr] *;* |
|        [Expr] | ⟶  |                   [Int_Lit], [Ident] |