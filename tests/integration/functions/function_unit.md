# Program
Status: 🟢
Assertions: 2

```rustleaf
var z = 0;
fn add(x, y) {
    z += 1;
    x + y;
}
assert(is_unit(add(2, 3)));
assert(z == 1);
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
parse_program: parsing statement at position 5 (Fn)
parse_statement: starting at position 5 (Fn)
consume_token: position 5 consumed Fn
consume_token: position 6 consumed Ident
consume_token: position 7 consumed LeftParen
consume_token: position 8 consumed Ident
consume_token: position 9 consumed Comma
consume_token: position 10 consumed Ident
consume_token: position 11 consumed RightParen
consume_token: position 12 consumed LeftBrace
parse_statement: starting at position 13 (Ident(z))
consume_token: position 13 consumed Ident
consume_token: position 14 consumed PlusEqual
parse_expression: starting at position 15 (Int(1))
consume_token: position 15 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 16 consumed Semicolon
parse_statement: success - parsed assignment
parse_statement: starting at position 17 (Ident(x))
consume_token: position 17 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 17 (Ident(x))
consume_token: position 17 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 18 consumed Plus
consume_token: position 19 consumed Ident
parse_primary: success - parsed identifier (y)
parse_expression: success - parsed precedence expression
consume_token: position 20 consumed Semicolon
consume_token: position 21 consumed RightBrace
parse_statement: success - parsed function declaration
parse_program: parsing statement at position 22 (Ident(assert))
parse_statement: starting at position 22 (Ident(assert))
consume_token: position 22 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 22 (Ident(assert))
consume_token: position 22 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 23 consumed LeftParen
parse_expression: starting at position 24 (Ident(is_unit))
consume_token: position 24 consumed Ident
parse_primary: success - parsed identifier (is_unit)
consume_token: position 25 consumed LeftParen
parse_expression: starting at position 26 (Ident(add))
consume_token: position 26 consumed Ident
parse_primary: success - parsed identifier (add)
consume_token: position 27 consumed LeftParen
parse_expression: starting at position 28 (Int(2))
consume_token: position 28 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 29 consumed Comma
parse_expression: starting at position 30 (Int(3))
consume_token: position 30 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 31 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 32 consumed RightParen
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
parse_expression: starting at position 37 (Ident(z))
consume_token: position 37 consumed Ident
parse_primary: success - parsed identifier (z)
consume_token: position 38 consumed EqualEqual
consume_token: position 39 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 40 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 41 consumed Semicolon
parse_program: parsed 4 statements
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
        1: Token(Ident, "z"),
        2: Token(Equal),
        3: Token(Int, "0"),
        4: Token(Semicolon),
        5: Token(Fn),
        6: Token(Ident, "add"),
        7: Token(LeftParen),
        8: Token(Ident, "x"),
        9: Token(Comma),
        10: Token(Ident, "y"),
        11: Token(RightParen),
        12: Token(LeftBrace),
        13: Token(Ident, "z"),
        14: Token(PlusEqual),
        15: Token(Int, "1"),
        16: Token(Semicolon),
        17: Token(Ident, "x"),
        18: Token(Plus),
        19: Token(Ident, "y"),
        20: Token(Semicolon),
        21: Token(RightBrace),
        22: Token(Ident, "assert"),
        23: Token(LeftParen),
        24: Token(Ident, "is_unit"),
        25: Token(LeftParen),
        26: Token(Ident, "add"),
        27: Token(LeftParen),
        28: Token(Int, "2"),
        29: Token(Comma),
        30: Token(Int, "3"),
        31: Token(RightParen),
        32: Token(RightParen),
        33: Token(RightParen),
        34: Token(Semicolon),
        35: Token(Ident, "assert"),
        36: Token(LeftParen),
        37: Token(Ident, "z"),
        38: Token(EqualEqual),
        39: Token(Int, "1"),
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
                    "z",
                ),
                value: Some(
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
            },
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
                    statements: [
                        Assignment {
                            target: Identifier(
                                "z",
                            ),
                            op: AddAssign,
                            value: Literal(
                                Int(
                                    1,
                                ),
                            ),
                        },
                        Expression(
                            Add(
                                Identifier(
                                    "x",
                                ),
                                Identifier(
                                    "y",
                                ),
                            ),
                        ),
                    ],
                    final_expr: None,
                },
                is_pub: false,
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        FunctionCall(
                            Identifier(
                                "is_unit",
                            ),
                            [
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
                            ],
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
                                "z",
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
                        name: "z",
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
                                    statements: [
                                        RustValue(
                                            EvalAssign {
                                                name: "z",
                                                expr: RustValue(
                                                    EvalCall {
                                                        func_expr: RustValue(
                                                            EvalGetAttr {
                                                                obj_expr: RustValue(
                                                                    EvalVariable {
                                                                        name: "z",
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
                                    ],
                                    final_expr: None,
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
                                        EvalVariable {
                                            name: "is_unit",
                                        },
                                    ),
                                    args: [
                                        RustValue(
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
                                                    name: "z",
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
            ],
        },
    ),
)
```