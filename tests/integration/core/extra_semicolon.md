# Program
Status: 🟢
Assertions: 1

```rustleaf
var x = 5;;  // Extra semicolon
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
parse_expression: starting at position 3 (Int(5))
consume_token: position 3 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 4 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 5 (Semicolon)
parse_statement: starting at position 5 (Semicolon)
consume_token: position 5 consumed Semicolon
parse_statement: success - parsed empty statement
parse_program: parsing statement at position 6 (Ident(assert))
parse_statement: starting at position 6 (Ident(assert))
consume_token: position 6 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 6 (Ident(assert))
consume_token: position 6 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 7 consumed LeftParen
parse_expression: starting at position 8 (Ident(x))
consume_token: position 8 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 9 consumed EqualEqual
consume_token: position 10 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 11 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 12 consumed Semicolon
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
        3: Token(Int, "5"),
        4: Token(Semicolon),
        5: Token(Semicolon),
        6: Token(Ident, "assert"),
        7: Token(LeftParen),
        8: Token(Ident, "x"),
        9: Token(EqualEqual),
        10: Token(Int, "5"),
        11: Token(RightParen),
        12: Token(Semicolon),
        13: Token(Eof)
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
                            5,
                        ),
                    ),
                ),
            },
            Empty,
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
                                        5,
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