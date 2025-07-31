# Program
Status: 🟢
Assertions: 1

```rustleaf
var arr = [1, 2, 3];
arr[0] = 99;
assert(arr == [99, 2, 3]);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
parse_expression: starting at position 3 (LeftBracket)
parse_primary: success - parsing list literal
parse_expression: starting at position 4 (Int(1))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: starting at position 6 (Int(2))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: starting at position 8 (Int(3))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 11 (Ident(arr))
parse_statement: starting at position 11 (Ident(arr))
parse_expression: starting at position 13 (Int(0))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: starting at position 16 (Int(99))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: success - parsed assignment
parse_program: parsing statement at position 18 (Ident(assert))
parse_statement: starting at position 18 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 18 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 20 (Ident(arr))
parse_primary: success - parsed identifier (arr)
parse_primary: success - parsing list literal
parse_expression: starting at position 23 (Int(99))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: starting at position 25 (Int(2))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: starting at position 27 (Int(3))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
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
        Token(Var),
        Token(Ident, "arr"),
        Token(Equal),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightBracket),
        Token(Semicolon),
        Token(Ident, "arr"),
        Token(LeftBracket),
        Token(Int, "0"),
        Token(RightBracket),
        Token(Equal),
        Token(Int, "99"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "arr"),
        Token(EqualEqual),
        Token(LeftBracket),
        Token(Int, "99"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
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
                    "arr",
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
            Assignment {
                target: GetItem(
                    Identifier(
                        "arr",
                    ),
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
                op: Assign,
                value: Literal(
                    Int(
                        99,
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
                                "arr",
                            ),
                            List(
                                [
                                    Literal(
                                        Int(
                                            99,
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
                        name: "arr",
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
                    EvalSetItem {
                        obj_expr: RustValue(
                            EvalVariable {
                                name: "arr",
                            },
                        ),
                        index_expr: RustValue(
                            EvalLiteral {
                                value: Int(
                                    0,
                                ),
                            },
                        ),
                        value_expr: RustValue(
                            EvalLiteral {
                                value: Int(
                                    99,
                                ),
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
                                                    name: "arr",
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
                                                                99,
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