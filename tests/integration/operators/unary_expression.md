# Program
Status: 🟢
Assertions: 4

```rustleaf
var positive = 42;
var negative = -positive;
var double_neg = -negative;
assert(negative == -42);
assert(double_neg == 42);
assert(-100 == -100);
assert(-(5 + 3) == -8);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
consume_token: position 0 consumed Var
consume_token: position 1 consumed Ident
consume_token: position 2 consumed Equal
parse_expression: starting at position 3 (Int(42))
consume_token: position 3 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 4 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 5 (Var)
parse_statement: starting at position 5 (Var)
consume_token: position 5 consumed Var
consume_token: position 6 consumed Ident
consume_token: position 7 consumed Equal
parse_expression: starting at position 8 (Minus)
consume_token: position 8 consumed Minus
consume_token: position 9 consumed Ident
parse_primary: success - parsed identifier (positive)
parse_expression: success - parsed precedence expression
consume_token: position 10 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 11 (Var)
parse_statement: starting at position 11 (Var)
consume_token: position 11 consumed Var
consume_token: position 12 consumed Ident
consume_token: position 13 consumed Equal
parse_expression: starting at position 14 (Minus)
consume_token: position 14 consumed Minus
consume_token: position 15 consumed Ident
parse_primary: success - parsed identifier (negative)
parse_expression: success - parsed precedence expression
consume_token: position 16 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 17 (Ident(assert))
parse_statement: starting at position 17 (Ident(assert))
consume_token: position 17 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 17 (Ident(assert))
consume_token: position 17 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 18 consumed LeftParen
parse_expression: starting at position 19 (Ident(negative))
consume_token: position 19 consumed Ident
parse_primary: success - parsed identifier (negative)
consume_token: position 20 consumed EqualEqual
consume_token: position 21 consumed Minus
consume_token: position 22 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 23 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 24 consumed Semicolon
parse_program: parsing statement at position 25 (Ident(assert))
parse_statement: starting at position 25 (Ident(assert))
consume_token: position 25 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 25 (Ident(assert))
consume_token: position 25 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 26 consumed LeftParen
parse_expression: starting at position 27 (Ident(double_neg))
consume_token: position 27 consumed Ident
parse_primary: success - parsed identifier (double_neg)
consume_token: position 28 consumed EqualEqual
consume_token: position 29 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 30 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 31 consumed Semicolon
parse_program: parsing statement at position 32 (Ident(assert))
parse_statement: starting at position 32 (Ident(assert))
consume_token: position 32 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 32 (Ident(assert))
consume_token: position 32 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 33 consumed LeftParen
parse_expression: starting at position 34 (Minus)
consume_token: position 34 consumed Minus
consume_token: position 35 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 36 consumed EqualEqual
consume_token: position 37 consumed Minus
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
parse_expression: starting at position 43 (Minus)
consume_token: position 43 consumed Minus
consume_token: position 44 consumed LeftParen
parse_primary: success - parsing parenthesized expression
parse_expression: starting at position 45 (Int(5))
consume_token: position 45 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 46 consumed Plus
consume_token: position 47 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 48 consumed RightParen
consume_token: position 49 consumed EqualEqual
consume_token: position 50 consumed Minus
consume_token: position 51 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 52 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 53 consumed Semicolon
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
        0: Token(Var),
        1: Token(Ident, "positive"),
        2: Token(Equal),
        3: Token(Int, "42"),
        4: Token(Semicolon),
        5: Token(Var),
        6: Token(Ident, "negative"),
        7: Token(Equal),
        8: Token(Minus),
        9: Token(Ident, "positive"),
        10: Token(Semicolon),
        11: Token(Var),
        12: Token(Ident, "double_neg"),
        13: Token(Equal),
        14: Token(Minus),
        15: Token(Ident, "negative"),
        16: Token(Semicolon),
        17: Token(Ident, "assert"),
        18: Token(LeftParen),
        19: Token(Ident, "negative"),
        20: Token(EqualEqual),
        21: Token(Minus),
        22: Token(Int, "42"),
        23: Token(RightParen),
        24: Token(Semicolon),
        25: Token(Ident, "assert"),
        26: Token(LeftParen),
        27: Token(Ident, "double_neg"),
        28: Token(EqualEqual),
        29: Token(Int, "42"),
        30: Token(RightParen),
        31: Token(Semicolon),
        32: Token(Ident, "assert"),
        33: Token(LeftParen),
        34: Token(Minus),
        35: Token(Int, "100"),
        36: Token(EqualEqual),
        37: Token(Minus),
        38: Token(Int, "100"),
        39: Token(RightParen),
        40: Token(Semicolon),
        41: Token(Ident, "assert"),
        42: Token(LeftParen),
        43: Token(Minus),
        44: Token(LeftParen),
        45: Token(Int, "5"),
        46: Token(Plus),
        47: Token(Int, "3"),
        48: Token(RightParen),
        49: Token(EqualEqual),
        50: Token(Minus),
        51: Token(Int, "8"),
        52: Token(RightParen),
        53: Token(Semicolon),
        54: Token(Eof)
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
                    "positive",
                ),
                value: Some(
                    Literal(
                        Int(
                            42,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "negative",
                ),
                value: Some(
                    Neg(
                        Identifier(
                            "positive",
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "double_neg",
                ),
                value: Some(
                    Neg(
                        Identifier(
                            "negative",
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
                                "negative",
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
                            Identifier(
                                "double_neg",
                            ),
                            Literal(
                                Int(
                                    42,
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
                            Neg(
                                Literal(
                                    Int(
                                        100,
                                    ),
                                ),
                            ),
                            Neg(
                                Literal(
                                    Int(
                                        100,
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
                            Neg(
                                Add(
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
                                ),
                            ),
                            Neg(
                                Literal(
                                    Int(
                                        8,
                                    ),
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