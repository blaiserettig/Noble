# Noble Formal Grammar
***

|               |    |                                                            |
|--------------:|:--:|-----------------------------------------------------------:|
| *Entry Point* | ⟶  |                                                    [Stmt]* |
|        [Stmt] | ⟶  |                [Exit], [VariableDec], [VariableAsm], [For] |
| [VariableDec] | ⟶  |                              [Type] [Ident] *=* [Expr] *;* |
| [VariableAsm] | ⟶  |                                     [Ident] *=* [Expr] *;* |
|         [For] | ⟶  | *for* [Ident] *in* [Int_Lit] *..* [Int_Lit] *{* [Stmt] *}* |
|        [Type] | ⟶  |                                                     *i32s* |
|       [Ident] | ⟶  |                               **user-defined non-keyword** |
|        [Exit] | ⟶  |                                          *exit* [Expr] *;* |
|        [Expr] | ⟶  |                                         [Int_Lit], [Ident] |
|     [Int_Lit] | ⟶  |                                        **integer literal** |
