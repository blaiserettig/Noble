# Noble Formal Grammar
***

|               |    |                                                            |
|--------------:|:--:|-----------------------------------------------------------:|
| *Entry Point* | ⟶  |                                                    [Stmt]* |
|        [Stmt] | ⟶  |                [Exit], [VariableDec], [VariableAsm], [For] |
| [VariableDec] | ⟶  |                              [Type] [Ident] *=* [Expr] *;* |
| [VariableAsm] | ⟶  |                                     [Ident] *=* [Expr] *;* |
|         [For] | ⟶  | *for* [Ident] *in* [Int_Lit] *to* [Int_Lit] *{* [Stmt] *}* |
|        [Type] | ⟶  |                                     *i32s*, *f32s*, *bool* |
|       [Ident] | ⟶  |                               **user-defined non-keyword** |
|        [Exit] | ⟶  |                                          *exit* [Expr] *;* |
|        [Expr] | ⟶  |                [Int_Lit], [Float_Lit], [Bool_Lit], [Ident] |
|     [Int_Lit] | ⟶  |                                        **integer literal** |
|   [Float_Lit] | ⟶  |                                 **floating point literal** |
|    [Bool_Lit] | ⟶  |                                        **boolean literal** |

