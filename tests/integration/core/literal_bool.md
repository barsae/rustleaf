# Program
Status: 🟢
Assertions: 4

```rustleaf
var t = true;
var f = false;
assert(t == true);
assert(f == false);
assert(t != f);
assert(not f == true);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
consume_token: position 0 consumed Var
consume_token: position 1 consumed Ident
consume_token: position 2 consumed Equal
parse_expression: starting at position 3 (True)
consume_token: position 3 consumed True
parse_primary: success - parsed boolean literal (true)
parse_expression: success - parsed precedence expression
consume_token: position 4 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 5 (Var)
parse_statement: starting at position 5 (Var)
consume_token: position 5 consumed Var
consume_token: position 6 consumed Ident
consume_token: position 7 consumed Equal
parse_expression: starting at position 8 (False)
consume_token: position 8 consumed False
parse_primary: success - parsed boolean literal (false)
parse_expression: success - parsed precedence expression
consume_token: position 9 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 10 (Ident(assert))
parse_statement: starting at position 10 (Ident(assert))
consume_token: position 10 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 10 (Ident(assert))
consume_token: position 10 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 11 consumed LeftParen
parse_expression: starting at position 12 (Ident(t))
consume_token: position 12 consumed Ident
parse_primary: success - parsed identifier (t)
consume_token: position 13 consumed EqualEqual
consume_token: position 14 consumed True
parse_primary: success - parsed boolean literal (true)
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
parse_expression: starting at position 19 (Ident(f))
consume_token: position 19 consumed Ident
parse_primary: success - parsed identifier (f)
consume_token: position 20 consumed EqualEqual
consume_token: position 21 consumed False
parse_primary: success - parsed boolean literal (false)
parse_expression: success - parsed precedence expression
consume_token: position 22 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 23 consumed Semicolon
parse_program: parsing statement at position 24 (Ident(assert))
parse_statement: starting at position 24 (Ident(assert))
consume_token: position 24 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 24 (Ident(assert))
consume_token: position 24 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 25 consumed LeftParen
parse_expression: starting at position 26 (Ident(t))
consume_token: position 26 consumed Ident
parse_primary: success - parsed identifier (t)
consume_token: position 27 consumed BangEqual
consume_token: position 28 consumed Ident
parse_primary: success - parsed identifier (f)
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
parse_expression: starting at position 33 (Not)
consume_token: position 33 consumed Not
consume_token: position 34 consumed Ident
parse_primary: success - parsed identifier (f)
consume_token: position 35 consumed EqualEqual
consume_token: position 36 consumed True
parse_primary: success - parsed boolean literal (true)
parse_expression: success - parsed precedence expression
consume_token: position 37 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 38 consumed Semicolon
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
        1: Token(Ident, "t"),
        2: Token(Equal),
        3: Token(True),
        4: Token(Semicolon),
        5: Token(Var),
        6: Token(Ident, "f"),
        7: Token(Equal),
        8: Token(False),
        9: Token(Semicolon),
        10: Token(Ident, "assert"),
        11: Token(LeftParen),
        12: Token(Ident, "t"),
        13: Token(EqualEqual),
        14: Token(True),
        15: Token(RightParen),
        16: Token(Semicolon),
        17: Token(Ident, "assert"),
        18: Token(LeftParen),
        19: Token(Ident, "f"),
        20: Token(EqualEqual),
        21: Token(False),
        22: Token(RightParen),
        23: Token(Semicolon),
        24: Token(Ident, "assert"),
        25: Token(LeftParen),
        26: Token(Ident, "t"),
        27: Token(BangEqual),
        28: Token(Ident, "f"),
        29: Token(RightParen),
        30: Token(Semicolon),
        31: Token(Ident, "assert"),
        32: Token(LeftParen),
        33: Token(Not),
        34: Token(Ident, "f"),
        35: Token(EqualEqual),
        36: Token(True),
        37: Token(RightParen),
        38: Token(Semicolon),
        39: Token(Eof)
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
                    "t",
                ),
                value: Some(
                    Literal(
                        Bool(
                            true,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "f",
                ),
                value: Some(
                    Literal(
                        Bool(
                            false,
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
                                "t",
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
                            Identifier(
                                "f",
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
                        Ne(
                            Identifier(
                                "t",
                            ),
                            Identifier(
                                "f",
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
                                Identifier(
                                    "f",
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