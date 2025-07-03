# Noble Formal Grammar
***

|               |   |                   |
|--------------:|:-:|------------------:|
| *Entry Point* | ⟶ |            [Stmt] |
|        [Stmt] | ⟶ |            [Exit] |
|        [Exit] | ⟶ | *exit* [Expr] *;* |
|        [Expr] | ⟶ |         [Int_Lit] |
