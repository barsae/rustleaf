# Program
Status: 🟢
Assertions: 4

```rustleaf
// Test 'not' as unary operator
assert(not true == false);
assert(not false == true);

// Test with expressions
var x = 5;
assert(not (x > 10) == true);
assert(not (x < 3) == true);   // x=5, x<3 is false, not false is true
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
parse_expression: starting at position 2 (Not)
consume_token: position 2 consumed Not
consume_token: position 3 consumed True
parse_primary: success - parsed boolean literal (true)
consume_token: position 4 consumed EqualEqual
consume_token: position 5 consumed False
parse_primary: success - parsed boolean literal (false)
parse_expression: success - parsed precedence expression
consume_token: position 6 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 7 consumed Semicolon
parse_program: parsing statement at position 8 (Ident(assert))
parse_statement: starting at position 8 (Ident(assert))
consume_token: position 8 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 8 (Ident(assert))
consume_token: position 8 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 9 consumed LeftParen
parse_expression: starting at position 10 (Not)
consume_token: position 10 consumed Not
consume_token: position 11 consumed False
parse_primary: success - parsed boolean literal (false)
consume_token: position 12 consumed EqualEqual
consume_token: position 13 consumed True
parse_primary: success - parsed boolean literal (true)
parse_expression: success - parsed precedence expression
consume_token: position 14 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 15 consumed Semicolon
parse_program: parsing statement at position 16 (Var)
parse_statement: starting at position 16 (Var)
consume_token: position 16 consumed Var
consume_token: position 17 consumed Ident
consume_token: position 18 consumed Equal
parse_expression: starting at position 19 (Int(5))
consume_token: position 19 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 20 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 21 (Ident(assert))
parse_statement: starting at position 21 (Ident(assert))
consume_token: position 21 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 21 (Ident(assert))
consume_token: position 21 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 22 consumed LeftParen
parse_expression: starting at position 23 (Not)
consume_token: position 23 consumed Not
consume_token: position 24 consumed LeftParen
parse_primary: success - parsing parenthesized expression
parse_expression: starting at position 25 (Ident(x))
consume_token: position 25 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 26 consumed Greater
consume_token: position 27 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 28 consumed RightParen
consume_token: position 29 consumed EqualEqual
consume_token: position 30 consumed True
parse_primary: success - parsed boolean literal (true)
parse_expression: success - parsed precedence expression
consume_token: position 31 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 32 consumed Semicolon
parse_program: parsing statement at position 33 (Ident(assert))
parse_statement: starting at position 33 (Ident(assert))
consume_token: position 33 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 33 (Ident(assert))
consume_token: position 33 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 34 consumed LeftParen
parse_expression: starting at position 35 (Not)
consume_token: position 35 consumed Not
consume_token: position 36 consumed LeftParen
parse_primary: success - parsing parenthesized expression
parse_expression: starting at position 37 (Ident(x))
consume_token: position 37 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 38 consumed Less
consume_token: position 39 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 40 consumed RightParen
consume_token: position 41 consumed EqualEqual
consume_token: position 42 consumed True
parse_primary: success - parsed boolean literal (true)
parse_expression: success - parsed precedence expression
consume_token: position 43 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 44 consumed Semicolon
parse_program: parsed 5 statements
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
        2: Token(Not),
        3: Token(True),
        4: Token(EqualEqual),
        5: Token(False),
        6: Token(RightParen),
        7: Token(Semicolon),
        8: Token(Ident, "assert"),
        9: Token(LeftParen),
        10: Token(Not),
        11: Token(False),
        12: Token(EqualEqual),
        13: Token(True),
        14: Token(RightParen),
        15: Token(Semicolon),
        16: Token(Var),
        17: Token(Ident, "x"),
        18: Token(Equal),
        19: Token(Int, "5"),
        20: Token(Semicolon),
        21: Token(Ident, "assert"),
        22: Token(LeftParen),
        23: Token(Not),
        24: Token(LeftParen),
        25: Token(Ident, "x"),
        26: Token(Greater),
        27: Token(Int, "10"),
        28: Token(RightParen),
        29: Token(EqualEqual),
        30: Token(True),
        31: Token(RightParen),
        32: Token(Semicolon),
        33: Token(Ident, "assert"),
        34: Token(LeftParen),
        35: Token(Not),
        36: Token(LeftParen),
        37: Token(Ident, "x"),
        38: Token(Less),
        39: Token(Int, "3"),
        40: Token(RightParen),
        41: Token(EqualEqual),
        42: Token(True),
        43: Token(RightParen),
        44: Token(Semicolon),
        45: Token(Eof)
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
            VarDecl {
                pattern: Variable(
                    "x",
                ),
                value: Some(
                    Literal(
                        Int(
                            5,
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
                            Not(
                                Gt(
                                    Identifier(
                                        "x",
                                    ),
                                    Literal(
                                        Int(
                                            10,
                                        ),
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Not(
                                Lt(
                                    Identifier(
                                        "x",
                                    ),
                                    Literal(
                                        Int(
                                            3,
                                        ),
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