# Program
Status: 🟢
Assertions: 2

```rustleaf
// List indexing and operations
var my_list = [1, 2, 3];
assert(my_list[0] == 1);
assert(my_list[2] == 3);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
consume_token: position 0 consumed Var
consume_token: position 1 consumed Ident
consume_token: position 2 consumed Equal
parse_expression: starting at position 3 (LeftBracket)
consume_token: position 3 consumed LeftBracket
parse_primary: success - parsing list literal
parse_list_literal: starting at position 4
parse_list_literal: parsing element at position 4
parse_expression: starting at position 4 (Int(1))
consume_token: position 4 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 5 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 6
parse_expression: starting at position 6 (Int(2))
consume_token: position 6 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 7 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 8
parse_expression: starting at position 8 (Int(3))
consume_token: position 8 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_list_literal: no comma, expecting ]
parse_list_literal: expecting ] at position 9
consume_token: position 9 consumed RightBracket
parse_list_literal: success
parse_expression: success - parsed precedence expression
consume_token: position 10 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 11 (Ident(assert))
parse_statement: starting at position 11 (Ident(assert))
consume_token: position 11 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 11 (Ident(assert))
consume_token: position 11 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 12 consumed LeftParen
parse_expression: starting at position 13 (Ident(my_list))
consume_token: position 13 consumed Ident
parse_primary: success - parsed identifier (my_list)
consume_token: position 14 consumed LeftBracket
parse_expression: starting at position 15 (Int(0))
consume_token: position 15 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 16 consumed RightBracket
consume_token: position 17 consumed EqualEqual
consume_token: position 18 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 19 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 20 consumed Semicolon
parse_program: parsing statement at position 21 (Ident(assert))
parse_statement: starting at position 21 (Ident(assert))
consume_token: position 21 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 21 (Ident(assert))
consume_token: position 21 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 22 consumed LeftParen
parse_expression: starting at position 23 (Ident(my_list))
consume_token: position 23 consumed Ident
parse_primary: success - parsed identifier (my_list)
consume_token: position 24 consumed LeftBracket
parse_expression: starting at position 25 (Int(2))
consume_token: position 25 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 26 consumed RightBracket
consume_token: position 27 consumed EqualEqual
consume_token: position 28 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 29 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 30 consumed Semicolon
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
        1: Token(Ident, "my_list"),
        2: Token(Equal),
        3: Token(LeftBracket),
        4: Token(Int, "1"),
        5: Token(Comma),
        6: Token(Int, "2"),
        7: Token(Comma),
        8: Token(Int, "3"),
        9: Token(RightBracket),
        10: Token(Semicolon),
        11: Token(Ident, "assert"),
        12: Token(LeftParen),
        13: Token(Ident, "my_list"),
        14: Token(LeftBracket),
        15: Token(Int, "0"),
        16: Token(RightBracket),
        17: Token(EqualEqual),
        18: Token(Int, "1"),
        19: Token(RightParen),
        20: Token(Semicolon),
        21: Token(Ident, "assert"),
        22: Token(LeftParen),
        23: Token(Ident, "my_list"),
        24: Token(LeftBracket),
        25: Token(Int, "2"),
        26: Token(RightBracket),
        27: Token(EqualEqual),
        28: Token(Int, "3"),
        29: Token(RightParen),
        30: Token(Semicolon),
        31: Token(Eof)
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
                    "my_list",
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
                                    "my_list",
                                ),
                                Literal(
                                    Int(
                                        0,
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
                                    "my_list",
                                ),
                                Literal(
                                    Int(
                                        2,
                                    ),
                                ),
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
    RustValue(
        EvalProgram {
            statements: [
                RustValue(
                    EvalDeclare {
                        name: "my_list",
                        init_expr: Some(
                            RustValue(
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
                                                            name: "my_list",
                                                        },
                                                    ),
                                                    index_expr: RustValue(
                                                        EvalLiteral {
                                                            value: Int(
                                                                0,
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
                                                            name: "my_list",
                                                        },
                                                    ),
                                                    index_expr: RustValue(
                                                        EvalLiteral {
                                                            value: Int(
                                                                2,
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
                                                    3,
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