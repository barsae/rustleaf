# Program
Status: 🟡
Assertions: 0

```rustleaf
42
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Int(42))
parse_statement: starting at position 0 (Int(42))
parse_statement: falling back to expression statement
parse_expression: starting at position 0 (Int(42))
consume_token: position 0 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found Eof at position 1
parse_expression: starting at position 0 (Int(42))
consume_token: position 0 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_program: parsed 1 statements
```

# Result
```rust
Ok(
    Int(42),
)
```

# Lex
```rust
Ok(
    [
        0: Token(Int, "42"),
        1: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            Expression(
                Literal(
                    Int(
                        42,
                    ),
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