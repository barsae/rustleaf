# Program
Status: 🟢
Assertions: 2

```rustleaf
var lambda = || 42;
var result = lambda();
assert(result == 42);
assert(is_unit(lambda) == false);  // Lambda should not be unit
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
consume_token: position 4 consumed Pipe
parse_expression: starting at position 5 (Int(42))
consume_token: position 5 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
consume_token: position 6 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 7 (Var)
parse_statement: starting at position 7 (Var)
consume_token: position 7 consumed Var
consume_token: position 8 consumed Ident
consume_token: position 9 consumed Equal
parse_expression: starting at position 10 (Ident(lambda))
consume_token: position 10 consumed Ident
parse_primary: success - parsed identifier (lambda)
consume_token: position 11 consumed LeftParen
consume_token: position 12 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 13 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 14 (Ident(assert))
parse_statement: starting at position 14 (Ident(assert))
consume_token: position 14 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 14 (Ident(assert))
consume_token: position 14 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 15 consumed LeftParen
parse_expression: starting at position 16 (Ident(result))
consume_token: position 16 consumed Ident
parse_primary: success - parsed identifier (result)
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
parse_expression: starting at position 23 (Ident(is_unit))
consume_token: position 23 consumed Ident
parse_primary: success - parsed identifier (is_unit)
consume_token: position 24 consumed LeftParen
parse_expression: starting at position 25 (Ident(lambda))
consume_token: position 25 consumed Ident
parse_primary: success - parsed identifier (lambda)
parse_expression: success - parsed precedence expression
consume_token: position 26 consumed RightParen
consume_token: position 27 consumed EqualEqual
consume_token: position 28 consumed False
parse_primary: success - parsed boolean literal (false)
parse_expression: success - parsed precedence expression
consume_token: position 29 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 30 consumed Semicolon
parse_program: parsed 4 statements
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
        1: Token(Ident, "lambda"),
        2: Token(Equal),
        3: Token(Pipe),
        4: Token(Pipe),
        5: Token(Int, "42"),
        6: Token(Semicolon),
        7: Token(Var),
        8: Token(Ident, "result"),
        9: Token(Equal),
        10: Token(Ident, "lambda"),
        11: Token(LeftParen),
        12: Token(RightParen),
        13: Token(Semicolon),
        14: Token(Ident, "assert"),
        15: Token(LeftParen),
        16: Token(Ident, "result"),
        17: Token(EqualEqual),
        18: Token(Int, "42"),
        19: Token(RightParen),
        20: Token(Semicolon),
        21: Token(Ident, "assert"),
        22: Token(LeftParen),
        23: Token(Ident, "is_unit"),
        24: Token(LeftParen),
        25: Token(Ident, "lambda"),
        26: Token(RightParen),
        27: Token(EqualEqual),
        28: Token(False),
        29: Token(RightParen),
        30: Token(Semicolon),
        31: Token(Eof)
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
                    "lambda",
                ),
                value: Some(
                    Lambda {
                        params: [],
                        body: Expression(
                            Literal(
                                Int(
                                    42,
                                ),
                            ),
                        ),
                    },
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "lambda",
                        ),
                        [],
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
                            FunctionCall(
                                Identifier(
                                    "is_unit",
                                ),
                                [
                                    Identifier(
                                        "lambda",
                                    ),
                                ],
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