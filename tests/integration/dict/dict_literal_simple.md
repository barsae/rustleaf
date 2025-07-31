# Program
Status: 🟢
Assertions: 2

```rustleaf
var x = {"a": 1, "b": 2,};
assert(x["a"] == 1);
assert(x["b"] == 2);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
consume_token: position 0 consumed Var
consume_token: position 1 consumed Ident
consume_token: position 2 consumed Equal
parse_expression: starting at position 3 (LeftBrace)
parse_primary: success - parsing block or dict
consume_token: position 3 consumed LeftBrace
consume_token: position 4 consumed String
parse_primary: success - parsed numeric/string literal
consume_token: position 5 consumed Colon
parse_expression: starting at position 6 (Int(1))
consume_token: position 6 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 7 consumed Comma
consume_token: position 8 consumed String
parse_primary: success - parsed numeric/string literal
consume_token: position 9 consumed Colon
parse_expression: starting at position 10 (Int(2))
consume_token: position 10 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 11 consumed Comma
consume_token: position 12 consumed RightBrace
parse_expression: success - parsed precedence expression
consume_token: position 13 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 14 (Ident(assert))
parse_statement: starting at position 14 (Ident(assert))
consume_token: position 14 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 14 (Ident(assert))
consume_token: position 14 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 15 consumed LeftParen
parse_expression: starting at position 16 (Ident(x))
consume_token: position 16 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 17 consumed LeftBracket
parse_expression: starting at position 18 (String(a))
consume_token: position 18 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 19 consumed RightBracket
consume_token: position 20 consumed EqualEqual
consume_token: position 21 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 22 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 23 consumed Semicolon
parse_program: parsing statement at position 24 (Ident(assert))
parse_statement: starting at position 24 (Ident(assert))
consume_token: position 24 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 24 (Ident(assert))
consume_token: position 24 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 25 consumed LeftParen
parse_expression: starting at position 26 (Ident(x))
consume_token: position 26 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 27 consumed LeftBracket
parse_expression: starting at position 28 (String(b))
consume_token: position 28 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 29 consumed RightBracket
consume_token: position 30 consumed EqualEqual
consume_token: position 31 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 32 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 33 consumed Semicolon
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
        0: Token(Var),
        1: Token(Ident, "x"),
        2: Token(Equal),
        3: Token(LeftBrace),
        4: Token(String, "a"),
        5: Token(Colon),
        6: Token(Int, "1"),
        7: Token(Comma),
        8: Token(String, "b"),
        9: Token(Colon),
        10: Token(Int, "2"),
        11: Token(Comma),
        12: Token(RightBrace),
        13: Token(Semicolon),
        14: Token(Ident, "assert"),
        15: Token(LeftParen),
        16: Token(Ident, "x"),
        17: Token(LeftBracket),
        18: Token(String, "a"),
        19: Token(RightBracket),
        20: Token(EqualEqual),
        21: Token(Int, "1"),
        22: Token(RightParen),
        23: Token(Semicolon),
        24: Token(Ident, "assert"),
        25: Token(LeftParen),
        26: Token(Ident, "x"),
        27: Token(LeftBracket),
        28: Token(String, "b"),
        29: Token(RightBracket),
        30: Token(EqualEqual),
        31: Token(Int, "2"),
        32: Token(RightParen),
        33: Token(Semicolon),
        34: Token(Eof)
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
                    Dict(
                        [
                            (
                                Literal(
                                    String(
                                        "a",
                                    ),
                                ),
                                Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                            ),
                            (
                                Literal(
                                    String(
                                        "b",
                                    ),
                                ),
                                Literal(
                                    Int(
                                        2,
                                    ),
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
                            GetItem(
                                Identifier(
                                    "x",
                                ),
                                Literal(
                                    String(
                                        "a",
                                    ),
                                ),
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            GetItem(
                                Identifier(
                                    "x",
                                ),
                                Literal(
                                    String(
                                        "b",
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    2,
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
                    EvalDeclare {
                        name: "x",
                        init_expr: Some(
                            RustValue(
                                EvalDict {
                                    pairs: [
                                        (
                                            RustValue(
                                                EvalLiteral {
                                                    value: String(
                                                        "a",
                                                    ),
                                                },
                                            ),
                                            RustValue(
                                                EvalLiteral {
                                                    value: Int(
                                                        1,
                                                    ),
                                                },
                                            ),
                                        ),
                                        (
                                            RustValue(
                                                EvalLiteral {
                                                    value: String(
                                                        "b",
                                                    ),
                                                },
                                            ),
                                            RustValue(
                                                EvalLiteral {
                                                    value: Int(
                                                        2,
                                                    ),
                                                },
                                            ),
                                        ),
                                    ],
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
                                                EvalGetItem {
                                                    obj_expr: RustValue(
                                                        EvalVariable {
                                                            name: "x",
                                                        },
                                                    ),
                                                    index_expr: RustValue(
                                                        EvalLiteral {
                                                            value: String(
                                                                "a",
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    1,
                                                ),
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ],
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
                                                EvalGetItem {
                                                    obj_expr: RustValue(
                                                        EvalVariable {
                                                            name: "x",
                                                        },
                                                    ),
                                                    index_expr: RustValue(
                                                        EvalLiteral {
                                                            value: String(
                                                                "b",
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                            attr_name: "op_eq",
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