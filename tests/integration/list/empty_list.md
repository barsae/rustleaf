# Program
Status: 🟢
Assertions: 1

```rustleaf
var empty = [];
assert(empty == []);
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
        Token(Ident, "empty"),
        Token(Equal),
        Token(LeftBracket),
        Token(RightBracket),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "empty"),
        Token(EqualEqual),
        Token(LeftBracket),
        Token(RightBracket),
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
            VarDecl {
                pattern: Variable(
                    "empty",
                ),
                value: Some(
                    List(
                        [],
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
                                "empty",
                            ),
                            List(
                                [],
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
    EvalProgram {
        statements: [
            EvalDeclare {
                name: "empty",
                init_expr: Some(
                    EvalList {
                        elements: [],
                    },
                ),
            },
            EvalCall {
                func_expr: EvalVariable {
                    name: "assert",
                },
                args: [
                    EvalCall {
                        func_expr: EvalGetAttr {
                            obj_expr: EvalVariable {
                                name: "empty",
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalList {
                                elements: [],
                            },
                        ],
                    },
                ],
            },
        ],
    },
)
```