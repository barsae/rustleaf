# Program
Status: 🟢
Assertions: 1

```rustleaf
var x = 1;
loop {
    x += 1;
    if (x < 2) {
        continue;
    }
    break;
}
assert(x == 2);
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
parse_program: parsing statement at position 5 (Loop)
parse_statement: starting at position 5 (Loop)
parse_expression: starting at position 5 (Loop)
consume_token: position 5 consumed Loop
parse_primary: success - parsing loop expression
consume_token: position 6 consumed LeftBrace
parse_statement: starting at position 7 (Ident(x))
consume_token: position 7 consumed Ident
consume_token: position 8 consumed PlusEqual
parse_expression: starting at position 9 (Int(1))
consume_token: position 9 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 10 consumed Semicolon
parse_statement: success - parsed assignment
parse_statement: starting at position 11 (If)
parse_expression: starting at position 11 (If)
consume_token: position 11 consumed If
parse_primary: success - parsing if expression
parse_expression: starting at position 12 (LeftParen)
consume_token: position 12 consumed LeftParen
parse_primary: success - parsing parenthesized expression
parse_expression: starting at position 13 (Ident(x))
consume_token: position 13 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 14 consumed Less
consume_token: position 15 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 16 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 17 consumed LeftBrace
parse_statement: starting at position 18 (Continue)
consume_token: position 18 consumed Continue
consume_token: position 19 consumed Semicolon
parse_statement: success - parsed continue statement
consume_token: position 20 consumed RightBrace
parse_expression: success - parsed precedence expression
parse_statement: success - parsed block-like expression statement
parse_statement: starting at position 21 (Break)
consume_token: position 21 consumed Break
consume_token: position 22 consumed Semicolon
parse_statement: success - parsed break statement
consume_token: position 23 consumed RightBrace
parse_expression: success - parsed precedence expression
parse_statement: success - parsed block-like expression statement
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
        1: Token(Ident, "x"),
        2: Token(Equal),
        3: Token(Int, "1"),
        4: Token(Semicolon),
        5: Token(Loop),
        6: Token(LeftBrace),
        7: Token(Ident, "x"),
        8: Token(PlusEqual),
        9: Token(Int, "1"),
        10: Token(Semicolon),
        11: Token(If),
        12: Token(LeftParen),
        13: Token(Ident, "x"),
        14: Token(Less),
        15: Token(Int, "2"),
        16: Token(RightParen),
        17: Token(LeftBrace),
        18: Token(Continue),
        19: Token(Semicolon),
        20: Token(RightBrace),
        21: Token(Break),
        22: Token(Semicolon),
        23: Token(RightBrace),
        24: Token(Ident, "assert"),
        25: Token(LeftParen),
        26: Token(Ident, "x"),
        27: Token(EqualEqual),
        28: Token(Int, "2"),
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
            Expression(
                Loop {
                    body: Block {
                        statements: [
                            Assignment {
                                target: Identifier(
                                    "x",
                                ),
                                op: AddAssign,
                                value: Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                            },
                            Expression(
                                If {
                                    condition: Lt(
                                        Identifier(
                                            "x",
                                        ),
                                        Literal(
                                            Int(
                                                2,
                                            ),
                                        ),
                                    ),
                                    then_expr: Block {
                                        statements: [
                                            Continue,
                                        ],
                                        final_expr: None,
                                    },
                                    else_expr: None,
                                },
                            ),
                            Break(
                                None,
                            ),
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
                    EvalLoop {
                        body: RustValue(
                            EvalBlock {
                                statements: [
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
                                    RustValue(
                                        EvalIf {
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
                                                                    2,
                                                                ),
                                                            },
                                                        ),
                                                    ],
                                                },
                                            ),
                                            then_expr: RustValue(
                                                EvalBlock {
                                                    statements: [
                                                        RustValue(
                                                            EvalContinue,
                                                        ),
                                                    ],
                                                    final_expr: None,
                                                },
                                            ),
                                            else_expr: None,
                                        },
                                    ),
                                    RustValue(
                                        EvalBreak {
                                            expr: None,
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