# Noble Formal Grammar
***

|               |   |                               |
|--------------:|:-:|------------------------------:|
| *Entry Point* | ⟶ |                       [Stmt]* |
|        [Stmt] | ⟶ |            [Exit], [Variable] |
|    [Variable] | ⟶ | [Type] [Ident] *=* [Expr] *;* |
|        [Type] | ⟶ |                        *i32s* |
|       [Ident] | ⟶ |  **user-defined non-keyword** |
|        [Exit] | ⟶ |             *exit* [Expr] *;* |
|        [Expr] | ⟶ |            [Int_Lit], [Ident] |