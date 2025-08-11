# Program
Status: 🟢
Assertions: 3

```rustleaf
var y;
var z;
y = 100;
z = "assigned later";
assert(y == 100);
assert(z == "assigned later");
assert(y + 23 == 123);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
consume_token: position 0 consumed Var
consume_token: position 1 consumed Ident
consume_token: position 2 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 3 (Var)
parse_statement: starting at position 3 (Var)
consume_token: position 3 consumed Var
consume_token: position 4 consumed Ident
consume_token: position 5 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 6 (Ident(y))
parse_statement: starting at position 6 (Ident(y))
consume_token: position 6 consumed Ident
consume_token: position 7 consumed Equal
parse_expression: starting at position 8 (Int(100))
consume_token: position 8 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 9 consumed Semicolon
parse_statement: success - parsed assignment
parse_program: parsing statement at position 10 (Ident(z))
parse_statement: starting at position 10 (Ident(z))
consume_token: position 10 consumed Ident
consume_token: position 11 consumed Equal
parse_expression: starting at position 12 (String(assigned later))
consume_token: position 12 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 13 consumed Semicolon
parse_statement: success - parsed assignment
parse_program: parsing statement at position 14 (Ident(assert))
parse_statement: starting at position 14 (Ident(assert))
consume_token: position 14 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 14 (Ident(assert))
consume_token: position 14 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 15 consumed LeftParen
parse_expression: starting at position 16 (Ident(y))
consume_token: position 16 consumed Ident
parse_primary: success - parsed identifier (y)
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
parse_expression: starting at position 23 (Ident(z))
consume_token: position 23 consumed Ident
parse_primary: success - parsed identifier (z)
consume_token: position 24 consumed EqualEqual
consume_token: position 25 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 26 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 27 consumed Semicolon
parse_program: parsing statement at position 28 (Ident(assert))
parse_statement: starting at position 28 (Ident(assert))
consume_token: position 28 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 28 (Ident(assert))
consume_token: position 28 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 29 consumed LeftParen
parse_expression: starting at position 30 (Ident(y))
consume_token: position 30 consumed Ident
parse_primary: success - parsed identifier (y)
consume_token: position 31 consumed Plus
consume_token: position 32 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 33 consumed EqualEqual
consume_token: position 34 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 35 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 36 consumed Semicolon
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
        1: Token(Ident, "y"),
        2: Token(Semicolon),
        3: Token(Var),
        4: Token(Ident, "z"),
        5: Token(Semicolon),
        6: Token(Ident, "y"),
        7: Token(Equal),
        8: Token(Int, "100"),
        9: Token(Semicolon),
        10: Token(Ident, "z"),
        11: Token(Equal),
        12: Token(String, "assigned later"),
        13: Token(Semicolon),
        14: Token(Ident, "assert"),
        15: Token(LeftParen),
        16: Token(Ident, "y"),
        17: Token(EqualEqual),
        18: Token(Int, "100"),
        19: Token(RightParen),
        20: Token(Semicolon),
        21: Token(Ident, "assert"),
        22: Token(LeftParen),
        23: Token(Ident, "z"),
        24: Token(EqualEqual),
        25: Token(String, "assigned later"),
        26: Token(RightParen),
        27: Token(Semicolon),
        28: Token(Ident, "assert"),
        29: Token(LeftParen),
        30: Token(Ident, "y"),
        31: Token(Plus),
        32: Token(Int, "23"),
        33: Token(EqualEqual),
        34: Token(Int, "123"),
        35: Token(RightParen),
        36: Token(Semicolon),
        37: Token(Eof)
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
                    "y",
                ),
                value: None,
            },
            VarDecl {
                pattern: Variable(
                    "z",
                ),
                value: None,
            },
            Assignment {
                target: Identifier(
                    "y",
                ),
                op: Assign,
                value: Literal(
                    Int(
                        100,
                    ),
                ),
            },
            Assignment {
                target: Identifier(
                    "z",
                ),
                op: Assign,
                value: Literal(
                    String(
                        "assigned later",
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
                                "y",
                            ),
                            Literal(
                                Int(
                                    100,
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
                                "z",
                            ),
                            Literal(
                                String(
                                    "assigned later",
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
                                Identifier(
                                    "y",
                                ),
                                Literal(
                                    Int(
                                        23,
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    123,
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