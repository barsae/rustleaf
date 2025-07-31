# Program
Status: 🟢
Assertions: 2

```rustleaf
var [first, *rest] = [1, 2, 3, 4];
assert(first == 1);
assert(rest == [2, 3, 4]);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
consume_token: position 0 consumed Var
consume_token: position 1 consumed LeftBracket
consume_token: position 2 consumed Ident
consume_token: position 3 consumed Comma
consume_token: position 4 consumed Star
consume_token: position 5 consumed Ident
consume_token: position 6 consumed RightBracket
consume_token: position 7 consumed Equal
parse_expression: starting at position 8 (LeftBracket)
consume_token: position 8 consumed LeftBracket
parse_primary: success - parsing list literal
parse_list_literal: starting at position 9
parse_list_literal: parsing element at position 9
parse_expression: starting at position 9 (Int(1))
consume_token: position 9 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 10 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 11
parse_expression: starting at position 11 (Int(2))
consume_token: position 11 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 12 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 13
parse_expression: starting at position 13 (Int(3))
consume_token: position 13 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 14 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 15
parse_expression: starting at position 15 (Int(4))
consume_token: position 15 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_list_literal: no comma, expecting ]
parse_list_literal: expecting ] at position 16
consume_token: position 16 consumed RightBracket
parse_list_literal: success
parse_expression: success - parsed precedence expression
consume_token: position 17 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 18 (Ident(assert))
parse_statement: starting at position 18 (Ident(assert))
consume_token: position 18 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 18 (Ident(assert))
consume_token: position 18 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 19 consumed LeftParen
parse_expression: starting at position 20 (Ident(first))
consume_token: position 20 consumed Ident
parse_primary: success - parsed identifier (first)
consume_token: position 21 consumed EqualEqual
consume_token: position 22 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 23 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 24 consumed Semicolon
parse_program: parsing statement at position 25 (Ident(assert))
parse_statement: starting at position 25 (Ident(assert))
consume_token: position 25 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 25 (Ident(assert))
consume_token: position 25 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 26 consumed LeftParen
parse_expression: starting at position 27 (Ident(rest))
consume_token: position 27 consumed Ident
parse_primary: success - parsed identifier (rest)
consume_token: position 28 consumed EqualEqual
consume_token: position 29 consumed LeftBracket
parse_primary: success - parsing list literal
parse_list_literal: starting at position 30
parse_list_literal: parsing element at position 30
parse_expression: starting at position 30 (Int(2))
consume_token: position 30 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 31 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 32
parse_expression: starting at position 32 (Int(3))
consume_token: position 32 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 33 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 34
parse_expression: starting at position 34 (Int(4))
consume_token: position 34 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_list_literal: no comma, expecting ]
parse_list_literal: expecting ] at position 35
consume_token: position 35 consumed RightBracket
parse_list_literal: success
parse_expression: success - parsed precedence expression
consume_token: position 36 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 37 consumed Semicolon
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
        1: Token(LeftBracket),
        2: Token(Ident, "first"),
        3: Token(Comma),
        4: Token(Star),
        5: Token(Ident, "rest"),
        6: Token(RightBracket),
        7: Token(Equal),
        8: Token(LeftBracket),
        9: Token(Int, "1"),
        10: Token(Comma),
        11: Token(Int, "2"),
        12: Token(Comma),
        13: Token(Int, "3"),
        14: Token(Comma),
        15: Token(Int, "4"),
        16: Token(RightBracket),
        17: Token(Semicolon),
        18: Token(Ident, "assert"),
        19: Token(LeftParen),
        20: Token(Ident, "first"),
        21: Token(EqualEqual),
        22: Token(Int, "1"),
        23: Token(RightParen),
        24: Token(Semicolon),
        25: Token(Ident, "assert"),
        26: Token(LeftParen),
        27: Token(Ident, "rest"),
        28: Token(EqualEqual),
        29: Token(LeftBracket),
        30: Token(Int, "2"),
        31: Token(Comma),
        32: Token(Int, "3"),
        33: Token(Comma),
        34: Token(Int, "4"),
        35: Token(RightBracket),
        36: Token(RightParen),
        37: Token(Semicolon),
        38: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            VarDecl {
                pattern: ListRest(
                    [
                        Variable(
                            "first",
                        ),
                    ],
                    Some(
                        "rest",
                    ),
                ),
                value: Some(
                    List(
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
                                "first",
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
                            Identifier(
                                "rest",
                            ),
                            List(
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
                    EvalDeclarePattern {
                        pattern: ListRest(
                            [
                                Variable(
                                    "first",
                                ),
                            ],
                            Some(
                                "rest",
                            ),
                        ),
                        init_expr: RustValue(
                            EvalList {
                                elements: [
                                    RustValue(
                                        EvalLiteral {
                                            value: Int(
                                                1,
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
                                    RustValue(
                                        EvalLiteral {
                                            value: Int(
                                                3,
                                            ),
                                        },
                                    ),
                                    RustValue(
                                        EvalLiteral {
                                            value: Int(
                                                4,
                                            ),
                                        },
                                    ),
                                ],
                            },
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
                                                    name: "first",
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
                                                EvalVariable {
                                                    name: "rest",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalList {
                                                elements: [
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
                                                    RustValue(
                                                        EvalLiteral {
                                                            value: Int(
                                                                4,
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
            ],
        },
    ),
)
```