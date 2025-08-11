# Program
Status: 🟢
Assertions: 1

```rustleaf
fn test(x, y) {
    x + y
}

var z = 1 | test(2);
assert(z == 3);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Fn)
parse_statement: starting at position 0 (Fn)
consume_token: position 0 consumed Fn
consume_token: position 1 consumed Ident
consume_token: position 2 consumed LeftParen
consume_token: position 3 consumed Ident
consume_token: position 4 consumed Comma
consume_token: position 5 consumed Ident
consume_token: position 6 consumed RightParen
consume_token: position 7 consumed LeftBrace
parse_statement: starting at position 8 (Ident(x))
consume_token: position 8 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 8 (Ident(x))
consume_token: position 8 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 9 consumed Plus
consume_token: position 10 consumed Ident
parse_primary: success - parsed identifier (y)
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 11
parse_expression: starting at position 8 (Ident(x))
consume_token: position 8 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 9 consumed Plus
consume_token: position 10 consumed Ident
parse_primary: success - parsed identifier (y)
parse_expression: success - parsed precedence expression
consume_token: position 11 consumed RightBrace
parse_statement: success - parsed function declaration
parse_program: parsing statement at position 12 (Var)
parse_statement: starting at position 12 (Var)
consume_token: position 12 consumed Var
consume_token: position 13 consumed Ident
consume_token: position 14 consumed Equal
parse_expression: starting at position 15 (Int(1))
consume_token: position 15 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 16 consumed Pipe
consume_token: position 17 consumed Ident
parse_primary: success - parsed identifier (test)
consume_token: position 18 consumed LeftParen
parse_expression: starting at position 19 (Int(2))
consume_token: position 19 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 20 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 21 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 22 (Ident(assert))
parse_statement: starting at position 22 (Ident(assert))
consume_token: position 22 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 22 (Ident(assert))
consume_token: position 22 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 23 consumed LeftParen
parse_expression: starting at position 24 (Ident(z))
consume_token: position 24 consumed Ident
parse_primary: success - parsed identifier (z)
consume_token: position 25 consumed EqualEqual
consume_token: position 26 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 27 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 28 consumed Semicolon
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
        0: Token(Fn),
        1: Token(Ident, "test"),
        2: Token(LeftParen),
        3: Token(Ident, "x"),
        4: Token(Comma),
        5: Token(Ident, "y"),
        6: Token(RightParen),
        7: Token(LeftBrace),
        8: Token(Ident, "x"),
        9: Token(Plus),
        10: Token(Ident, "y"),
        11: Token(RightBrace),
        12: Token(Var),
        13: Token(Ident, "z"),
        14: Token(Equal),
        15: Token(Int, "1"),
        16: Token(Pipe),
        17: Token(Ident, "test"),
        18: Token(LeftParen),
        19: Token(Int, "2"),
        20: Token(RightParen),
        21: Token(Semicolon),
        22: Token(Ident, "assert"),
        23: Token(LeftParen),
        24: Token(Ident, "z"),
        25: Token(EqualEqual),
        26: Token(Int, "3"),
        27: Token(RightParen),
        28: Token(Semicolon),
        29: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            FnDecl {
                name: "test",
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
                    statements: [],
                    final_expr: Some(
                        Add(
                            Identifier(
                                "x",
                            ),
                            Identifier(
                                "y",
                            ),
                        ),
                    ),
                },
                is_pub: false,
            },
            VarDecl {
                pattern: Variable(
                    "z",
                ),
                value: Some(
                    Pipe(
                        Literal(
                            Int(
                                1,
                            ),
                        ),
                        FunctionCall(
                            Identifier(
                                "test",
                            ),
                            [
                                Literal(
                                    Int(
                                        2,
                                    ),
                                ),
                            ],
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
                                "z",
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
    RustValue(<unknown>),
)
```