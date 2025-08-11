# Program
Status: 🟢
Assertions: 7

```rustleaf
assert(sqrt(4) == 2.0);
assert(abs(-5) == 5);
assert(floor(3.7) == 3);
assert(ceil(3.2) == 4);
assert(round(3.5) == 4);
assert(min(5, 3) == 3);
assert(max(5, 3) == 5);
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
parse_expression: starting at position 2 (Ident(sqrt))
consume_token: position 2 consumed Ident
parse_primary: success - parsed identifier (sqrt)
consume_token: position 3 consumed LeftParen
parse_expression: starting at position 4 (Int(4))
consume_token: position 4 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 5 consumed RightParen
consume_token: position 6 consumed EqualEqual
consume_token: position 7 consumed Float
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 8 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 9 consumed Semicolon
parse_program: parsing statement at position 10 (Ident(assert))
parse_statement: starting at position 10 (Ident(assert))
consume_token: position 10 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 10 (Ident(assert))
consume_token: position 10 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 11 consumed LeftParen
parse_expression: starting at position 12 (Ident(abs))
consume_token: position 12 consumed Ident
parse_primary: success - parsed identifier (abs)
consume_token: position 13 consumed LeftParen
parse_expression: starting at position 14 (Minus)
consume_token: position 14 consumed Minus
consume_token: position 15 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 16 consumed RightParen
consume_token: position 17 consumed EqualEqual
consume_token: position 18 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 19 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 20 consumed Semicolon
parse_program: parsing statement at position 21 (Ident(assert))
parse_statement: starting at position 21 (Ident(assert))
consume_token: position 21 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 21 (Ident(assert))
consume_token: position 21 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 22 consumed LeftParen
parse_expression: starting at position 23 (Ident(floor))
consume_token: position 23 consumed Ident
parse_primary: success - parsed identifier (floor)
consume_token: position 24 consumed LeftParen
parse_expression: starting at position 25 (Float(3.7))
consume_token: position 25 consumed Float
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 26 consumed RightParen
consume_token: position 27 consumed EqualEqual
consume_token: position 28 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 29 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 30 consumed Semicolon
parse_program: parsing statement at position 31 (Ident(assert))
parse_statement: starting at position 31 (Ident(assert))
consume_token: position 31 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 31 (Ident(assert))
consume_token: position 31 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 32 consumed LeftParen
parse_expression: starting at position 33 (Ident(ceil))
consume_token: position 33 consumed Ident
parse_primary: success - parsed identifier (ceil)
consume_token: position 34 consumed LeftParen
parse_expression: starting at position 35 (Float(3.2))
consume_token: position 35 consumed Float
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 36 consumed RightParen
consume_token: position 37 consumed EqualEqual
consume_token: position 38 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 39 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 40 consumed Semicolon
parse_program: parsing statement at position 41 (Ident(assert))
parse_statement: starting at position 41 (Ident(assert))
consume_token: position 41 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 41 (Ident(assert))
consume_token: position 41 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 42 consumed LeftParen
parse_expression: starting at position 43 (Ident(round))
consume_token: position 43 consumed Ident
parse_primary: success - parsed identifier (round)
consume_token: position 44 consumed LeftParen
parse_expression: starting at position 45 (Float(3.5))
consume_token: position 45 consumed Float
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 46 consumed RightParen
consume_token: position 47 consumed EqualEqual
consume_token: position 48 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 49 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 50 consumed Semicolon
parse_program: parsing statement at position 51 (Ident(assert))
parse_statement: starting at position 51 (Ident(assert))
consume_token: position 51 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 51 (Ident(assert))
consume_token: position 51 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 52 consumed LeftParen
parse_expression: starting at position 53 (Ident(min))
consume_token: position 53 consumed Ident
parse_primary: success - parsed identifier (min)
consume_token: position 54 consumed LeftParen
parse_expression: starting at position 55 (Int(5))
consume_token: position 55 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 56 consumed Comma
parse_expression: starting at position 57 (Int(3))
consume_token: position 57 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 58 consumed RightParen
consume_token: position 59 consumed EqualEqual
consume_token: position 60 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 61 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 62 consumed Semicolon
parse_program: parsing statement at position 63 (Ident(assert))
parse_statement: starting at position 63 (Ident(assert))
consume_token: position 63 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 63 (Ident(assert))
consume_token: position 63 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 64 consumed LeftParen
parse_expression: starting at position 65 (Ident(max))
consume_token: position 65 consumed Ident
parse_primary: success - parsed identifier (max)
consume_token: position 66 consumed LeftParen
parse_expression: starting at position 67 (Int(5))
consume_token: position 67 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 68 consumed Comma
parse_expression: starting at position 69 (Int(3))
consume_token: position 69 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 70 consumed RightParen
consume_token: position 71 consumed EqualEqual
consume_token: position 72 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 73 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 74 consumed Semicolon
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
        0: Token(Ident, "assert"),
        1: Token(LeftParen),
        2: Token(Ident, "sqrt"),
        3: Token(LeftParen),
        4: Token(Int, "4"),
        5: Token(RightParen),
        6: Token(EqualEqual),
        7: Token(Float, "2.0"),
        8: Token(RightParen),
        9: Token(Semicolon),
        10: Token(Ident, "assert"),
        11: Token(LeftParen),
        12: Token(Ident, "abs"),
        13: Token(LeftParen),
        14: Token(Minus),
        15: Token(Int, "5"),
        16: Token(RightParen),
        17: Token(EqualEqual),
        18: Token(Int, "5"),
        19: Token(RightParen),
        20: Token(Semicolon),
        21: Token(Ident, "assert"),
        22: Token(LeftParen),
        23: Token(Ident, "floor"),
        24: Token(LeftParen),
        25: Token(Float, "3.7"),
        26: Token(RightParen),
        27: Token(EqualEqual),
        28: Token(Int, "3"),
        29: Token(RightParen),
        30: Token(Semicolon),
        31: Token(Ident, "assert"),
        32: Token(LeftParen),
        33: Token(Ident, "ceil"),
        34: Token(LeftParen),
        35: Token(Float, "3.2"),
        36: Token(RightParen),
        37: Token(EqualEqual),
        38: Token(Int, "4"),
        39: Token(RightParen),
        40: Token(Semicolon),
        41: Token(Ident, "assert"),
        42: Token(LeftParen),
        43: Token(Ident, "round"),
        44: Token(LeftParen),
        45: Token(Float, "3.5"),
        46: Token(RightParen),
        47: Token(EqualEqual),
        48: Token(Int, "4"),
        49: Token(RightParen),
        50: Token(Semicolon),
        51: Token(Ident, "assert"),
        52: Token(LeftParen),
        53: Token(Ident, "min"),
        54: Token(LeftParen),
        55: Token(Int, "5"),
        56: Token(Comma),
        57: Token(Int, "3"),
        58: Token(RightParen),
        59: Token(EqualEqual),
        60: Token(Int, "3"),
        61: Token(RightParen),
        62: Token(Semicolon),
        63: Token(Ident, "assert"),
        64: Token(LeftParen),
        65: Token(Ident, "max"),
        66: Token(LeftParen),
        67: Token(Int, "5"),
        68: Token(Comma),
        69: Token(Int, "3"),
        70: Token(RightParen),
        71: Token(EqualEqual),
        72: Token(Int, "5"),
        73: Token(RightParen),
        74: Token(Semicolon),
        75: Token(Eof)
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
                            FunctionCall(
                                Identifier(
                                    "sqrt",
                                ),
                                [
                                    Literal(
                                        Int(
                                            4,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Float(
                                    2.0,
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
                            FunctionCall(
                                Identifier(
                                    "abs",
                                ),
                                [
                                    Neg(
                                        Literal(
                                            Int(
                                                5,
                                            ),
                                        ),
                                    ),
                                ],
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            FunctionCall(
                                Identifier(
                                    "floor",
                                ),
                                [
                                    Literal(
                                        Float(
                                            3.7,
                                        ),
                                    ),
                                ],
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            FunctionCall(
                                Identifier(
                                    "ceil",
                                ),
                                [
                                    Literal(
                                        Float(
                                            3.2,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    4,
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
                            FunctionCall(
                                Identifier(
                                    "round",
                                ),
                                [
                                    Literal(
                                        Float(
                                            3.5,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    4,
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
                            FunctionCall(
                                Identifier(
                                    "min",
                                ),
                                [
                                    Literal(
                                        Int(
                                            5,
                                        ),
                                    ),
                                    Literal(
                                        Int(
                                            3,
                                        ),
                                    ),
                                ],
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            FunctionCall(
                                Identifier(
                                    "max",
                                ),
                                [
                                    Literal(
                                        Int(
                                            5,
                                        ),
                                    ),
                                    Literal(
                                        Int(
                                            3,
                                        ),
                                    ),
                                ],
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
    RustValue(<unknown>),
)
```