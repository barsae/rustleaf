# Program
Status: 🟢
Assertions: 1

```rustleaf
fn test_return() {
    return 42;
}

var result = test_return();
assert(result == 42);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Fn)
parse_statement: starting at position 0 (Fn)
consume_token: position 0 consumed Fn
consume_token: position 1 consumed Ident
consume_token: position 2 consumed LeftParen
consume_token: position 3 consumed RightParen
consume_token: position 4 consumed LeftBrace
parse_statement: starting at position 5 (Return)
consume_token: position 5 consumed Return
parse_expression: starting at position 6 (Int(42))
consume_token: position 6 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 7 consumed Semicolon
parse_statement: success - parsed return statement
consume_token: position 8 consumed RightBrace
parse_statement: success - parsed function declaration
parse_program: parsing statement at position 9 (Var)
parse_statement: starting at position 9 (Var)
consume_token: position 9 consumed Var
consume_token: position 10 consumed Ident
consume_token: position 11 consumed Equal
parse_expression: starting at position 12 (Ident(test_return))
consume_token: position 12 consumed Ident
parse_primary: success - parsed identifier (test_return)
consume_token: position 13 consumed LeftParen
consume_token: position 14 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 15 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 16 (Ident(assert))
parse_statement: starting at position 16 (Ident(assert))
consume_token: position 16 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 16 (Ident(assert))
consume_token: position 16 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 17 consumed LeftParen
parse_expression: starting at position 18 (Ident(result))
consume_token: position 18 consumed Ident
parse_primary: success - parsed identifier (result)
consume_token: position 19 consumed EqualEqual
consume_token: position 20 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 21 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 22 consumed Semicolon
parse_program: parsed 3 statements
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
        1: Token(Ident, "test_return"),
        2: Token(LeftParen),
        3: Token(RightParen),
        4: Token(LeftBrace),
        5: Token(Return),
        6: Token(Int, "42"),
        7: Token(Semicolon),
        8: Token(RightBrace),
        9: Token(Var),
        10: Token(Ident, "result"),
        11: Token(Equal),
        12: Token(Ident, "test_return"),
        13: Token(LeftParen),
        14: Token(RightParen),
        15: Token(Semicolon),
        16: Token(Ident, "assert"),
        17: Token(LeftParen),
        18: Token(Ident, "result"),
        19: Token(EqualEqual),
        20: Token(Int, "42"),
        21: Token(RightParen),
        22: Token(Semicolon),
        23: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            FnDecl {
                name: "test_return",
                params: [],
                body: Block {
                    statements: [
                        Return(
                            Some(
                                Literal(
                                    Int(
                                        42,
                                    ),
                                ),
                            ),
                        ),
                    ],
                    final_expr: None,
                },
                is_pub: false,
            },
            VarDecl {
                pattern: Variable(
                    "result",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "test_return",
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
                                "result",
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
    RustValue(
        EvalProgram {
            statements: [
                RustValue(
                    EvalFunction {
                        data: FunctionData {
                            name: "test_return",
                            params: [],
                            body: RustValue(
                                EvalBlock {
                                    statements: [
                                        RustValue(
                                            EvalReturn {
                                                expr: Some(
                                                    RustValue(
                                                        EvalLiteral {
                                                            value: Int(
                                                                42,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                    ],
                                    final_expr: None,
                                },
                            ),
                        },
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "result",
                        init_expr: Some(
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalVariable {
                                            name: "test_return",
                                        },
                                    ),
                                    args: [],
                                },
                            ),
                        ),
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
                                                EvalVariable {
                                                    name: "result",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    42,
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