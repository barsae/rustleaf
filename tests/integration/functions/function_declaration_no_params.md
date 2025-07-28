# Program
Status: 🟢
Assertions: 4

```rustleaf
fn hello() { 42 }
fn get_pi() { 3.14 }
fn get_greeting() { "Hello, World!" }

var result1 = hello();
var result2 = get_pi();
var result3 = get_greeting();

assert(result1 == 42);
assert(result2 == 3.14);
assert(result3 == "Hello, World!");
assert(hello() == 42);
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
        Token(Fn),
        Token(Ident, "hello"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(Int, "42"),
        Token(RightBrace),
        Token(Fn),
        Token(Ident, "get_pi"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(Float, "3.14"),
        Token(RightBrace),
        Token(Fn),
        Token(Ident, "get_greeting"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(String, "Hello, World!"),
        Token(RightBrace),
        Token(Var),
        Token(Ident, "result1"),
        Token(Equal),
        Token(Ident, "hello"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result2"),
        Token(Equal),
        Token(Ident, "get_pi"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result3"),
        Token(Equal),
        Token(Ident, "get_greeting"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result1"),
        Token(EqualEqual),
        Token(Int, "42"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result2"),
        Token(EqualEqual),
        Token(Float, "3.14"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result3"),
        Token(EqualEqual),
        Token(String, "Hello, World!"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "hello"),
        Token(LeftParen),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "42"),
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
            FnDecl {
                name: "hello",
                params: [],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Literal(
                            Int(
                                42,
                            ),
                        ),
                    ),
                },
                is_pub: false,
            },
            FnDecl {
                name: "get_pi",
                params: [],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Literal(
                            Float(
                                3.14,
                            ),
                        ),
                    ),
                },
                is_pub: false,
            },
            FnDecl {
                name: "get_greeting",
                params: [],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Literal(
                            String(
                                "Hello, World!",
                            ),
                        ),
                    ),
                },
                is_pub: false,
            },
            VarDecl {
                pattern: Variable(
                    "result1",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "hello",
                        ),
                        [],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result2",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "get_pi",
                        ),
                        [],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result3",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "get_greeting",
                        ),
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
                                "result1",
                            ),
                            Literal(
                                Int(
                                    42,
                                ),
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Identifier(
                                "result2",
                            ),
                            Literal(
                                Float(
                                    3.14,
                                ),
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Identifier(
                                "result3",
                            ),
                            Literal(
                                String(
                                    "Hello, World!",
                                ),
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            FunctionCall(
                                Identifier(
                                    "hello",
                                ),
                                [],
                            ),
                            Literal(
                                Int(
                                    42,
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
            EvalFunction {
                data: FunctionData {
                    name: "hello",
                    params: [],
                    body: EvalBlock {
                        statements: [],
                        final_expr: Some(
                            EvalLiteral {
                                value: Int(
                                    42,
                                ),
                            },
                        ),
                    },
                },
            },
            EvalFunction {
                data: FunctionData {
                    name: "get_pi",
                    params: [],
                    body: EvalBlock {
                        statements: [],
                        final_expr: Some(
                            EvalLiteral {
                                value: Float(
                                    3.14,
                                ),
                            },
                        ),
                    },
                },
            },
            EvalFunction {
                data: FunctionData {
                    name: "get_greeting",
                    params: [],
                    body: EvalBlock {
                        statements: [],
                        final_expr: Some(
                            EvalLiteral {
                                value: String(
                                    "Hello, World!",
                                ),
                            },
                        ),
                    },
                },
            },
            EvalDeclare {
                name: "result1",
                init_expr: Some(
                    EvalCall {
                        func_expr: EvalVariable {
                            name: "hello",
                        },
                        args: [],
                    },
                ),
            },
            EvalDeclare {
                name: "result2",
                init_expr: Some(
                    EvalCall {
                        func_expr: EvalVariable {
                            name: "get_pi",
                        },
                        args: [],
                    },
                ),
            },
            EvalDeclare {
                name: "result3",
                init_expr: Some(
                    EvalCall {
                        func_expr: EvalVariable {
                            name: "get_greeting",
                        },
                        args: [],
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
                                name: "result1",
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: Int(
                                    42,
                                ),
                            },
                        ],
                    },
                ],
            },
            EvalCall {
                func_expr: EvalVariable {
                    name: "assert",
                },
                args: [
                    EvalCall {
                        func_expr: EvalGetAttr {
                            obj_expr: EvalVariable {
                                name: "result2",
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: Float(
                                    3.14,
                                ),
                            },
                        ],
                    },
                ],
            },
            EvalCall {
                func_expr: EvalVariable {
                    name: "assert",
                },
                args: [
                    EvalCall {
                        func_expr: EvalGetAttr {
                            obj_expr: EvalVariable {
                                name: "result3",
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: String(
                                    "Hello, World!",
                                ),
                            },
                        ],
                    },
                ],
            },
            EvalCall {
                func_expr: EvalVariable {
                    name: "assert",
                },
                args: [
                    EvalCall {
                        func_expr: EvalGetAttr {
                            obj_expr: EvalCall {
                                func_expr: EvalVariable {
                                    name: "hello",
                                },
                                args: [],
                            },
                            attr_name: "op_eq",
                        },
                        args: [
                            EvalLiteral {
                                value: Int(
                                    42,
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