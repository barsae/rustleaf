# Program
Status: 🟢
Assertions: 1

```rustleaf
var x = {};
assert(x is Dict);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
consume_token: position 0 consumed Var
consume_token: position 1 consumed Ident
consume_token: position 2 consumed Equal
parse_expression: starting at position 3 (LeftBrace)
parse_primary: success - parsing block or dict
consume_token: position 3 consumed LeftBrace
consume_token: position 4 consumed RightBrace
parse_expression: success - parsed precedence expression
consume_token: position 5 consumed Semicolon
parse_statement: success - parsed var declaration
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
consume_token: position 9 consumed Is
consume_token: position 10 consumed Ident
parse_primary: success - parsed identifier (Dict)
parse_expression: success - parsed precedence expression
consume_token: position 11 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 12 consumed Semicolon
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
        1: Token(Ident, "x"),
        2: Token(Equal),
        3: Token(LeftBrace),
        4: Token(RightBrace),
        5: Token(Semicolon),
        6: Token(Ident, "assert"),
        7: Token(LeftParen),
        8: Token(Ident, "x"),
        9: Token(Is),
        10: Token(Ident, "Dict"),
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
                    Dict(
                        [],
                    ),
                ),
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Is(
                            Identifier(
                                "x",
                            ),
                            Identifier(
                                "Dict",
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
                                EvalDict {
                                    pairs: [],
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
                                EvalIs {
                                    left: RustValue(
                                        EvalVariable {
                                            name: "x",
                                        },
                                    ),
                                    right: RustValue(
                                        EvalVariable {
                                            name: "Dict",
                                        },
                                    ),
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