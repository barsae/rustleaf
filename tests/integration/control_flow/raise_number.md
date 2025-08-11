# Program
Status: 🟢
Assertions: 1

```rustleaf
var i;
try {
    raise(42);
} catch e {
    i = e;
}
assert(i == 42);
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
parse_program: parsing statement at position 3 (Try)
parse_statement: starting at position 3 (Try)
parse_expression: starting at position 3 (Try)
consume_token: position 3 consumed Try
parse_primary: success - parsing try expression
consume_token: position 4 consumed LeftBrace
parse_statement: starting at position 5 (Ident(raise))
consume_token: position 5 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 5 (Ident(raise))
consume_token: position 5 consumed Ident
parse_primary: success - parsed identifier (raise)
consume_token: position 6 consumed LeftParen
parse_expression: starting at position 7 (Int(42))
consume_token: position 7 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 8 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 9 consumed Semicolon
consume_token: position 10 consumed RightBrace
consume_token: position 11 consumed Catch
consume_token: position 12 consumed Ident
consume_token: position 13 consumed LeftBrace
parse_statement: starting at position 14 (Ident(i))
consume_token: position 14 consumed Ident
consume_token: position 15 consumed Equal
parse_expression: starting at position 16 (Ident(e))
consume_token: position 16 consumed Ident
parse_primary: success - parsed identifier (e)
parse_expression: success - parsed precedence expression
consume_token: position 17 consumed Semicolon
parse_statement: success - parsed assignment
consume_token: position 18 consumed RightBrace
parse_expression: success - parsed precedence expression
parse_statement: success - parsed block-like expression statement
parse_program: parsing statement at position 19 (Ident(assert))
parse_statement: starting at position 19 (Ident(assert))
consume_token: position 19 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 19 (Ident(assert))
consume_token: position 19 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 20 consumed LeftParen
parse_expression: starting at position 21 (Ident(i))
consume_token: position 21 consumed Ident
parse_primary: success - parsed identifier (i)
consume_token: position 22 consumed EqualEqual
consume_token: position 23 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 24 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 25 consumed Semicolon
parse_program: parsed 3 statements
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
        1: Token(Ident, "i"),
        2: Token(Semicolon),
        3: Token(Try),
        4: Token(LeftBrace),
        5: Token(Ident, "raise"),
        6: Token(LeftParen),
        7: Token(Int, "42"),
        8: Token(RightParen),
        9: Token(Semicolon),
        10: Token(RightBrace),
        11: Token(Catch),
        12: Token(Ident, "e"),
        13: Token(LeftBrace),
        14: Token(Ident, "i"),
        15: Token(Equal),
        16: Token(Ident, "e"),
        17: Token(Semicolon),
        18: Token(RightBrace),
        19: Token(Ident, "assert"),
        20: Token(LeftParen),
        21: Token(Ident, "i"),
        22: Token(EqualEqual),
        23: Token(Int, "42"),
        24: Token(RightParen),
        25: Token(Semicolon),
        26: Token(Eof)
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
                    "i",
                ),
                value: None,
            },
            Expression(
                Try {
                    body: Block {
                        statements: [
                            Expression(
                                FunctionCall(
                                    Identifier(
                                        "raise",
                                    ),
                                    [
                                        Literal(
                                            Int(
                                                42,
                                            ),
                                        ),
                                    ],
                                ),
                            ),
                        ],
                        final_expr: None,
                    },
                    catch: CatchClause {
                        pattern: Variable(
                            "e",
                        ),
                        body: Block {
                            statements: [
                                Assignment {
                                    target: Identifier(
                                        "i",
                                    ),
                                    op: Assign,
                                    value: Identifier(
                                        "e",
                                    ),
                                },
                            ],
                            final_expr: None,
                        },
                    },
                },
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Identifier(
                                "i",
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