# Program
Status: 🟢
Assertions: 3

```rustleaf
var list = [1, 2, 3, "hello", true];
assert(list[0] == 1);
assert(list[3] == "hello");
assert(list[4] == true);
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
consume_token: position 9 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 10
parse_expression: starting at position 10 (String(hello))
consume_token: position 10 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 11 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 12
parse_expression: starting at position 12 (True)
consume_token: position 12 consumed True
parse_primary: success - parsed boolean literal (true)
parse_expression: success - parsed precedence expression
parse_list_literal: no comma, expecting ]
parse_list_literal: expecting ] at position 13
consume_token: position 13 consumed RightBracket
parse_list_literal: success
parse_expression: success - parsed precedence expression
consume_token: position 14 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 15 (Ident(assert))
parse_statement: starting at position 15 (Ident(assert))
consume_token: position 15 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 15 (Ident(assert))
consume_token: position 15 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 16 consumed LeftParen
parse_expression: starting at position 17 (Ident(list))
consume_token: position 17 consumed Ident
parse_primary: success - parsed identifier (list)
consume_token: position 18 consumed LeftBracket
parse_expression: starting at position 19 (Int(0))
consume_token: position 19 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 20 consumed RightBracket
consume_token: position 21 consumed EqualEqual
consume_token: position 22 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 23 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 24 consumed Semicolon
parse_program: parsing statement at position 25 (Ident(assert))
parse_statement: starting at position 25 (Ident(assert))
consume_token: position 25 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 25 (Ident(assert))
consume_token: position 25 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 26 consumed LeftParen
parse_expression: starting at position 27 (Ident(list))
consume_token: position 27 consumed Ident
parse_primary: success - parsed identifier (list)
consume_token: position 28 consumed LeftBracket
parse_expression: starting at position 29 (Int(3))
consume_token: position 29 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 30 consumed RightBracket
consume_token: position 31 consumed EqualEqual
consume_token: position 32 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 33 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 34 consumed Semicolon
parse_program: parsing statement at position 35 (Ident(assert))
parse_statement: starting at position 35 (Ident(assert))
consume_token: position 35 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 35 (Ident(assert))
consume_token: position 35 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 36 consumed LeftParen
parse_expression: starting at position 37 (Ident(list))
consume_token: position 37 consumed Ident
parse_primary: success - parsed identifier (list)
consume_token: position 38 consumed LeftBracket
parse_expression: starting at position 39 (Int(4))
consume_token: position 39 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 40 consumed RightBracket
consume_token: position 41 consumed EqualEqual
consume_token: position 42 consumed True
parse_primary: success - parsed boolean literal (true)
parse_expression: success - parsed precedence expression
consume_token: position 43 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 44 consumed Semicolon
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
        1: Token(Ident, "list"),
        2: Token(Equal),
        3: Token(LeftBracket),
        4: Token(Int, "1"),
        5: Token(Comma),
        6: Token(Int, "2"),
        7: Token(Comma),
        8: Token(Int, "3"),
        9: Token(Comma),
        10: Token(String, "hello"),
        11: Token(Comma),
        12: Token(True),
        13: Token(RightBracket),
        14: Token(Semicolon),
        15: Token(Ident, "assert"),
        16: Token(LeftParen),
        17: Token(Ident, "list"),
        18: Token(LeftBracket),
        19: Token(Int, "0"),
        20: Token(RightBracket),
        21: Token(EqualEqual),
        22: Token(Int, "1"),
        23: Token(RightParen),
        24: Token(Semicolon),
        25: Token(Ident, "assert"),
        26: Token(LeftParen),
        27: Token(Ident, "list"),
        28: Token(LeftBracket),
        29: Token(Int, "3"),
        30: Token(RightBracket),
        31: Token(EqualEqual),
        32: Token(String, "hello"),
        33: Token(RightParen),
        34: Token(Semicolon),
        35: Token(Ident, "assert"),
        36: Token(LeftParen),
        37: Token(Ident, "list"),
        38: Token(LeftBracket),
        39: Token(Int, "4"),
        40: Token(RightBracket),
        41: Token(EqualEqual),
        42: Token(True),
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
            VarDecl {
                pattern: Variable(
                    "list",
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
                            Literal(
                                String(
                                    "hello",
                                ),
                            ),
                            Literal(
                                Bool(
                                    true,
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
                            GetItem(
                                Identifier(
                                    "list",
                                ),
                                Literal(
                                    Int(
                                        0,
                                    ),
                                ),
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
                            GetItem(
                                Identifier(
                                    "list",
                                ),
                                Literal(
                                    Int(
                                        3,
                                    ),
                                ),
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            GetItem(
                                Identifier(
                                    "list",
                                ),
                                Literal(
                                    Int(
                                        4,
                                    ),
                                ),
                            ),
                            Literal(
                                Bool(
                                    true,
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