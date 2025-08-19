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
        0: Token(Use),
        1: Token(Ident, "math"),
        2: Token(DoubleColon),
        3: Token(Ident, "adder"),
        4: Token(Semicolon),
        5: Token(Var),
        6: Token(Ident, "x"),
        7: Token(Equal),
        8: Token(Ident, "adder"),
        9: Token(LeftParen),
        10: Token(Int, "1"),
        11: Token(Comma),
        12: Token(Int, "2"),
        13: Token(RightParen),
        14: Token(Semicolon),
        15: Token(Ident, "assert"),
        16: Token(LeftParen),
        17: Token(Ident, "x"),
        18: Token(EqualEqual),
        19: Token(Int, "3"),
        20: Token(RightParen),
        21: Token(Semicolon),
        22: Token(Eof)
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
    RustValue(<unknown>),
)
```