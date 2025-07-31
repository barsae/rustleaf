# Program
Status: 🟢
Assertions: 2

```rustleaf
var x = 1;
var inner_x;
var outer_x;
{
    var x = 2;
    inner_x = x;
}
outer_x = x;
assert(inner_x == 2);
assert(outer_x == 1);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
consume_token: position 0 consumed Var
consume_token: position 1 consumed Ident
consume_token: position 2 consumed Equal
parse_expression: starting at position 3 (Int(1))
consume_token: position 3 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 4 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 5 (Var)
parse_statement: starting at position 5 (Var)
consume_token: position 5 consumed Var
consume_token: position 6 consumed Ident
consume_token: position 7 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 8 (Var)
parse_statement: starting at position 8 (Var)
consume_token: position 8 consumed Var
consume_token: position 9 consumed Ident
consume_token: position 10 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 11 (LeftBrace)
parse_statement: starting at position 11 (LeftBrace)
parse_expression: starting at position 11 (LeftBrace)
parse_primary: success - parsing block or dict
consume_token: position 11 consumed LeftBrace
parse_primary: failed - no matching primary expression for Var
parse_statement: starting at position 12 (Var)
consume_token: position 12 consumed Var
consume_token: position 13 consumed Ident
consume_token: position 14 consumed Equal
parse_expression: starting at position 15 (Int(2))
consume_token: position 15 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 16 consumed Semicolon
parse_statement: success - parsed var declaration
parse_statement: starting at position 17 (Ident(inner_x))
consume_token: position 17 consumed Ident
consume_token: position 18 consumed Equal
parse_expression: starting at position 19 (Ident(x))
consume_token: position 19 consumed Ident
parse_primary: success - parsed identifier (x)
parse_expression: success - parsed precedence expression
consume_token: position 20 consumed Semicolon
parse_statement: success - parsed assignment
consume_token: position 21 consumed RightBrace
parse_expression: success - parsed precedence expression
parse_statement: success - parsed block-like expression statement
parse_program: parsing statement at position 22 (Ident(outer_x))
parse_statement: starting at position 22 (Ident(outer_x))
consume_token: position 22 consumed Ident
consume_token: position 23 consumed Equal
parse_expression: starting at position 24 (Ident(x))
consume_token: position 24 consumed Ident
parse_primary: success - parsed identifier (x)
parse_expression: success - parsed precedence expression
consume_token: position 25 consumed Semicolon
parse_statement: success - parsed assignment
parse_program: parsing statement at position 26 (Ident(assert))
parse_statement: starting at position 26 (Ident(assert))
consume_token: position 26 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 26 (Ident(assert))
consume_token: position 26 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 27 consumed LeftParen
parse_expression: starting at position 28 (Ident(inner_x))
consume_token: position 28 consumed Ident
parse_primary: success - parsed identifier (inner_x)
consume_token: position 29 consumed EqualEqual
consume_token: position 30 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 31 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 32 consumed Semicolon
parse_program: parsing statement at position 33 (Ident(assert))
parse_statement: starting at position 33 (Ident(assert))
consume_token: position 33 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 33 (Ident(assert))
consume_token: position 33 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 34 consumed LeftParen
parse_expression: starting at position 35 (Ident(outer_x))
consume_token: position 35 consumed Ident
parse_primary: success - parsed identifier (outer_x)
consume_token: position 36 consumed EqualEqual
consume_token: position 37 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 38 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 39 consumed Semicolon
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
        3: Token(Int, "1"),
        4: Token(Semicolon),
        5: Token(Var),
        6: Token(Ident, "inner_x"),
        7: Token(Semicolon),
        8: Token(Var),
        9: Token(Ident, "outer_x"),
        10: Token(Semicolon),
        11: Token(LeftBrace),
        12: Token(Var),
        13: Token(Ident, "x"),
        14: Token(Equal),
        15: Token(Int, "2"),
        16: Token(Semicolon),
        17: Token(Ident, "inner_x"),
        18: Token(Equal),
        19: Token(Ident, "x"),
        20: Token(Semicolon),
        21: Token(RightBrace),
        22: Token(Ident, "outer_x"),
        23: Token(Equal),
        24: Token(Ident, "x"),
        25: Token(Semicolon),
        26: Token(Ident, "assert"),
        27: Token(LeftParen),
        28: Token(Ident, "inner_x"),
        29: Token(EqualEqual),
        30: Token(Int, "2"),
        31: Token(RightParen),
        32: Token(Semicolon),
        33: Token(Ident, "assert"),
        34: Token(LeftParen),
        35: Token(Ident, "outer_x"),
        36: Token(EqualEqual),
        37: Token(Int, "1"),
        38: Token(RightParen),
        39: Token(Semicolon),
        40: Token(Eof)
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
                            1,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "inner_x",
                ),
                value: None,
            },
            VarDecl {
                pattern: Variable(
                    "outer_x",
                ),
                value: None,
            },
            Expression(
                Block(
                    Block {
                        statements: [
                            VarDecl {
                                pattern: Variable(
                                    "x",
                                ),
                                value: Some(
                                    Literal(
                                        Int(
                                            2,
                                        ),
                                    ),
                                ),
                            },
                            Assignment {
                                target: Identifier(
                                    "inner_x",
                                ),
                                op: Assign,
                                value: Identifier(
                                    "x",
                                ),
                            },
                        ],
                        final_expr: None,
                    },
                ),
            ),
            Assignment {
                target: Identifier(
                    "outer_x",
                ),
                op: Assign,
                value: Identifier(
                    "x",
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
                                "inner_x",
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Identifier(
                                "outer_x",
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
                        name: "x",
                        init_expr: Some(
                            RustValue(
                                EvalLiteral {
                                    value: Int(
                                        1,
                                    ),
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "inner_x",
                        init_expr: None,
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "outer_x",
                        init_expr: None,
                    },
                ),
                RustValue(
                    EvalBlock {
                        statements: [
                            RustValue(
                                EvalDeclare {
                                    name: "x",
                                    init_expr: Some(
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    2,
                                                ),
                                            },
                                        ),
                                    ),
                                },
                            ),
                            RustValue(
                                EvalAssign {
                                    name: "inner_x",
                                    expr: RustValue(
                                        EvalVariable {
                                            name: "x",
                                        },
                                    ),
                                },
                            ),
                        ],
                        final_expr: None,
                    },
                ),
                RustValue(
                    EvalAssign {
                        name: "outer_x",
                        expr: RustValue(
                            EvalVariable {
                                name: "x",
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
                                                    name: "inner_x",
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
                                                    name: "outer_x",
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