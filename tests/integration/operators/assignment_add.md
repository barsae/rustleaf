# Program
Status: 🟢
Assertions: 1

```rustleaf
var x = 0;
x += 5;
assert(x == 5);
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
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(Ident, "x"),
        Token(PlusEqual),
        Token(Int, "5"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(EqualEqual),
        Token(Int, "5"),
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
                    "x",
                ),
                value: Some(
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
            },
            Assignment {
                target: Identifier(
                    "x",
                ),
                op: AddAssign,
                value: Literal(
                    Int(
                        5,
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
                                    5,
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
    EvalProgram {
        statements: [
            EvalDeclare {
                name: "x",
                init_expr: Some(
                    EvalLiteral {
                        value: Int(
                            0,
                        ),
                    },
                ),
            },
            EvalAssign {
                name: "x",
                expr: EvalCall {
                    func_expr: EvalGetAttr {
                        obj_expr: EvalVariable {
                            name: "x",
                        },
                        attr_name: "op_add",
                    },
                    args: [
                        EvalLiteral {
                            value: Int(
                                5,
                            ),
                        },
                    ],
                },
            },
            EvalCall {
                func_expr: EvalVariable {
                    name: "assert",
                },
                args: [
                    EvalCall {
                        func_expr: EvalGetAttr {
                            obj_expr: EvalVariable {
                                name: "x",
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: Int(
                                    5,
                                ),
                            },
                        ],
                    },
                ],
            },
        ],
    },
)
```