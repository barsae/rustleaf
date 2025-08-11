# Program
Status: 🟡
Assertions: 0

```rustleaf
var x = 5;
var y = 3;
x + y
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
parse_program: parsing statement at position 5 (Var)
parse_statement: starting at position 5 (Var)
consume_token: position 5 consumed Var
consume_token: position 6 consumed Ident
consume_token: position 7 consumed Equal
parse_expression: starting at position 8 (Int(3))
consume_token: position 8 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 9 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 10 (Ident(x))
parse_statement: starting at position 10 (Ident(x))
consume_token: position 10 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 10 (Ident(x))
consume_token: position 10 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 11 consumed Plus
consume_token: position 12 consumed Ident
parse_primary: success - parsed identifier (y)
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found Eof at position 13
parse_expression: starting at position 10 (Ident(x))
consume_token: position 10 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 11 consumed Plus
consume_token: position 12 consumed Ident
parse_primary: success - parsed identifier (y)
parse_expression: success - parsed precedence expression
parse_program: parsed 3 statements
```

# Result
```rust
Ok(
    Int(8),
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
        5: Token(Var),
        6: Token(Ident, "y"),
        7: Token(Equal),
        8: Token(Int, "3"),
        9: Token(Semicolon),
        10: Token(Ident, "x"),
        11: Token(Plus),
        12: Token(Ident, "y"),
        13: Token(Eof)
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
            VarDecl {
                pattern: Variable(
                    "y",
                ),
                value: Some(
                    Literal(
                        Int(
                            3,
                        ),
                    ),
                ),
            },
            Expression(
                Add(
                    Identifier(
                        "x",
                    ),
                    Identifier(
                        "y",
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