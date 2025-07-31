# Program
Status: 🟢
Assertions: 4

```rustleaf
var x = 42;
var y = "hello";
var z = true;
assert(x == 42);
assert(y == "hello");
assert(z == true);
assert(x + 8 == 50);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
consume_token: position 0 consumed Var
consume_token: position 1 consumed Ident
consume_token: position 2 consumed Equal
parse_expression: starting at position 3 (Int(42))
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
parse_expression: starting at position 8 (String(hello))
consume_token: position 8 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 9 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 10 (Var)
parse_statement: starting at position 10 (Var)
consume_token: position 10 consumed Var
consume_token: position 11 consumed Ident
consume_token: position 12 consumed Equal
parse_expression: starting at position 13 (True)
consume_token: position 13 consumed True
parse_primary: success - parsed boolean literal (true)
parse_expression: success - parsed precedence expression
consume_token: position 14 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 15 (Ident(assert))
parse_statement: starting at position 15 (Ident(assert))
consume_token: position 15 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 15 (Ident(assert))
consume_token: position 15 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 16 consumed LeftParen
parse_expression: starting at position 17 (Ident(x))
consume_token: position 17 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 18 consumed EqualEqual
consume_token: position 19 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 20 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 21 consumed Semicolon
parse_program: parsing statement at position 22 (Ident(assert))
parse_statement: starting at position 22 (Ident(assert))
consume_token: position 22 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 22 (Ident(assert))
consume_token: position 22 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 23 consumed LeftParen
parse_expression: starting at position 24 (Ident(y))
consume_token: position 24 consumed Ident
parse_primary: success - parsed identifier (y)
consume_token: position 25 consumed EqualEqual
consume_token: position 26 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 27 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 28 consumed Semicolon
parse_program: parsing statement at position 29 (Ident(assert))
parse_statement: starting at position 29 (Ident(assert))
consume_token: position 29 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 29 (Ident(assert))
consume_token: position 29 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 30 consumed LeftParen
parse_expression: starting at position 31 (Ident(z))
consume_token: position 31 consumed Ident
parse_primary: success - parsed identifier (z)
consume_token: position 32 consumed EqualEqual
consume_token: position 33 consumed True
parse_primary: success - parsed boolean literal (true)
parse_expression: success - parsed precedence expression
consume_token: position 34 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 35 consumed Semicolon
parse_program: parsing statement at position 36 (Ident(assert))
parse_statement: starting at position 36 (Ident(assert))
consume_token: position 36 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 36 (Ident(assert))
consume_token: position 36 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 37 consumed LeftParen
parse_expression: starting at position 38 (Ident(x))
consume_token: position 38 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 39 consumed Plus
consume_token: position 40 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 41 consumed EqualEqual
consume_token: position 42 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 43 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 44 consumed Semicolon
parse_program: parsed 7 statements
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
        3: Token(Int, "42"),
        4: Token(Semicolon),
        5: Token(Var),
        6: Token(Ident, "y"),
        7: Token(Equal),
        8: Token(String, "hello"),
        9: Token(Semicolon),
        10: Token(Var),
        11: Token(Ident, "z"),
        12: Token(Equal),
        13: Token(True),
        14: Token(Semicolon),
        15: Token(Ident, "assert"),
        16: Token(LeftParen),
        17: Token(Ident, "x"),
        18: Token(EqualEqual),
        19: Token(Int, "42"),
        20: Token(RightParen),
        21: Token(Semicolon),
        22: Token(Ident, "assert"),
        23: Token(LeftParen),
        24: Token(Ident, "y"),
        25: Token(EqualEqual),
        26: Token(String, "hello"),
        27: Token(RightParen),
        28: Token(Semicolon),
        29: Token(Ident, "assert"),
        30: Token(LeftParen),
        31: Token(Ident, "z"),
        32: Token(EqualEqual),
        33: Token(True),
        34: Token(RightParen),
        35: Token(Semicolon),
        36: Token(Ident, "assert"),
        37: Token(LeftParen),
        38: Token(Ident, "x"),
        39: Token(Plus),
        40: Token(Int, "8"),
        41: Token(EqualEqual),
        42: Token(Int, "50"),
        43: Token(RightParen),
        44: Token(Semicolon),
        45: Token(Eof)
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
                            42,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "y",
                ),
                value: Some(
                    Literal(
                        String(
                            "hello",
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "z",
                ),
                value: Some(
                    Literal(
                        Bool(
                            true,
                        ),
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
                                "x",
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
                                "y",
                            ),
                            Literal(
                                String(
                                    "hello",
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
                                "z",
                            ),
                            Literal(
                                Bool(
                                    true,
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
                            Add(
                                Identifier(
                                    "x",
                                ),
                                Literal(
                                    Int(
                                        8,
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    50,
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
                                        42,
                                    ),
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "y",
                        init_expr: Some(
                            RustValue(
                                EvalLiteral {
                                    value: String(
                                        "hello",
                                    ),
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "z",
                        init_expr: Some(
                            RustValue(
                                EvalLiteral {
                                    value: Bool(
                                        true,
                                    ),
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
                                                    name: "y",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: String(
                                                    "hello",
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
                                                    name: "z",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Bool(
                                                    true,
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
                                                                    8,
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
                                                    50,
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