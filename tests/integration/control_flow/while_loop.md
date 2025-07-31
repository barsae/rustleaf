# Program
Status: 🟢
Assertions: 2

```rustleaf
var x = 0;
var count = 0;
while x < 3 {
    count = count + 1;
    x = x + 1;
}
assert(x == 3);
assert(count == 3);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
consume_token: position 0 consumed Var
consume_token: position 1 consumed Ident
consume_token: position 2 consumed Equal
parse_expression: starting at position 3 (Int(0))
consume_token: position 3 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 4 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 5 (Var)
parse_statement: starting at position 5 (Var)
consume_token: position 5 consumed Var
consume_token: position 6 consumed Ident
consume_token: position 7 consumed Equal
parse_expression: starting at position 8 (Int(0))
consume_token: position 8 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 9 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 10 (While)
parse_statement: starting at position 10 (While)
parse_expression: starting at position 10 (While)
consume_token: position 10 consumed While
parse_primary: success - parsing while expression
parse_expression: starting at position 11 (Ident(x))
consume_token: position 11 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 12 consumed Less
consume_token: position 13 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 14 consumed LeftBrace
parse_statement: starting at position 15 (Ident(count))
consume_token: position 15 consumed Ident
consume_token: position 16 consumed Equal
parse_expression: starting at position 17 (Ident(count))
consume_token: position 17 consumed Ident
parse_primary: success - parsed identifier (count)
consume_token: position 18 consumed Plus
consume_token: position 19 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 20 consumed Semicolon
parse_statement: success - parsed assignment
parse_statement: starting at position 21 (Ident(x))
consume_token: position 21 consumed Ident
consume_token: position 22 consumed Equal
parse_expression: starting at position 23 (Ident(x))
consume_token: position 23 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 24 consumed Plus
consume_token: position 25 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 26 consumed Semicolon
parse_statement: success - parsed assignment
consume_token: position 27 consumed RightBrace
parse_expression: success - parsed precedence expression
parse_statement: success - parsed block-like expression statement
parse_program: parsing statement at position 28 (Ident(assert))
parse_statement: starting at position 28 (Ident(assert))
consume_token: position 28 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 28 (Ident(assert))
consume_token: position 28 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 29 consumed LeftParen
parse_expression: starting at position 30 (Ident(x))
consume_token: position 30 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 31 consumed EqualEqual
consume_token: position 32 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 33 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 34 consumed Semicolon
parse_program: parsing statement at position 35 (Ident(assert))
parse_statement: starting at position 35 (Ident(assert))
consume_token: position 35 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 35 (Ident(assert))
consume_token: position 35 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 36 consumed LeftParen
parse_expression: starting at position 37 (Ident(count))
consume_token: position 37 consumed Ident
parse_primary: success - parsed identifier (count)
consume_token: position 38 consumed EqualEqual
consume_token: position 39 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 40 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 41 consumed Semicolon
parse_program: parsed 5 statements
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
        3: Token(Int, "0"),
        4: Token(Semicolon),
        5: Token(Var),
        6: Token(Ident, "count"),
        7: Token(Equal),
        8: Token(Int, "0"),
        9: Token(Semicolon),
        10: Token(While),
        11: Token(Ident, "x"),
        12: Token(Less),
        13: Token(Int, "3"),
        14: Token(LeftBrace),
        15: Token(Ident, "count"),
        16: Token(Equal),
        17: Token(Ident, "count"),
        18: Token(Plus),
        19: Token(Int, "1"),
        20: Token(Semicolon),
        21: Token(Ident, "x"),
        22: Token(Equal),
        23: Token(Ident, "x"),
        24: Token(Plus),
        25: Token(Int, "1"),
        26: Token(Semicolon),
        27: Token(RightBrace),
        28: Token(Ident, "assert"),
        29: Token(LeftParen),
        30: Token(Ident, "x"),
        31: Token(EqualEqual),
        32: Token(Int, "3"),
        33: Token(RightParen),
        34: Token(Semicolon),
        35: Token(Ident, "assert"),
        36: Token(LeftParen),
        37: Token(Ident, "count"),
        38: Token(EqualEqual),
        39: Token(Int, "3"),
        40: Token(RightParen),
        41: Token(Semicolon),
        42: Token(Eof)
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
            VarDecl {
                pattern: Variable(
                    "count",
                ),
                value: Some(
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
            },
            Expression(
                While {
                    condition: Lt(
                        Identifier(
                            "x",
                        ),
                        Literal(
                            Int(
                                3,
                            ),
                        ),
                    ),
                    body: Block {
                        statements: [
                            Assignment {
                                target: Identifier(
                                    "count",
                                ),
                                op: Assign,
                                value: Add(
                                    Identifier(
                                        "count",
                                    ),
                                    Literal(
                                        Int(
                                            1,
                                        ),
                                    ),
                                ),
                            },
                            Assignment {
                                target: Identifier(
                                    "x",
                                ),
                                op: Assign,
                                value: Add(
                                    Identifier(
                                        "x",
                                    ),
                                    Literal(
                                        Int(
                                            1,
                                        ),
                                    ),
                                ),
                            },
                        ],
                        final_expr: None,
                    },
                },
            ),
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
                                    3,
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
                                "count",
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
                        name: "x",
                        init_expr: Some(
                            RustValue(
                                EvalLiteral {
                                    value: Int(
                                        0,
                                    ),
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "count",
                        init_expr: Some(
                            RustValue(
                                EvalLiteral {
                                    value: Int(
                                        0,
                                    ),
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalWhile {
                        condition: RustValue(
                            EvalCall {
                                func_expr: RustValue(
                                    EvalGetAttr {
                                        obj_expr: RustValue(
                                            EvalVariable {
                                                name: "x",
                                            },
                                        ),
                                        attr_name: "op_lt",
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
                        body: RustValue(
                            EvalBlock {
                                statements: [
                                    RustValue(
                                        EvalAssign {
                                            name: "count",
                                            expr: RustValue(
                                                EvalCall {
                                                    func_expr: RustValue(
                                                        EvalGetAttr {
                                                            obj_expr: RustValue(
                                                                EvalVariable {
                                                                    name: "count",
                                                                },
                                                            ),
                                                            attr_name: "op_add",
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
                                        },
                                    ),
                                    RustValue(
                                        EvalAssign {
                                            name: "x",
                                            expr: RustValue(
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
                                                            EvalLiteral {
                                                                value: Int(
                                                                    1,
                                                                ),
                                                            },
                                                        ),
                                                    ],
                                                },
                                            ),
                                        },
                                    ),
                                ],
                                final_expr: None,
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
                                                    name: "x",
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
                                                    name: "count",
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