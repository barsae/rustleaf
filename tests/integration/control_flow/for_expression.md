# Program
Status: 🟢
Assertions: 1

```rustleaf
var s = 0;
for x in [1, 2, 3] {
    s += x;
}
assert(s == 6);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
consume_token: position 0 consumed Var
consume_token: position 1 consumed Ident
consume_token: position 2 consumed Equal
parse_expression: starting at position 3 (Int(0))
consume_token: position 3 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 4 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 5 (For)
parse_statement: starting at position 5 (For)
parse_expression: starting at position 5 (For)
consume_token: position 5 consumed For
parse_primary: success - parsing for expression
consume_token: position 6 consumed Ident
consume_token: position 7 consumed In
parse_expression: starting at position 8 (LeftBracket)
consume_token: position 8 consumed LeftBracket
parse_primary: success - parsing list literal
parse_list_literal: starting at position 9
parse_list_literal: parsing element at position 9
parse_expression: starting at position 9 (Int(1))
consume_token: position 9 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 10 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 11
parse_expression: starting at position 11 (Int(2))
consume_token: position 11 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 12 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 13
parse_expression: starting at position 13 (Int(3))
consume_token: position 13 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_list_literal: no comma, expecting ]
parse_list_literal: expecting ] at position 14
consume_token: position 14 consumed RightBracket
parse_list_literal: success
parse_expression: success - parsed precedence expression
consume_token: position 15 consumed LeftBrace
parse_statement: starting at position 16 (Ident(s))
consume_token: position 16 consumed Ident
consume_token: position 17 consumed PlusEqual
parse_expression: starting at position 18 (Ident(x))
consume_token: position 18 consumed Ident
parse_primary: success - parsed identifier (x)
parse_expression: success - parsed precedence expression
consume_token: position 19 consumed Semicolon
parse_statement: success - parsed assignment
consume_token: position 20 consumed RightBrace
parse_expression: success - parsed precedence expression
parse_statement: success - parsed block-like expression statement
parse_program: parsing statement at position 21 (Ident(assert))
parse_statement: starting at position 21 (Ident(assert))
consume_token: position 21 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 21 (Ident(assert))
consume_token: position 21 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 22 consumed LeftParen
parse_expression: starting at position 23 (Ident(s))
consume_token: position 23 consumed Ident
parse_primary: success - parsed identifier (s)
consume_token: position 24 consumed EqualEqual
consume_token: position 25 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 26 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 27 consumed Semicolon
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
        1: Token(Ident, "s"),
        2: Token(Equal),
        3: Token(Int, "0"),
        4: Token(Semicolon),
        5: Token(For),
        6: Token(Ident, "x"),
        7: Token(In),
        8: Token(LeftBracket),
        9: Token(Int, "1"),
        10: Token(Comma),
        11: Token(Int, "2"),
        12: Token(Comma),
        13: Token(Int, "3"),
        14: Token(RightBracket),
        15: Token(LeftBrace),
        16: Token(Ident, "s"),
        17: Token(PlusEqual),
        18: Token(Ident, "x"),
        19: Token(Semicolon),
        20: Token(RightBrace),
        21: Token(Ident, "assert"),
        22: Token(LeftParen),
        23: Token(Ident, "s"),
        24: Token(EqualEqual),
        25: Token(Int, "6"),
        26: Token(RightParen),
        27: Token(Semicolon),
        28: Token(Eof)
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
                    "s",
                ),
                value: Some(
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
            },
            Expression(
                For {
                    pattern: Variable(
                        "x",
                    ),
                    iter: List(
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
                    body: Block {
                        statements: [
                            Assignment {
                                target: Identifier(
                                    "s",
                                ),
                                op: AddAssign,
                                value: Identifier(
                                    "x",
                                ),
                            },
                        ],
                        final_expr: None,
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
                                "s",
                            ),
                            Literal(
                                Int(
                                    6,
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