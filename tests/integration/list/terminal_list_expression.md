# Program
Status: 🟢
Assertions: 1

```rustleaf
fn f() {
    var j = [10, 20];
    for i in j {
        i;
    }
    [1, 2, 3]
}
// This used to parse as "for_loop[1, 2, 3]", an indexing operation
assert(f() == [1, 2, 3]);
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
parse_statement: starting at position 5 (Var)
consume_token: position 5 consumed Var
consume_token: position 6 consumed Ident
consume_token: position 7 consumed Equal
parse_expression: starting at position 8 (LeftBracket)
consume_token: position 8 consumed LeftBracket
parse_primary: success - parsing list literal
parse_list_literal: starting at position 9
parse_list_literal: parsing element at position 9
parse_expression: starting at position 9 (Int(10))
consume_token: position 9 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 10 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 11
parse_expression: starting at position 11 (Int(20))
consume_token: position 11 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_list_literal: no comma, expecting ]
parse_list_literal: expecting ] at position 12
consume_token: position 12 consumed RightBracket
parse_list_literal: success
parse_expression: success - parsed precedence expression
consume_token: position 13 consumed Semicolon
parse_statement: success - parsed var declaration
parse_statement: starting at position 14 (For)
parse_expression: starting at position 14 (For)
consume_token: position 14 consumed For
parse_primary: success - parsing for expression
consume_token: position 15 consumed Ident
consume_token: position 16 consumed In
parse_expression: starting at position 17 (Ident(j))
consume_token: position 17 consumed Ident
parse_primary: success - parsed identifier (j)
parse_expression: success - parsed precedence expression
consume_token: position 18 consumed LeftBrace
parse_statement: starting at position 19 (Ident(i))
consume_token: position 19 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 19 (Ident(i))
consume_token: position 19 consumed Ident
parse_primary: success - parsed identifier (i)
parse_expression: success - parsed precedence expression
consume_token: position 20 consumed Semicolon
consume_token: position 21 consumed RightBrace
parse_expression: success - parsed precedence expression
parse_statement: success - parsed block-like expression statement
parse_statement: starting at position 22 (LeftBracket)
parse_statement: falling back to expression statement
parse_expression: starting at position 22 (LeftBracket)
consume_token: position 22 consumed LeftBracket
parse_primary: success - parsing list literal
parse_list_literal: starting at position 23
parse_list_literal: parsing element at position 23
parse_expression: starting at position 23 (Int(1))
consume_token: position 23 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 24 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 25
parse_expression: starting at position 25 (Int(2))
consume_token: position 25 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 26 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 27
parse_expression: starting at position 27 (Int(3))
consume_token: position 27 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_list_literal: no comma, expecting ]
parse_list_literal: expecting ] at position 28
consume_token: position 28 consumed RightBracket
parse_list_literal: success
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 29
parse_expression: starting at position 22 (LeftBracket)
consume_token: position 22 consumed LeftBracket
parse_primary: success - parsing list literal
parse_list_literal: starting at position 23
parse_list_literal: parsing element at position 23
parse_expression: starting at position 23 (Int(1))
consume_token: position 23 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 24 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 25
parse_expression: starting at position 25 (Int(2))
consume_token: position 25 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 26 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 27
parse_expression: starting at position 27 (Int(3))
consume_token: position 27 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_list_literal: no comma, expecting ]
parse_list_literal: expecting ] at position 28
consume_token: position 28 consumed RightBracket
parse_list_literal: success
parse_expression: success - parsed precedence expression
consume_token: position 29 consumed RightBrace
parse_statement: success - parsed function declaration
parse_program: parsing statement at position 30 (Ident(assert))
parse_statement: starting at position 30 (Ident(assert))
consume_token: position 30 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 30 (Ident(assert))
consume_token: position 30 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 31 consumed LeftParen
parse_expression: starting at position 32 (Ident(f))
consume_token: position 32 consumed Ident
parse_primary: success - parsed identifier (f)
consume_token: position 33 consumed LeftParen
consume_token: position 34 consumed RightParen
consume_token: position 35 consumed EqualEqual
consume_token: position 36 consumed LeftBracket
parse_primary: success - parsing list literal
parse_list_literal: starting at position 37
parse_list_literal: parsing element at position 37
parse_expression: starting at position 37 (Int(1))
consume_token: position 37 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 38 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 39
parse_expression: starting at position 39 (Int(2))
consume_token: position 39 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 40 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 41
parse_expression: starting at position 41 (Int(3))
consume_token: position 41 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_list_literal: no comma, expecting ]
parse_list_literal: expecting ] at position 42
consume_token: position 42 consumed RightBracket
parse_list_literal: success
parse_expression: success - parsed precedence expression
consume_token: position 43 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 44 consumed Semicolon
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
        1: Token(Ident, "f"),
        2: Token(LeftParen),
        3: Token(RightParen),
        4: Token(LeftBrace),
        5: Token(Var),
        6: Token(Ident, "j"),
        7: Token(Equal),
        8: Token(LeftBracket),
        9: Token(Int, "10"),
        10: Token(Comma),
        11: Token(Int, "20"),
        12: Token(RightBracket),
        13: Token(Semicolon),
        14: Token(For),
        15: Token(Ident, "i"),
        16: Token(In),
        17: Token(Ident, "j"),
        18: Token(LeftBrace),
        19: Token(Ident, "i"),
        20: Token(Semicolon),
        21: Token(RightBrace),
        22: Token(LeftBracket),
        23: Token(Int, "1"),
        24: Token(Comma),
        25: Token(Int, "2"),
        26: Token(Comma),
        27: Token(Int, "3"),
        28: Token(RightBracket),
        29: Token(RightBrace),
        30: Token(Ident, "assert"),
        31: Token(LeftParen),
        32: Token(Ident, "f"),
        33: Token(LeftParen),
        34: Token(RightParen),
        35: Token(EqualEqual),
        36: Token(LeftBracket),
        37: Token(Int, "1"),
        38: Token(Comma),
        39: Token(Int, "2"),
        40: Token(Comma),
        41: Token(Int, "3"),
        42: Token(RightBracket),
        43: Token(RightParen),
        44: Token(Semicolon),
        45: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            FnDecl {
                name: "f",
                params: [],
                body: Block {
                    statements: [
                        VarDecl {
                            pattern: Variable(
                                "j",
                            ),
                            value: Some(
                                List(
                                    [
                                        Literal(
                                            Int(
                                                10,
                                            ),
                                        ),
                                        Literal(
                                            Int(
                                                20,
                                            ),
                                        ),
                                    ],
                                ),
                            ),
                        },
                        Expression(
                            For {
                                pattern: Variable(
                                    "i",
                                ),
                                iter: Identifier(
                                    "j",
                                ),
                                body: Block {
                                    statements: [
                                        Expression(
                                            Identifier(
                                                "i",
                                            ),
                                        ),
                                    ],
                                    final_expr: None,
                                },
                            },
                        ),
                    ],
                    final_expr: Some(
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
                                    "f",
                                ),
                                [],
                            ),
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