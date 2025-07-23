# Program
Status: 🟢

```rustleaf
// Test expressions in collections
var list_with_expr = [1 + 2, 3 * 4];

// Test expressions in dict
var dict_with_expr = {"sum": 1 + 2, "product": 3 * 4};

// Test variable in collections
var x = 5;
var list_with_var = [x, x + 1];
```

# Output
None

# Result
```rust
Ok(
    Unit,
)
```

# Lex
```rust
Ok(
    [
        Token(Var),
        Token(Ident, "list_with_expr"),
        Token(Equal),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(Plus),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(Star),
        Token(Int, "4"),
        Token(RightBracket),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "dict_with_expr"),
        Token(Equal),
        Token(LeftBrace),
        Token(String, "sum"),
        Token(Colon),
        Token(Int, "1"),
        Token(Plus),
        Token(Int, "2"),
        Token(Comma),
        Token(String, "product"),
        Token(Colon),
        Token(Int, "3"),
        Token(Star),
        Token(Int, "4"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "5"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "list_with_var"),
        Token(Equal),
        Token(LeftBracket),
        Token(Ident, "x"),
        Token(Comma),
        Token(Ident, "x"),
        Token(Plus),
        Token(Int, "1"),
        Token(RightBracket),
        Token(Semicolon),
        Token(Eof),
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            VarDecl {
                pattern: Variable(
                    "list_with_expr",
                ),
                value: Some(
                    List(
                        [
                            Add(
                                Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                                Literal(
                                    Int(
                                        2,
                                    ),
                                ),
                            ),
                            Mul(
                                Literal(
                                    Int(
                                        3,
                                    ),
                                ),
                                Literal(
                                    Int(
                                        4,
                                    ),
                                ),
                            ),
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "dict_with_expr",
                ),
                value: Some(
                    Dict(
                        [
                            (
                                Literal(
                                    String(
                                        "sum",
                                    ),
                                ),
                                Add(
                                    Literal(
                                        Int(
                                            1,
                                        ),
                                    ),
                                    Literal(
                                        Int(
                                            2,
                                        ),
                                    ),
                                ),
                            ),
                            (
                                Literal(
                                    String(
                                        "product",
                                    ),
                                ),
                                Mul(
                                    Literal(
                                        Int(
                                            3,
                                        ),
                                    ),
                                    Literal(
                                        Int(
                                            4,
                                        ),
                                    ),
                                ),
                            ),
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "x",
                ),
                value: Some(
                    Literal(
                        Int(
                            5,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "list_with_var",
                ),
                value: Some(
                    List(
                        [
                            Identifier(
                                "x",
                            ),
                            Add(
                                Identifier(
                                    "x",
                                ),
                                Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                            ),
                        ],
                    ),
                ),
            },
        ],
    ),
)
```

# Eval
```rust
Ok(
    Block(
        [
            Declare(
                "list_with_expr",
                Some(
                    List(
                        [
                            Call(
                                GetAttr(
                                    Literal(
                                        Int(
                                            1,
                                        ),
                                    ),
                                    "op_add",
                                ),
                                [
                                    Literal(
                                        Int(
                                            2,
                                        ),
                                    ),
                                ],
                            ),
                            Call(
                                GetAttr(
                                    Literal(
                                        Int(
                                            3,
                                        ),
                                    ),
                                    "op_mul",
                                ),
                                [
                                    Literal(
                                        Int(
                                            4,
                                        ),
                                    ),
                                ],
                            ),
                        ],
                    ),
                ),
            ),
            Declare(
                "dict_with_expr",
                Some(
                    Dict(
                        [
                            (
                                Literal(
                                    String(
                                        "sum",
                                    ),
                                ),
                                Call(
                                    GetAttr(
                                        Literal(
                                            Int(
                                                1,
                                            ),
                                        ),
                                        "op_add",
                                    ),
                                    [
                                        Literal(
                                            Int(
                                                2,
                                            ),
                                        ),
                                    ],
                                ),
                            ),
                            (
                                Literal(
                                    String(
                                        "product",
                                    ),
                                ),
                                Call(
                                    GetAttr(
                                        Literal(
                                            Int(
                                                3,
                                            ),
                                        ),
                                        "op_mul",
                                    ),
                                    [
                                        Literal(
                                            Int(
                                                4,
                                            ),
                                        ),
                                    ],
                                ),
                            ),
                        ],
                    ),
                ),
            ),
            Declare(
                "x",
                Some(
                    Literal(
                        Int(
                            5,
                        ),
                    ),
                ),
            ),
            Declare(
                "list_with_var",
                Some(
                    List(
                        [
                            Variable(
                                "x",
                            ),
                            Call(
                                GetAttr(
                                    Variable(
                                        "x",
                                    ),
                                    "op_add",
                                ),
                                [
                                    Literal(
                                        Int(
                                            1,
                                        ),
                                    ),
                                ],
                            ),
                        ],
                    ),
                ),
            ),
        ],
        None,
    ),
)
```