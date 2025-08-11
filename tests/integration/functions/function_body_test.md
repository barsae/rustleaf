# Program
Status: 🟢
Assertions: 1

```rustleaf
fn greet() {
    "hello"
}
assert(greet() == "hello");
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Fn)
parse_statement: starting at position 0 (Fn)
consume_token: position 0 consumed Fn
consume_token: position 1 consumed Ident
consume_token: position 2 consumed LeftParen
consume_token: position 3 consumed RightParen
consume_token: position 4 consumed LeftBrace
parse_statement: starting at position 5 (String(hello))
parse_statement: falling back to expression statement
parse_expression: starting at position 5 (String(hello))
consume_token: position 5 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 6
parse_expression: starting at position 5 (String(hello))
consume_token: position 5 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 6 consumed RightBrace
parse_statement: success - parsed function declaration
parse_program: parsing statement at position 7 (Ident(assert))
parse_statement: starting at position 7 (Ident(assert))
consume_token: position 7 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 7 (Ident(assert))
consume_token: position 7 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 8 consumed LeftParen
parse_expression: starting at position 9 (Ident(greet))
consume_token: position 9 consumed Ident
parse_primary: success - parsed identifier (greet)
consume_token: position 10 consumed LeftParen
consume_token: position 11 consumed RightParen
consume_token: position 12 consumed EqualEqual
consume_token: position 13 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 14 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 15 consumed Semicolon
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
        0: Token(Fn),
        1: Token(Ident, "greet"),
        2: Token(LeftParen),
        3: Token(RightParen),
        4: Token(LeftBrace),
        5: Token(String, "hello"),
        6: Token(RightBrace),
        7: Token(Ident, "assert"),
        8: Token(LeftParen),
        9: Token(Ident, "greet"),
        10: Token(LeftParen),
        11: Token(RightParen),
        12: Token(EqualEqual),
        13: Token(String, "hello"),
        14: Token(RightParen),
        15: Token(Semicolon),
        16: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            FnDecl {
                name: "greet",
                params: [],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Literal(
                            String(
                                "hello",
                            ),
                        ),
                    ),
                },
                is_pub: false,
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
                                    "greet",
                                ),
                                [],
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