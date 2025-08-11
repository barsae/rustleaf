# Program
Status: 🟢
Assertions: 1

```rustleaf
assert((loop {
    break 42;
}) == 42);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Ident(assert))
parse_statement: starting at position 0 (Ident(assert))
consume_token: position 0 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 0 (Ident(assert))
consume_token: position 0 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 1 consumed LeftParen
parse_expression: starting at position 2 (LeftParen)
consume_token: position 2 consumed LeftParen
parse_primary: success - parsing parenthesized expression
parse_expression: starting at position 3 (Loop)
consume_token: position 3 consumed Loop
parse_primary: success - parsing loop expression
consume_token: position 4 consumed LeftBrace
parse_statement: starting at position 5 (Break)
consume_token: position 5 consumed Break
parse_expression: starting at position 6 (Int(42))
consume_token: position 6 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 7 consumed Semicolon
parse_statement: success - parsed break statement
consume_token: position 8 consumed RightBrace
parse_expression: success - parsed precedence expression
consume_token: position 9 consumed RightParen
consume_token: position 10 consumed EqualEqual
consume_token: position 11 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 12 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 13 consumed Semicolon
parse_program: parsed 1 statements
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
        0: Token(Ident, "assert"),
        1: Token(LeftParen),
        2: Token(LeftParen),
        3: Token(Loop),
        4: Token(LeftBrace),
        5: Token(Break),
        6: Token(Int, "42"),
        7: Token(Semicolon),
        8: Token(RightBrace),
        9: Token(RightParen),
        10: Token(EqualEqual),
        11: Token(Int, "42"),
        12: Token(RightParen),
        13: Token(Semicolon),
        14: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Loop {
                                body: Block {
                                    statements: [
                                        Break(
                                            Some(
                                                Literal(
                                                    Int(
                                                        42,
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ],
                                    final_expr: None,
                                },
                            },
                            Literal(
                                Int(
                                    42,
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