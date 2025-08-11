# Program
Status: 🟢
Assertions: 1

```rustleaf
var y = {
    var x = 10;
    x + 5
};
assert(y == 15);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
consume_token: position 0 consumed Var
consume_token: position 1 consumed Ident
consume_token: position 2 consumed Equal
parse_expression: starting at position 3 (LeftBrace)
parse_primary: success - parsing block or dict
consume_token: position 3 consumed LeftBrace
parse_primary: failed - no matching primary expression for Var
parse_statement: starting at position 4 (Var)
consume_token: position 4 consumed Var
consume_token: position 5 consumed Ident
consume_token: position 6 consumed Equal
parse_expression: starting at position 7 (Int(10))
consume_token: position 7 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 8 consumed Semicolon
parse_statement: success - parsed var declaration
parse_statement: starting at position 9 (Ident(x))
consume_token: position 9 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 9 (Ident(x))
consume_token: position 9 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 10 consumed Plus
consume_token: position 11 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 12
parse_expression: starting at position 9 (Ident(x))
consume_token: position 9 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 10 consumed Plus
consume_token: position 11 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 12 consumed RightBrace
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
parse_program: parsed 2 statements
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
        2: Token(Equal),
        3: Token(LeftBrace),
        4: Token(Var),
        5: Token(Ident, "x"),
        6: Token(Equal),
        7: Token(Int, "10"),
        8: Token(Semicolon),
        9: Token(Ident, "x"),
        10: Token(Plus),
        11: Token(Int, "5"),
        12: Token(RightBrace),
        13: Token(Semicolon),
        14: Token(Ident, "assert"),
        15: Token(LeftParen),
        16: Token(Ident, "y"),
        17: Token(EqualEqual),
        18: Token(Int, "15"),
        19: Token(RightParen),
        20: Token(Semicolon),
        21: Token(Eof)
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
                value: Some(
                    Block(
                        Block {
                            statements: [
                                VarDecl {
                                    pattern: Variable(
                                        "x",
                                    ),
                                    value: Some(
                                        Literal(
                                            Int(
                                                10,
                                            ),
                                        ),
                                    ),
                                },
                            ],
                            final_expr: Some(
                                Add(
                                    Identifier(
                                        "x",
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
                                    15,
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