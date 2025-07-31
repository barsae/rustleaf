# Program
Status: 🟢
Assertions: 1

```rustleaf
var name = "World";
var result = "Hello ${name}!
This is a multiline string with interpolation.";
print(result);
assert(result == "Hello World!
This is a multiline string with interpolation.");
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
consume_token: position 0 consumed Var
consume_token: position 1 consumed Ident
consume_token: position 2 consumed Equal
parse_expression: starting at position 3 (String(World))
consume_token: position 3 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 4 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 5 (Var)
parse_statement: starting at position 5 (Var)
consume_token: position 5 consumed Var
consume_token: position 6 consumed Ident
consume_token: position 7 consumed Equal
parse_expression: starting at position 8 (StringPart(Hello ))
parse_primary: success - parsing interpolated string
consume_token: position 8 consumed StringPart
consume_token: position 9 consumed InterpolationStart
parse_expression: starting at position 10 (Ident(name))
consume_token: position 10 consumed Ident
parse_primary: success - parsed identifier (name)
parse_expression: success - parsed precedence expression
consume_token: position 11 consumed InterpolationEnd
consume_token: position 12 consumed StringPart
parse_expression: success - parsed precedence expression
consume_token: position 13 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 14 (Ident(print))
parse_statement: starting at position 14 (Ident(print))
consume_token: position 14 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 14 (Ident(print))
consume_token: position 14 consumed Ident
parse_primary: success - parsed identifier (print)
consume_token: position 15 consumed LeftParen
parse_expression: starting at position 16 (Ident(result))
consume_token: position 16 consumed Ident
parse_primary: success - parsed identifier (result)
parse_expression: success - parsed precedence expression
consume_token: position 17 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 18 consumed Semicolon
parse_program: parsing statement at position 19 (Ident(assert))
parse_statement: starting at position 19 (Ident(assert))
consume_token: position 19 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 19 (Ident(assert))
consume_token: position 19 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 20 consumed LeftParen
parse_expression: starting at position 21 (Ident(result))
consume_token: position 21 consumed Ident
parse_primary: success - parsed identifier (result)
consume_token: position 22 consumed EqualEqual
consume_token: position 23 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 24 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 25 consumed Semicolon
parse_program: parsed 4 statements
Hello World!
This is a multiline string with interpolation.
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
        1: Token(Ident, "name"),
        2: Token(Equal),
        3: Token(String, "World"),
        4: Token(Semicolon),
        5: Token(Var),
        6: Token(Ident, "result"),
        7: Token(Equal),
        8: Token(StringPart, "Hello "),
        9: Token(InterpolationStart),
        10: Token(Ident, "name"),
        11: Token(InterpolationEnd),
        12: Token(StringPart, "!\nThis is a multiline string with interpolation."),
        13: Token(Semicolon),
        14: Token(Ident, "print"),
        15: Token(LeftParen),
        16: Token(Ident, "result"),
        17: Token(RightParen),
        18: Token(Semicolon),
        19: Token(Ident, "assert"),
        20: Token(LeftParen),
        21: Token(Ident, "result"),
        22: Token(EqualEqual),
        23: Token(String, "Hello World!\nThis is a multiline string with interpolation."),
        24: Token(RightParen),
        25: Token(Semicolon),
        26: Token(Eof)
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
                    "name",
                ),
                value: Some(
                    Literal(
                        String(
                            "World",
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result",
                ),
                value: Some(
                    InterpolatedString(
                        [
                            Text(
                                "Hello ",
                            ),
                            Expression(
                                Identifier(
                                    "name",
                                ),
                            ),
                            Text(
                                "!\nThis is a multiline string with interpolation.",
                            ),
                        ],
                    ),
                ),
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "print",
                    ),
                    [
                        Identifier(
                            "result",
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
                                "result",
                            ),
                            Literal(
                                String(
                                    "Hello World!\nThis is a multiline string with interpolation.",
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
                        name: "name",
                        init_expr: Some(
                            RustValue(
                                EvalLiteral {
                                    value: String(
                                        "World",
                                    ),
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "result",
                        init_expr: Some(
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalGetAttr {
                                            obj_expr: RustValue(
                                                EvalLiteral {
                                                    value: String(
                                                        "Hello ",
                                                    ),
                                                },
                                            ),
                                            attr_name: "op_add",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalCall {
                                                func_expr: RustValue(
                                                    EvalVariable {
                                                        name: "str",
                                                    },
                                                ),
                                                args: [
                                                    RustValue(
                                                        EvalVariable {
                                                            name: "name",
                                                        },
                                                    ),
                                                ],
                                            },
                                        ),
                                        RustValue(
                                            EvalLiteral {
                                                value: String(
                                                    "!\nThis is a multiline string with interpolation.",
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
                                name: "print",
                            },
                        ),
                        args: [
                            RustValue(
                                EvalVariable {
                                    name: "result",
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
                                                    name: "result",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: String(
                                                    "Hello World!\nThis is a multiline string with interpolation.",
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