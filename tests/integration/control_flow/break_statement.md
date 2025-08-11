# Program
Status: 🟢
Assertions: 4

```rustleaf
var result = loop {
    break 42;
};

var result2 = loop {
    break "hello";
};

var counter = 0;
var result3 = loop {
    counter = counter + 1;
    if counter == 3 {
        break counter * 5;
    }
};

assert(result == 42);
assert(result2 == "hello");
assert(result3 == 15);
assert(counter == 3);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
consume_token: position 0 consumed Var
consume_token: position 1 consumed Ident
consume_token: position 2 consumed Equal
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
consume_token: position 9 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 10 (Var)
parse_statement: starting at position 10 (Var)
consume_token: position 10 consumed Var
consume_token: position 11 consumed Ident
consume_token: position 12 consumed Equal
parse_expression: starting at position 13 (Loop)
consume_token: position 13 consumed Loop
parse_primary: success - parsing loop expression
consume_token: position 14 consumed LeftBrace
parse_statement: starting at position 15 (Break)
consume_token: position 15 consumed Break
parse_expression: starting at position 16 (String(hello))
consume_token: position 16 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 17 consumed Semicolon
parse_statement: success - parsed break statement
consume_token: position 18 consumed RightBrace
parse_expression: success - parsed precedence expression
consume_token: position 19 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 20 (Var)
parse_statement: starting at position 20 (Var)
consume_token: position 20 consumed Var
consume_token: position 21 consumed Ident
consume_token: position 22 consumed Equal
parse_expression: starting at position 23 (Int(0))
consume_token: position 23 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 24 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 25 (Var)
parse_statement: starting at position 25 (Var)
consume_token: position 25 consumed Var
consume_token: position 26 consumed Ident
consume_token: position 27 consumed Equal
parse_expression: starting at position 28 (Loop)
consume_token: position 28 consumed Loop
parse_primary: success - parsing loop expression
consume_token: position 29 consumed LeftBrace
parse_statement: starting at position 30 (Ident(counter))
consume_token: position 30 consumed Ident
consume_token: position 31 consumed Equal
parse_expression: starting at position 32 (Ident(counter))
consume_token: position 32 consumed Ident
parse_primary: success - parsed identifier (counter)
consume_token: position 33 consumed Plus
consume_token: position 34 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 35 consumed Semicolon
parse_statement: success - parsed assignment
parse_statement: starting at position 36 (If)
parse_expression: starting at position 36 (If)
consume_token: position 36 consumed If
parse_primary: success - parsing if expression
parse_expression: starting at position 37 (Ident(counter))
consume_token: position 37 consumed Ident
parse_primary: success - parsed identifier (counter)
consume_token: position 38 consumed EqualEqual
consume_token: position 39 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 40 consumed LeftBrace
parse_statement: starting at position 41 (Break)
consume_token: position 41 consumed Break
parse_expression: starting at position 42 (Ident(counter))
consume_token: position 42 consumed Ident
parse_primary: success - parsed identifier (counter)
consume_token: position 43 consumed Star
consume_token: position 44 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 45 consumed Semicolon
parse_statement: success - parsed break statement
consume_token: position 46 consumed RightBrace
parse_expression: success - parsed precedence expression
parse_statement: success - parsed block-like expression statement
consume_token: position 47 consumed RightBrace
parse_expression: success - parsed precedence expression
consume_token: position 48 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 49 (Ident(assert))
parse_statement: starting at position 49 (Ident(assert))
consume_token: position 49 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 49 (Ident(assert))
consume_token: position 49 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 50 consumed LeftParen
parse_expression: starting at position 51 (Ident(result))
consume_token: position 51 consumed Ident
parse_primary: success - parsed identifier (result)
consume_token: position 52 consumed EqualEqual
consume_token: position 53 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 54 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 55 consumed Semicolon
parse_program: parsing statement at position 56 (Ident(assert))
parse_statement: starting at position 56 (Ident(assert))
consume_token: position 56 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 56 (Ident(assert))
consume_token: position 56 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 57 consumed LeftParen
parse_expression: starting at position 58 (Ident(result2))
consume_token: position 58 consumed Ident
parse_primary: success - parsed identifier (result2)
consume_token: position 59 consumed EqualEqual
consume_token: position 60 consumed String
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
parse_expression: starting at position 65 (Ident(result3))
consume_token: position 65 consumed Ident
parse_primary: success - parsed identifier (result3)
consume_token: position 66 consumed EqualEqual
consume_token: position 67 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 68 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 69 consumed Semicolon
parse_program: parsing statement at position 70 (Ident(assert))
parse_statement: starting at position 70 (Ident(assert))
consume_token: position 70 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 70 (Ident(assert))
consume_token: position 70 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 71 consumed LeftParen
parse_expression: starting at position 72 (Ident(counter))
consume_token: position 72 consumed Ident
parse_primary: success - parsed identifier (counter)
consume_token: position 73 consumed EqualEqual
consume_token: position 74 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 75 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 76 consumed Semicolon
parse_program: parsed 8 statements
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
        1: Token(Ident, "result"),
        2: Token(Equal),
        3: Token(Loop),
        4: Token(LeftBrace),
        5: Token(Break),
        6: Token(Int, "42"),
        7: Token(Semicolon),
        8: Token(RightBrace),
        9: Token(Semicolon),
        10: Token(Var),
        11: Token(Ident, "result2"),
        12: Token(Equal),
        13: Token(Loop),
        14: Token(LeftBrace),
        15: Token(Break),
        16: Token(String, "hello"),
        17: Token(Semicolon),
        18: Token(RightBrace),
        19: Token(Semicolon),
        20: Token(Var),
        21: Token(Ident, "counter"),
        22: Token(Equal),
        23: Token(Int, "0"),
        24: Token(Semicolon),
        25: Token(Var),
        26: Token(Ident, "result3"),
        27: Token(Equal),
        28: Token(Loop),
        29: Token(LeftBrace),
        30: Token(Ident, "counter"),
        31: Token(Equal),
        32: Token(Ident, "counter"),
        33: Token(Plus),
        34: Token(Int, "1"),
        35: Token(Semicolon),
        36: Token(If),
        37: Token(Ident, "counter"),
        38: Token(EqualEqual),
        39: Token(Int, "3"),
        40: Token(LeftBrace),
        41: Token(Break),
        42: Token(Ident, "counter"),
        43: Token(Star),
        44: Token(Int, "5"),
        45: Token(Semicolon),
        46: Token(RightBrace),
        47: Token(RightBrace),
        48: Token(Semicolon),
        49: Token(Ident, "assert"),
        50: Token(LeftParen),
        51: Token(Ident, "result"),
        52: Token(EqualEqual),
        53: Token(Int, "42"),
        54: Token(RightParen),
        55: Token(Semicolon),
        56: Token(Ident, "assert"),
        57: Token(LeftParen),
        58: Token(Ident, "result2"),
        59: Token(EqualEqual),
        60: Token(String, "hello"),
        61: Token(RightParen),
        62: Token(Semicolon),
        63: Token(Ident, "assert"),
        64: Token(LeftParen),
        65: Token(Ident, "result3"),
        66: Token(EqualEqual),
        67: Token(Int, "15"),
        68: Token(RightParen),
        69: Token(Semicolon),
        70: Token(Ident, "assert"),
        71: Token(LeftParen),
        72: Token(Ident, "counter"),
        73: Token(EqualEqual),
        74: Token(Int, "3"),
        75: Token(RightParen),
        76: Token(Semicolon),
        77: Token(Eof)
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
                    "result",
                ),
                value: Some(
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
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result2",
                ),
                value: Some(
                    Loop {
                        body: Block {
                            statements: [
                                Break(
                                    Some(
                                        Literal(
                                            String(
                                                "hello",
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            final_expr: None,
                        },
                    },
                ),
            },
            VarDecl {
                pattern: Variable(
                    "counter",
                ),
                value: Some(
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result3",
                ),
                value: Some(
                    Loop {
                        body: Block {
                            statements: [
                                Assignment {
                                    target: Identifier(
                                        "counter",
                                    ),
                                    op: Assign,
                                    value: Add(
                                        Identifier(
                                            "counter",
                                        ),
                                        Literal(
                                            Int(
                                                1,
                                            ),
                                        ),
                                    ),
                                },
                                Expression(
                                    If {
                                        condition: Eq(
                                            Identifier(
                                                "counter",
                                            ),
                                            Literal(
                                                Int(
                                                    3,
                                                ),
                                            ),
                                        ),
                                        then_expr: Block {
                                            statements: [
                                                Break(
                                                    Some(
                                                        Mul(
                                                            Identifier(
                                                                "counter",
                                                            ),
                                                            Literal(
                                                                Int(
                                                                    5,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ],
                                            final_expr: None,
                                        },
                                        else_expr: None,
                                    },
                                ),
                            ],
                            final_expr: None,
                        },
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
                            Identifier(
                                "result",
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
                            Identifier(
                                "result2",
                            ),
                            Literal(
                                String(
                                    "hello",
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
                                "result3",
                            ),
                            Literal(
                                Int(
                                    15,
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
                                "counter",
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