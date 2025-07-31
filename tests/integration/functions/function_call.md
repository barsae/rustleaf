# Program
Status: 🟢
Assertions: 1

```rustleaf
fn add(x, y) { x + y }
assert(add(2, 3) == 5);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Fn)
parse_statement: starting at position 0 (Fn)
consume_token: position 0 consumed Fn
consume_token: position 1 consumed Ident
consume_token: position 2 consumed LeftParen
consume_token: position 3 consumed Ident
consume_token: position 4 consumed Comma
consume_token: position 5 consumed Ident
consume_token: position 6 consumed RightParen
consume_token: position 7 consumed LeftBrace
parse_statement: starting at position 8 (Ident(x))
consume_token: position 8 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 8 (Ident(x))
consume_token: position 8 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 9 consumed Plus
consume_token: position 10 consumed Ident
parse_primary: success - parsed identifier (y)
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 11
parse_expression: starting at position 8 (Ident(x))
consume_token: position 8 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 9 consumed Plus
consume_token: position 10 consumed Ident
parse_primary: success - parsed identifier (y)
parse_expression: success - parsed precedence expression
consume_token: position 11 consumed RightBrace
parse_statement: success - parsed function declaration
parse_program: parsing statement at position 12 (Ident(assert))
parse_statement: starting at position 12 (Ident(assert))
consume_token: position 12 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 12 (Ident(assert))
consume_token: position 12 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 13 consumed LeftParen
parse_expression: starting at position 14 (Ident(add))
consume_token: position 14 consumed Ident
parse_primary: success - parsed identifier (add)
consume_token: position 15 consumed LeftParen
parse_expression: starting at position 16 (Int(2))
consume_token: position 16 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 17 consumed Comma
parse_expression: starting at position 18 (Int(3))
consume_token: position 18 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 19 consumed RightParen
consume_token: position 20 consumed EqualEqual
consume_token: position 21 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 22 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 23 consumed Semicolon
parse_program: parsed 2 statements
```

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
        0: Token(Fn),
        1: Token(Ident, "add"),
        2: Token(LeftParen),
        3: Token(Ident, "x"),
        4: Token(Comma),
        5: Token(Ident, "y"),
        6: Token(RightParen),
        7: Token(LeftBrace),
        8: Token(Ident, "x"),
        9: Token(Plus),
        10: Token(Ident, "y"),
        11: Token(RightBrace),
        12: Token(Ident, "assert"),
        13: Token(LeftParen),
        14: Token(Ident, "add"),
        15: Token(LeftParen),
        16: Token(Int, "2"),
        17: Token(Comma),
        18: Token(Int, "3"),
        19: Token(RightParen),
        20: Token(EqualEqual),
        21: Token(Int, "5"),
        22: Token(RightParen),
        23: Token(Semicolon),
        24: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            FnDecl {
                name: "add",
                params: [
                    Parameter {
                        name: "x",
                        default: None,
                        kind: Regular,
                    },
                    Parameter {
                        name: "y",
                        default: None,
                        kind: Regular,
                    },
                ],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Add(
                            Identifier(
                                "x",
                            ),
                            Identifier(
                                "y",
                            ),
                        ),
                    ),
                },
                is_pub: false,
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            FunctionCall(
                                Identifier(
                                    "add",
                                ),
                                [
                                    Literal(
                                        Int(
                                            2,
                                        ),
                                    ),
                                    Literal(
                                        Int(
                                            3,
                                        ),
                                    ),
                                ],
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
    RustValue(
        EvalProgram {
            statements: [
                RustValue(
                    EvalFunction {
                        data: FunctionData {
                            name: "add",
                            params: [
                                (
                                    "x",
                                    None,
                                    Regular,
                                ),
                                (
                                    "y",
                                    None,
                                    Regular,
                                ),
                            ],
                            body: RustValue(
                                EvalBlock {
                                    statements: [],
                                    final_expr: Some(
                                        RustValue(
                                            EvalCall {
                                                func_expr: RustValue(
                                                    EvalGetAttr {
                                                        obj_expr: RustValue(
                                                            EvalVariable {
                                                                name: "x",
                                                            },
                                                        ),
                                                        attr_name: "op_add",
                                                    },
                                                ),
                                                args: [
                                                    RustValue(
                                                        EvalVariable {
                                                            name: "y",
                                                        },
                                                    ),
                                                ],
                                            },
                                        ),
                                    ),
                                },
                            ),
                        },
                    },
                ),
                RustValue(
                    EvalCall {
                        func_expr: RustValue(
                            EvalVariable {
                                name: "assert",
                            },
                        ),
                        args: [
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalGetAttr {
                                            obj_expr: RustValue(
                                                EvalCall {
                                                    func_expr: RustValue(
                                                        EvalVariable {
                                                            name: "add",
                                                        },
                                                    ),
                                                    args: [
                                                        RustValue(
                                                            EvalLiteral {
                                                                value: Int(
                                                                    2,
                                                                ),
                                                            },
                                                        ),
                                                        RustValue(
                                                            EvalLiteral {
                                                                value: Int(
                                                                    3,
                                                                ),
                                                            },
                                                        ),
                                                    ],
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    5,
                                                ),
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ],
                    },
                ),
            ],
        },
    ),
)
```