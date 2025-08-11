# Program
Status: 🟢
Assertions: 3

```rustleaf
assert(-42 == -42);
assert(not true == false);
assert(not false == true);
// Bitwise NOT (~) operator has been removed from RustLeaf
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
parse_expression: starting at position 2 (Minus)
consume_token: position 2 consumed Minus
consume_token: position 3 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 4 consumed EqualEqual
consume_token: position 5 consumed Minus
consume_token: position 6 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 7 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 8 consumed Semicolon
parse_program: parsing statement at position 9 (Ident(assert))
parse_statement: starting at position 9 (Ident(assert))
consume_token: position 9 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 9 (Ident(assert))
consume_token: position 9 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 10 consumed LeftParen
parse_expression: starting at position 11 (Not)
consume_token: position 11 consumed Not
consume_token: position 12 consumed True
parse_primary: success - parsed boolean literal (true)
consume_token: position 13 consumed EqualEqual
consume_token: position 14 consumed False
parse_primary: success - parsed boolean literal (false)
parse_expression: success - parsed precedence expression
consume_token: position 15 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 16 consumed Semicolon
parse_program: parsing statement at position 17 (Ident(assert))
parse_statement: starting at position 17 (Ident(assert))
consume_token: position 17 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 17 (Ident(assert))
consume_token: position 17 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 18 consumed LeftParen
parse_expression: starting at position 19 (Not)
consume_token: position 19 consumed Not
consume_token: position 20 consumed False
parse_primary: success - parsed boolean literal (false)
consume_token: position 21 consumed EqualEqual
consume_token: position 22 consumed True
parse_primary: success - parsed boolean literal (true)
parse_expression: success - parsed precedence expression
consume_token: position 23 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 24 consumed Semicolon
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
        2: Token(Minus),
        3: Token(Int, "42"),
        4: Token(EqualEqual),
        5: Token(Minus),
        6: Token(Int, "42"),
        7: Token(RightParen),
        8: Token(Semicolon),
        9: Token(Ident, "assert"),
        10: Token(LeftParen),
        11: Token(Not),
        12: Token(True),
        13: Token(EqualEqual),
        14: Token(False),
        15: Token(RightParen),
        16: Token(Semicolon),
        17: Token(Ident, "assert"),
        18: Token(LeftParen),
        19: Token(Not),
        20: Token(False),
        21: Token(EqualEqual),
        22: Token(True),
        23: Token(RightParen),
        24: Token(Semicolon),
        25: Token(Eof)
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
                            Neg(
                                Literal(
                                    Int(
                                        42,
                                    ),
                                ),
                            ),
                            Neg(
                                Literal(
                                    Int(
                                        42,
                                    ),
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
                            Not(
                                Literal(
                                    Bool(
                                        true,
                                    ),
                                ),
                            ),
                            Literal(
                                Bool(
                                    false,
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
                            Not(
                                Literal(
                                    Bool(
                                        false,
                                    ),
                                ),
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