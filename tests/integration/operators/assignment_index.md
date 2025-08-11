# Program
Status: 🟢
Assertions: 1

```rustleaf
var arr = [1, 2, 3];
arr[0] = 99;
assert(arr == [99, 2, 3]);
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
parse_list_literal: parsing element at position 4
parse_expression: starting at position 4 (Int(1))
consume_token: position 4 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 5 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 6
parse_expression: starting at position 6 (Int(2))
consume_token: position 6 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 7 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 8
parse_expression: starting at position 8 (Int(3))
consume_token: position 8 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_list_literal: no comma, expecting ]
parse_list_literal: expecting ] at position 9
consume_token: position 9 consumed RightBracket
parse_list_literal: success
parse_expression: success - parsed precedence expression
consume_token: position 10 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 11 (Ident(arr))
parse_statement: starting at position 11 (Ident(arr))
consume_token: position 11 consumed Ident
consume_token: position 12 consumed LeftBracket
parse_expression: starting at position 13 (Int(0))
consume_token: position 13 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 14 consumed RightBracket
consume_token: position 15 consumed Equal
parse_expression: starting at position 16 (Int(99))
consume_token: position 16 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 17 consumed Semicolon
parse_statement: success - parsed assignment
parse_program: parsing statement at position 18 (Ident(assert))
parse_statement: starting at position 18 (Ident(assert))
consume_token: position 18 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 18 (Ident(assert))
consume_token: position 18 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 19 consumed LeftParen
parse_expression: starting at position 20 (Ident(arr))
consume_token: position 20 consumed Ident
parse_primary: success - parsed identifier (arr)
consume_token: position 21 consumed EqualEqual
consume_token: position 22 consumed LeftBracket
parse_primary: success - parsing list literal
parse_list_literal: starting at position 23
parse_list_literal: parsing element at position 23
parse_expression: starting at position 23 (Int(99))
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
consume_token: position 29 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 30 consumed Semicolon
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
        1: Token(Ident, "arr"),
        2: Token(Equal),
        3: Token(LeftBracket),
        4: Token(Int, "1"),
        5: Token(Comma),
        6: Token(Int, "2"),
        7: Token(Comma),
        8: Token(Int, "3"),
        9: Token(RightBracket),
        10: Token(Semicolon),
        11: Token(Ident, "arr"),
        12: Token(LeftBracket),
        13: Token(Int, "0"),
        14: Token(RightBracket),
        15: Token(Equal),
        16: Token(Int, "99"),
        17: Token(Semicolon),
        18: Token(Ident, "assert"),
        19: Token(LeftParen),
        20: Token(Ident, "arr"),
        21: Token(EqualEqual),
        22: Token(LeftBracket),
        23: Token(Int, "99"),
        24: Token(Comma),
        25: Token(Int, "2"),
        26: Token(Comma),
        27: Token(Int, "3"),
        28: Token(RightBracket),
        29: Token(RightParen),
        30: Token(Semicolon),
        31: Token(Eof)
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
                    "arr",
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
            Assignment {
                target: GetItem(
                    Identifier(
                        "arr",
                    ),
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
                op: Assign,
                value: Literal(
                    Int(
                        99,
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
                                "arr",
                            ),
                            List(
                                [
                                    Literal(
                                        Int(
                                            99,
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