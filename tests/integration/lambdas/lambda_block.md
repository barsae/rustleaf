# Program
Status: 🟢
Assertions: 4

```rustleaf
var processor = |x| {
    var temp = x * 2;
    temp + 1
};

var complex_lambda = |y| {
    var first = y + 10;
    var second = first * 3;
    second - 5
};

assert(processor(5) == 11);
assert(processor(0) == 1);
assert(complex_lambda(2) == 31);
assert(complex_lambda(10) == 55);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
consume_token: position 0 consumed Var
consume_token: position 1 consumed Ident
consume_token: position 2 consumed Equal
parse_expression: starting at position 3 (Pipe)
parse_primary: success - parsing lambda expression
consume_token: position 3 consumed Pipe
consume_token: position 4 consumed Ident
consume_token: position 5 consumed Pipe
consume_token: position 6 consumed LeftBrace
parse_statement: starting at position 7 (Var)
consume_token: position 7 consumed Var
consume_token: position 8 consumed Ident
consume_token: position 9 consumed Equal
parse_expression: starting at position 10 (Ident(x))
consume_token: position 10 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 11 consumed Star
consume_token: position 12 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 13 consumed Semicolon
parse_statement: success - parsed var declaration
parse_statement: starting at position 14 (Ident(temp))
consume_token: position 14 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 14 (Ident(temp))
consume_token: position 14 consumed Ident
parse_primary: success - parsed identifier (temp)
consume_token: position 15 consumed Plus
consume_token: position 16 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 17
parse_expression: starting at position 14 (Ident(temp))
consume_token: position 14 consumed Ident
parse_primary: success - parsed identifier (temp)
consume_token: position 15 consumed Plus
consume_token: position 16 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 17 consumed RightBrace
parse_expression: success - parsed precedence expression
consume_token: position 18 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 19 (Var)
parse_statement: starting at position 19 (Var)
consume_token: position 19 consumed Var
consume_token: position 20 consumed Ident
consume_token: position 21 consumed Equal
parse_expression: starting at position 22 (Pipe)
parse_primary: success - parsing lambda expression
consume_token: position 22 consumed Pipe
consume_token: position 23 consumed Ident
consume_token: position 24 consumed Pipe
consume_token: position 25 consumed LeftBrace
parse_statement: starting at position 26 (Var)
consume_token: position 26 consumed Var
consume_token: position 27 consumed Ident
consume_token: position 28 consumed Equal
parse_expression: starting at position 29 (Ident(y))
consume_token: position 29 consumed Ident
parse_primary: success - parsed identifier (y)
consume_token: position 30 consumed Plus
consume_token: position 31 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 32 consumed Semicolon
parse_statement: success - parsed var declaration
parse_statement: starting at position 33 (Var)
consume_token: position 33 consumed Var
consume_token: position 34 consumed Ident
consume_token: position 35 consumed Equal
parse_expression: starting at position 36 (Ident(first))
consume_token: position 36 consumed Ident
parse_primary: success - parsed identifier (first)
consume_token: position 37 consumed Star
consume_token: position 38 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 39 consumed Semicolon
parse_statement: success - parsed var declaration
parse_statement: starting at position 40 (Ident(second))
consume_token: position 40 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 40 (Ident(second))
consume_token: position 40 consumed Ident
parse_primary: success - parsed identifier (second)
consume_token: position 41 consumed Minus
consume_token: position 42 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 43
parse_expression: starting at position 40 (Ident(second))
consume_token: position 40 consumed Ident
parse_primary: success - parsed identifier (second)
consume_token: position 41 consumed Minus
consume_token: position 42 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 43 consumed RightBrace
parse_expression: success - parsed precedence expression
consume_token: position 44 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 45 (Ident(assert))
parse_statement: starting at position 45 (Ident(assert))
consume_token: position 45 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 45 (Ident(assert))
consume_token: position 45 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 46 consumed LeftParen
parse_expression: starting at position 47 (Ident(processor))
consume_token: position 47 consumed Ident
parse_primary: success - parsed identifier (processor)
consume_token: position 48 consumed LeftParen
parse_expression: starting at position 49 (Int(5))
consume_token: position 49 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 50 consumed RightParen
consume_token: position 51 consumed EqualEqual
consume_token: position 52 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 53 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 54 consumed Semicolon
parse_program: parsing statement at position 55 (Ident(assert))
parse_statement: starting at position 55 (Ident(assert))
consume_token: position 55 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 55 (Ident(assert))
consume_token: position 55 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 56 consumed LeftParen
parse_expression: starting at position 57 (Ident(processor))
consume_token: position 57 consumed Ident
parse_primary: success - parsed identifier (processor)
consume_token: position 58 consumed LeftParen
parse_expression: starting at position 59 (Int(0))
consume_token: position 59 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 60 consumed RightParen
consume_token: position 61 consumed EqualEqual
consume_token: position 62 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 63 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 64 consumed Semicolon
parse_program: parsing statement at position 65 (Ident(assert))
parse_statement: starting at position 65 (Ident(assert))
consume_token: position 65 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 65 (Ident(assert))
consume_token: position 65 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 66 consumed LeftParen
parse_expression: starting at position 67 (Ident(complex_lambda))
consume_token: position 67 consumed Ident
parse_primary: success - parsed identifier (complex_lambda)
consume_token: position 68 consumed LeftParen
parse_expression: starting at position 69 (Int(2))
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
parse_program: parsing statement at position 75 (Ident(assert))
parse_statement: starting at position 75 (Ident(assert))
consume_token: position 75 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 75 (Ident(assert))
consume_token: position 75 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 76 consumed LeftParen
parse_expression: starting at position 77 (Ident(complex_lambda))
consume_token: position 77 consumed Ident
parse_primary: success - parsed identifier (complex_lambda)
consume_token: position 78 consumed LeftParen
parse_expression: starting at position 79 (Int(10))
consume_token: position 79 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 80 consumed RightParen
consume_token: position 81 consumed EqualEqual
consume_token: position 82 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 83 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 84 consumed Semicolon
parse_program: parsed 6 statements
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
        1: Token(Ident, "processor"),
        2: Token(Equal),
        3: Token(Pipe),
        4: Token(Ident, "x"),
        5: Token(Pipe),
        6: Token(LeftBrace),
        7: Token(Var),
        8: Token(Ident, "temp"),
        9: Token(Equal),
        10: Token(Ident, "x"),
        11: Token(Star),
        12: Token(Int, "2"),
        13: Token(Semicolon),
        14: Token(Ident, "temp"),
        15: Token(Plus),
        16: Token(Int, "1"),
        17: Token(RightBrace),
        18: Token(Semicolon),
        19: Token(Var),
        20: Token(Ident, "complex_lambda"),
        21: Token(Equal),
        22: Token(Pipe),
        23: Token(Ident, "y"),
        24: Token(Pipe),
        25: Token(LeftBrace),
        26: Token(Var),
        27: Token(Ident, "first"),
        28: Token(Equal),
        29: Token(Ident, "y"),
        30: Token(Plus),
        31: Token(Int, "10"),
        32: Token(Semicolon),
        33: Token(Var),
        34: Token(Ident, "second"),
        35: Token(Equal),
        36: Token(Ident, "first"),
        37: Token(Star),
        38: Token(Int, "3"),
        39: Token(Semicolon),
        40: Token(Ident, "second"),
        41: Token(Minus),
        42: Token(Int, "5"),
        43: Token(RightBrace),
        44: Token(Semicolon),
        45: Token(Ident, "assert"),
        46: Token(LeftParen),
        47: Token(Ident, "processor"),
        48: Token(LeftParen),
        49: Token(Int, "5"),
        50: Token(RightParen),
        51: Token(EqualEqual),
        52: Token(Int, "11"),
        53: Token(RightParen),
        54: Token(Semicolon),
        55: Token(Ident, "assert"),
        56: Token(LeftParen),
        57: Token(Ident, "processor"),
        58: Token(LeftParen),
        59: Token(Int, "0"),
        60: Token(RightParen),
        61: Token(EqualEqual),
        62: Token(Int, "1"),
        63: Token(RightParen),
        64: Token(Semicolon),
        65: Token(Ident, "assert"),
        66: Token(LeftParen),
        67: Token(Ident, "complex_lambda"),
        68: Token(LeftParen),
        69: Token(Int, "2"),
        70: Token(RightParen),
        71: Token(EqualEqual),
        72: Token(Int, "31"),
        73: Token(RightParen),
        74: Token(Semicolon),
        75: Token(Ident, "assert"),
        76: Token(LeftParen),
        77: Token(Ident, "complex_lambda"),
        78: Token(LeftParen),
        79: Token(Int, "10"),
        80: Token(RightParen),
        81: Token(EqualEqual),
        82: Token(Int, "55"),
        83: Token(RightParen),
        84: Token(Semicolon),
        85: Token(Eof)
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
                    "processor",
                ),
                value: Some(
                    Lambda {
                        params: [
                            "x",
                        ],
                        body: Block(
                            Block {
                                statements: [
                                    VarDecl {
                                        pattern: Variable(
                                            "temp",
                                        ),
                                        value: Some(
                                            Mul(
                                                Identifier(
                                                    "x",
                                                ),
                                                Literal(
                                                    Int(
                                                        2,
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                ],
                                final_expr: Some(
                                    Add(
                                        Identifier(
                                            "temp",
                                        ),
                                        Literal(
                                            Int(
                                                1,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                ),
            },
            VarDecl {
                pattern: Variable(
                    "complex_lambda",
                ),
                value: Some(
                    Lambda {
                        params: [
                            "y",
                        ],
                        body: Block(
                            Block {
                                statements: [
                                    VarDecl {
                                        pattern: Variable(
                                            "first",
                                        ),
                                        value: Some(
                                            Add(
                                                Identifier(
                                                    "y",
                                                ),
                                                Literal(
                                                    Int(
                                                        10,
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                    VarDecl {
                                        pattern: Variable(
                                            "second",
                                        ),
                                        value: Some(
                                            Mul(
                                                Identifier(
                                                    "first",
                                                ),
                                                Literal(
                                                    Int(
                                                        3,
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                ],
                                final_expr: Some(
                                    Sub(
                                        Identifier(
                                            "second",
                                        ),
                                        Literal(
                                            Int(
                                                5,
                                            ),
                                        ),
                                    ),
                                ),
                            },
                        ),
                    },
                ),
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            FunctionCall(
                                Identifier(
                                    "processor",
                                ),
                                [
                                    Literal(
                                        Int(
                                            5,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    11,
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
                                    "processor",
                                ),
                                [
                                    Literal(
                                        Int(
                                            0,
                                        ),
                                    ),
                                ],
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
                            FunctionCall(
                                Identifier(
                                    "complex_lambda",
                                ),
                                [
                                    Literal(
                                        Int(
                                            2,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    31,
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
                                    "complex_lambda",
                                ),
                                [
                                    Literal(
                                        Int(
                                            10,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    55,
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