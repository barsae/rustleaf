# Program
Status: 🟢
Assertions: 1

```rustleaf
var x = 5;
assert((if x > 0 { "positive" } else { "zero or negative" }) == "positive");
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
consume_token: position 0 consumed Var
consume_token: position 1 consumed Ident
consume_token: position 2 consumed Equal
parse_expression: starting at position 3 (Int(5))
consume_token: position 3 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 4 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 5 (Ident(assert))
parse_statement: starting at position 5 (Ident(assert))
consume_token: position 5 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 5 (Ident(assert))
consume_token: position 5 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 6 consumed LeftParen
parse_expression: starting at position 7 (LeftParen)
consume_token: position 7 consumed LeftParen
parse_primary: success - parsing parenthesized expression
parse_expression: starting at position 8 (If)
consume_token: position 8 consumed If
parse_primary: success - parsing if expression
parse_expression: starting at position 9 (Ident(x))
consume_token: position 9 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 10 consumed Greater
consume_token: position 11 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 12 consumed LeftBrace
parse_statement: starting at position 13 (String(positive))
parse_statement: falling back to expression statement
parse_expression: starting at position 13 (String(positive))
consume_token: position 13 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 14
parse_expression: starting at position 13 (String(positive))
consume_token: position 13 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 14 consumed RightBrace
consume_token: position 15 consumed Else
consume_token: position 16 consumed LeftBrace
parse_statement: starting at position 17 (String(zero or negative))
parse_statement: falling back to expression statement
parse_expression: starting at position 17 (String(zero or negative))
consume_token: position 17 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 18
parse_expression: starting at position 17 (String(zero or negative))
consume_token: position 17 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 18 consumed RightBrace
parse_expression: success - parsed precedence expression
consume_token: position 19 consumed RightParen
consume_token: position 20 consumed EqualEqual
consume_token: position 21 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 22 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 23 consumed Semicolon
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
        1: Token(Ident, "x"),
        2: Token(Equal),
        3: Token(Int, "5"),
        4: Token(Semicolon),
        5: Token(Ident, "assert"),
        6: Token(LeftParen),
        7: Token(LeftParen),
        8: Token(If),
        9: Token(Ident, "x"),
        10: Token(Greater),
        11: Token(Int, "0"),
        12: Token(LeftBrace),
        13: Token(String, "positive"),
        14: Token(RightBrace),
        15: Token(Else),
        16: Token(LeftBrace),
        17: Token(String, "zero or negative"),
        18: Token(RightBrace),
        19: Token(RightParen),
        20: Token(EqualEqual),
        21: Token(String, "positive"),
        22: Token(RightParen),
        23: Token(Semicolon),
        24: Token(Eof)
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
                            If {
                                condition: Gt(
                                    Identifier(
                                        "x",
                                    ),
                                    Literal(
                                        Int(
                                            0,
                                        ),
                                    ),
                                ),
                                then_expr: Block {
                                    statements: [],
                                    final_expr: Some(
                                        Literal(
                                            String(
                                                "positive",
                                            ),
                                        ),
                                    ),
                                },
                                else_expr: Some(
                                    Block {
                                        statements: [],
                                        final_expr: Some(
                                            Literal(
                                                String(
                                                    "zero or negative",
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                            },
                            Literal(
                                String(
                                    "positive",
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