# Program
Status: 🟢
Assertions: 1

```rustleaf
var x = 0;
x += 5;
assert(x == 5);
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
parse_program: parsing statement at position 5 (Ident(x))
parse_statement: starting at position 5 (Ident(x))
consume_token: position 5 consumed Ident
consume_token: position 6 consumed PlusEqual
parse_expression: starting at position 7 (Int(5))
consume_token: position 7 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 8 consumed Semicolon
parse_statement: success - parsed assignment
parse_program: parsing statement at position 9 (Ident(assert))
parse_statement: starting at position 9 (Ident(assert))
consume_token: position 9 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 9 (Ident(assert))
consume_token: position 9 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 10 consumed LeftParen
parse_expression: starting at position 11 (Ident(x))
consume_token: position 11 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 12 consumed EqualEqual
consume_token: position 13 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 14 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 15 consumed Semicolon
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
        3: Token(Int, "0"),
        4: Token(Semicolon),
        5: Token(Ident, "x"),
        6: Token(PlusEqual),
        7: Token(Int, "5"),
        8: Token(Semicolon),
        9: Token(Ident, "assert"),
        10: Token(LeftParen),
        11: Token(Ident, "x"),
        12: Token(EqualEqual),
        13: Token(Int, "5"),
        14: Token(RightParen),
        15: Token(Semicolon),
        16: Token(Eof)
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
            Assignment {
                target: Identifier(
                    "x",
                ),
                op: AddAssign,
                value: Literal(
                    Int(
                        5,
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
                                    5,
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
                                                5,
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
                                                    5,
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