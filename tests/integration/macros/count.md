# Program
Status: 🔴
Assertions: 0

```rustleaf
var counter = 0;

#[macro]
fn count(ast) {

    ast
}

#[count]
fn test() {}

test();
test();
test();
// assert(counter == 3);
```

# Output
None

# Result
```rust
Err(
    "Undefined variable: count",
)
```

# Lex
```rust
Ok(
    [
        Token(Var),
        Token(Ident, "counter"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(Hash),
        Token(LeftBracket),
        Token(Macro),
        Token(RightBracket),
        Token(Fn),
        Token(Ident, "count"),
        Token(LeftParen),
        Token(Ident, "ast"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "ast"),
        Token(RightBrace),
        Token(Hash),
        Token(LeftBracket),
        Token(Ident, "count"),
        Token(RightBracket),
        Token(Fn),
        Token(Ident, "test"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(RightBrace),
        Token(Ident, "test"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "test"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "test"),
        Token(LeftParen),
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
                    "counter",
                ),
                value: Some(
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
            },
            Macro {
                name: "macro",
                args: [],
                statement: FnDecl {
                    name: "count",
                    params: [
                        Parameter {
                            name: "ast",
                            default: None,
                            kind: Regular,
                        },
                    ],
                    body: Block {
                        statements: [],
                        final_expr: Some(
                            Identifier(
                                "ast",
                            ),
                        ),
                    },
                    is_pub: false,
                },
            },
            Macro {
                name: "count",
                args: [],
                statement: FnDecl {
                    name: "test",
                    params: [],
                    body: Block {
                        statements: [],
                        final_expr: None,
                    },
                    is_pub: false,
                },
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "test",
                    ),
                    [],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "test",
                    ),
                    [],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "test",
                    ),
                    [],
                ),
            ),
        ],
    ),
)
```

# Eval
```rust
Ok(
    Eval(
        EvalRef(
            EvalProgram {
                statements: [
                    EvalRef(
                        EvalDeclare {
                            name: "counter",
                            init_expr: Some(
                                EvalRef(
                                    EvalLiteral {
                                        value: Int(
                                            0,
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                    EvalRef(
                        EvalMacro {
                            data: MacroData {
                                macro_fn: Eval(
                                    EvalRef(
                                        EvalVariable {
                                            name: "macro",
                                        },
                                    ),
                                ),
                                target: Eval(
                                    EvalRef(
                                        EvalFunction {
                                            data: FunctionData {
                                                name: "count",
                                                params: [
                                                    (
                                                        "ast",
                                                        None,
                                                        Regular,
                                                    ),
                                                ],
                                                body: Eval(
                                                    EvalRef(
                                                        EvalBlock {
                                                            statements: [],
                                                            final_expr: Some(
                                                                EvalRef(
                                                                    EvalVariable {
                                                                        name: "ast",
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            },
                                        },
                                    ),
                                ),
                                args: [],
                            },
                        },
                    ),
                    EvalRef(
                        EvalMacro {
                            data: MacroData {
                                macro_fn: Eval(
                                    EvalRef(
                                        EvalVariable {
                                            name: "count",
                                        },
                                    ),
                                ),
                                target: Eval(
                                    EvalRef(
                                        EvalFunction {
                                            data: FunctionData {
                                                name: "test",
                                                params: [],
                                                body: Eval(
                                                    EvalRef(
                                                        EvalBlock {
                                                            statements: [],
                                                            final_expr: None,
                                                        },
                                                    ),
                                                ),
                                            },
                                        },
                                    ),
                                ),
                                args: [],
                            },
                        },
                    ),
                    EvalRef(
                        EvalCall {
                            func_expr: EvalRef(
                                EvalVariable {
                                    name: "test",
                                },
                            ),
                            args: [],
                        },
                    ),
                    EvalRef(
                        EvalCall {
                            func_expr: EvalRef(
                                EvalVariable {
                                    name: "test",
                                },
                            ),
                            args: [],
                        },
                    ),
                    EvalRef(
                        EvalCall {
                            func_expr: EvalRef(
                                EvalVariable {
                                    name: "test",
                                },
                            ),
                            args: [],
                        },
                    ),
                ],
            },
        ),
    ),
)
```