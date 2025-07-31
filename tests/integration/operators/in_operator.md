# Program
Status: 🟢
Assertions: 1

```rustleaf
var l = [1, 2, 3];
assert(1 in l);
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
parse_expression: starting at position 13 (Int(1))
consume_token: position 13 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 14 consumed In
consume_token: position 15 consumed Ident
parse_primary: success - parsed identifier (l)
parse_expression: success - parsed precedence expression
consume_token: position 16 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 17 consumed Semicolon
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
        0: Token(Var),
        1: Token(Ident, "l"),
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
        13: Token(Int, "1"),
        14: Token(In),
        15: Token(Ident, "l"),
        16: Token(RightParen),
        17: Token(Semicolon),
        18: Token(Eof)
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
                    "l",
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
                        In(
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                            Identifier(
                                "l",
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
                        name: "l",
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
                                                EvalVariable {
                                                    name: "l",
                                                },
                                            ),
                                            attr_name: "op_contains",
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
            ],
        },
    ),
)
```