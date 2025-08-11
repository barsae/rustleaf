# Program
Status: 🟢
Assertions: 3

```rustleaf
assert(true);
assert(1 == 1);
assert(10 + 5 == 15, "Math should work");
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
parse_expression: starting at position 2 (True)
consume_token: position 2 consumed True
parse_primary: success - parsed boolean literal (true)
parse_expression: success - parsed precedence expression
consume_token: position 3 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 4 consumed Semicolon
parse_program: parsing statement at position 5 (Ident(assert))
parse_statement: starting at position 5 (Ident(assert))
consume_token: position 5 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 5 (Ident(assert))
consume_token: position 5 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 6 consumed LeftParen
parse_expression: starting at position 7 (Int(1))
consume_token: position 7 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 8 consumed EqualEqual
consume_token: position 9 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 10 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 11 consumed Semicolon
parse_program: parsing statement at position 12 (Ident(assert))
parse_statement: starting at position 12 (Ident(assert))
consume_token: position 12 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 12 (Ident(assert))
consume_token: position 12 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 13 consumed LeftParen
parse_expression: starting at position 14 (Int(10))
consume_token: position 14 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 15 consumed Plus
consume_token: position 16 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 17 consumed EqualEqual
consume_token: position 18 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 19 consumed Comma
parse_expression: starting at position 20 (String(Math should work))
consume_token: position 20 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 21 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 22 consumed Semicolon
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
        0: Token(Ident, "assert"),
        1: Token(LeftParen),
        2: Token(True),
        3: Token(RightParen),
        4: Token(Semicolon),
        5: Token(Ident, "assert"),
        6: Token(LeftParen),
        7: Token(Int, "1"),
        8: Token(EqualEqual),
        9: Token(Int, "1"),
        10: Token(RightParen),
        11: Token(Semicolon),
        12: Token(Ident, "assert"),
        13: Token(LeftParen),
        14: Token(Int, "10"),
        15: Token(Plus),
        16: Token(Int, "5"),
        17: Token(EqualEqual),
        18: Token(Int, "15"),
        19: Token(Comma),
        20: Token(String, "Math should work"),
        21: Token(RightParen),
        22: Token(Semicolon),
        23: Token(Eof)
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
                        Literal(
                            Bool(
                                true,
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
                            Literal(
                                Int(
                                    1,
                                ),
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Add(
                                Literal(
                                    Int(
                                        10,
                                    ),
                                ),
                                Literal(
                                    Int(
                                        5,
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    15,
                                ),
                            ),
                        ),
                        Literal(
                            String(
                                "Math should work",
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