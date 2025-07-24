# Program
Status: 🟢
Assertions: 1

```rustleaf
use math::adder;
var x = adder(1, 2);
assert(x == 3);
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
        Token(Use),
        Token(Ident, "math"),
        Token(DoubleColon),
        Token(Ident, "adder"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "x"),
        Token(Equal),
        Token(Ident, "adder"),
        Token(LeftParen),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(EqualEqual),
        Token(Int, "3"),
        Token(RightParen),
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
            Import(
                ImportSpec {
                    module: "math",
                    items: Specific(
                        [
                            ImportItem {
                                name: "adder",
                                alias: None,
                            },
                        ],
                    ),
                },
            ),
            VarDecl {
                pattern: Variable(
                    "x",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "adder",
                        ),
                        [
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
                        ],
                    ),
                ),
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Identifier(
                                "x",
                            ),
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                        ),
                    ],
                ),
            ),
        ],
    ),
)
```

# Eval
```rust
Ok(
    Block(
        [
            Import {
                module: "math",
                items: Specific(
                    [
                        ImportItem {
                            name: "adder",
                            alias: None,
                        },
                    ],
                ),
            },
            Declare(
                "x",
                Some(
                    Call(
                        Variable(
                            "adder",
                        ),
                        [
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
                        ],
                    ),
                ),
            ),
        ],
        Some(
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Variable(
                                "x",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                        ],
                    ),
                ],
            ),
        ),
    ),
)
```