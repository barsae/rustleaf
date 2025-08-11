# Program
Status: 🟢
Assertions: 1

```rustleaf
var empty = [];
assert(empty == []);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
consume_token: position 0 consumed Var
consume_token: position 1 consumed Ident
consume_token: position 2 consumed Equal
parse_expression: starting at position 3 (LeftBracket)
consume_token: position 3 consumed LeftBracket
parse_primary: success - parsing list literal
parse_list_literal: starting at position 4
consume_token: position 4 consumed RightBracket
parse_list_literal: empty list
parse_expression: success - parsed precedence expression
consume_token: position 5 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 6 (Ident(assert))
parse_statement: starting at position 6 (Ident(assert))
consume_token: position 6 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 6 (Ident(assert))
consume_token: position 6 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 7 consumed LeftParen
parse_expression: starting at position 8 (Ident(empty))
consume_token: position 8 consumed Ident
parse_primary: success - parsed identifier (empty)
consume_token: position 9 consumed EqualEqual
consume_token: position 10 consumed LeftBracket
parse_primary: success - parsing list literal
parse_list_literal: starting at position 11
consume_token: position 11 consumed RightBracket
parse_list_literal: empty list
parse_expression: success - parsed precedence expression
consume_token: position 12 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 13 consumed Semicolon
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
        1: Token(Ident, "empty"),
        2: Token(Equal),
        3: Token(LeftBracket),
        4: Token(RightBracket),
        5: Token(Semicolon),
        6: Token(Ident, "assert"),
        7: Token(LeftParen),
        8: Token(Ident, "empty"),
        9: Token(EqualEqual),
        10: Token(LeftBracket),
        11: Token(RightBracket),
        12: Token(RightParen),
        13: Token(Semicolon),
        14: Token(Eof)
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
                    "empty",
                ),
                value: Some(
                    List(
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
                                "empty",
                            ),
                            List(
                                [],
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