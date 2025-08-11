# Program
Status: 🟢
Assertions: 3

```rustleaf
var [a, b, c] = [1, 2, 3];
assert(a == 1);
assert(b == 2);
assert(c == 3);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
consume_token: position 0 consumed Var
consume_token: position 1 consumed LeftBracket
consume_token: position 2 consumed Ident
consume_token: position 3 consumed Comma
consume_token: position 4 consumed Ident
consume_token: position 5 consumed Comma
consume_token: position 6 consumed Ident
consume_token: position 7 consumed RightBracket
consume_token: position 8 consumed Equal
parse_expression: starting at position 9 (LeftBracket)
consume_token: position 9 consumed LeftBracket
parse_primary: success - parsing list literal
parse_list_literal: starting at position 10
parse_list_literal: parsing element at position 10
parse_expression: starting at position 10 (Int(1))
consume_token: position 10 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 11 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 12
parse_expression: starting at position 12 (Int(2))
consume_token: position 12 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 13 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 14
parse_expression: starting at position 14 (Int(3))
consume_token: position 14 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_list_literal: no comma, expecting ]
parse_list_literal: expecting ] at position 15
consume_token: position 15 consumed RightBracket
parse_list_literal: success
parse_expression: success - parsed precedence expression
consume_token: position 16 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 17 (Ident(assert))
parse_statement: starting at position 17 (Ident(assert))
consume_token: position 17 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 17 (Ident(assert))
consume_token: position 17 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 18 consumed LeftParen
parse_expression: starting at position 19 (Ident(a))
consume_token: position 19 consumed Ident
parse_primary: success - parsed identifier (a)
consume_token: position 20 consumed EqualEqual
consume_token: position 21 consumed Int
parse_primary: success - parsed numeric/string literal
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
parse_expression: starting at position 26 (Ident(b))
consume_token: position 26 consumed Ident
parse_primary: success - parsed identifier (b)
consume_token: position 27 consumed EqualEqual
consume_token: position 28 consumed Int
parse_primary: success - parsed numeric/string literal
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
parse_expression: starting at position 33 (Ident(c))
consume_token: position 33 consumed Ident
parse_primary: success - parsed identifier (c)
consume_token: position 34 consumed EqualEqual
consume_token: position 35 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 36 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 37 consumed Semicolon
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
        1: Token(LeftBracket),
        2: Token(Ident, "a"),
        3: Token(Comma),
        4: Token(Ident, "b"),
        5: Token(Comma),
        6: Token(Ident, "c"),
        7: Token(RightBracket),
        8: Token(Equal),
        9: Token(LeftBracket),
        10: Token(Int, "1"),
        11: Token(Comma),
        12: Token(Int, "2"),
        13: Token(Comma),
        14: Token(Int, "3"),
        15: Token(RightBracket),
        16: Token(Semicolon),
        17: Token(Ident, "assert"),
        18: Token(LeftParen),
        19: Token(Ident, "a"),
        20: Token(EqualEqual),
        21: Token(Int, "1"),
        22: Token(RightParen),
        23: Token(Semicolon),
        24: Token(Ident, "assert"),
        25: Token(LeftParen),
        26: Token(Ident, "b"),
        27: Token(EqualEqual),
        28: Token(Int, "2"),
        29: Token(RightParen),
        30: Token(Semicolon),
        31: Token(Ident, "assert"),
        32: Token(LeftParen),
        33: Token(Ident, "c"),
        34: Token(EqualEqual),
        35: Token(Int, "3"),
        36: Token(RightParen),
        37: Token(Semicolon),
        38: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            VarDecl {
                pattern: List(
                    [
                        Variable(
                            "a",
                        ),
                        Variable(
                            "b",
                        ),
                        Variable(
                            "c",
                        ),
                    ],
                ),
                value: Some(
                    List(
                        [
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                            Literal(
                                Int(
                                    2,
                                ),
                            ),
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                        ],
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
                                "a",
                            ),
                            Literal(
                                Int(
                                    1,
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
                                "b",
                            ),
                            Literal(
                                Int(
                                    2,
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
                                "c",
                            ),
                            Literal(
                                Int(
                                    3,
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