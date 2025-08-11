# Program
Status: 🟢
Assertions: 6

```rustleaf
// Test is_unit with various values
assert(not is_unit(42));
assert(not is_unit("hello"));
assert(not is_unit(true));
assert(not is_unit([]));
assert(not is_unit({}));

// Test with a function that returns Unit
fn side_effect() {
    var x = 1;
}
assert(is_unit(side_effect()));
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Ident(assert))
parse_statement: starting at position 0 (Ident(assert))
consume_token: position 0 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 0 (Ident(assert))
consume_token: position 0 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 1 consumed LeftParen
parse_expression: starting at position 2 (Not)
consume_token: position 2 consumed Not
consume_token: position 3 consumed Ident
parse_primary: success - parsed identifier (is_unit)
consume_token: position 4 consumed LeftParen
parse_expression: starting at position 5 (Int(42))
consume_token: position 5 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 6 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 7 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 8 consumed Semicolon
parse_program: parsing statement at position 9 (Ident(assert))
parse_statement: starting at position 9 (Ident(assert))
consume_token: position 9 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 9 (Ident(assert))
consume_token: position 9 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 10 consumed LeftParen
parse_expression: starting at position 11 (Not)
consume_token: position 11 consumed Not
consume_token: position 12 consumed Ident
parse_primary: success - parsed identifier (is_unit)
consume_token: position 13 consumed LeftParen
parse_expression: starting at position 14 (String(hello))
consume_token: position 14 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 15 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 16 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 17 consumed Semicolon
parse_program: parsing statement at position 18 (Ident(assert))
parse_statement: starting at position 18 (Ident(assert))
consume_token: position 18 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 18 (Ident(assert))
consume_token: position 18 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 19 consumed LeftParen
parse_expression: starting at position 20 (Not)
consume_token: position 20 consumed Not
consume_token: position 21 consumed Ident
parse_primary: success - parsed identifier (is_unit)
consume_token: position 22 consumed LeftParen
parse_expression: starting at position 23 (True)
consume_token: position 23 consumed True
parse_primary: success - parsed boolean literal (true)
parse_expression: success - parsed precedence expression
consume_token: position 24 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 25 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 26 consumed Semicolon
parse_program: parsing statement at position 27 (Ident(assert))
parse_statement: starting at position 27 (Ident(assert))
consume_token: position 27 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 27 (Ident(assert))
consume_token: position 27 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 28 consumed LeftParen
parse_expression: starting at position 29 (Not)
consume_token: position 29 consumed Not
consume_token: position 30 consumed Ident
parse_primary: success - parsed identifier (is_unit)
consume_token: position 31 consumed LeftParen
parse_expression: starting at position 32 (LeftBracket)
consume_token: position 32 consumed LeftBracket
parse_primary: success - parsing list literal
parse_list_literal: starting at position 33
consume_token: position 33 consumed RightBracket
parse_list_literal: empty list
parse_expression: success - parsed precedence expression
consume_token: position 34 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 35 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 36 consumed Semicolon
parse_program: parsing statement at position 37 (Ident(assert))
parse_statement: starting at position 37 (Ident(assert))
consume_token: position 37 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 37 (Ident(assert))
consume_token: position 37 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 38 consumed LeftParen
parse_expression: starting at position 39 (Not)
consume_token: position 39 consumed Not
consume_token: position 40 consumed Ident
parse_primary: success - parsed identifier (is_unit)
consume_token: position 41 consumed LeftParen
parse_expression: starting at position 42 (LeftBrace)
parse_primary: success - parsing block or dict
consume_token: position 42 consumed LeftBrace
consume_token: position 43 consumed RightBrace
parse_expression: success - parsed precedence expression
consume_token: position 44 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 45 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 46 consumed Semicolon
parse_program: parsing statement at position 47 (Fn)
parse_statement: starting at position 47 (Fn)
consume_token: position 47 consumed Fn
consume_token: position 48 consumed Ident
consume_token: position 49 consumed LeftParen
consume_token: position 50 consumed RightParen
consume_token: position 51 consumed LeftBrace
parse_statement: starting at position 52 (Var)
consume_token: position 52 consumed Var
consume_token: position 53 consumed Ident
consume_token: position 54 consumed Equal
parse_expression: starting at position 55 (Int(1))
consume_token: position 55 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 56 consumed Semicolon
parse_statement: success - parsed var declaration
consume_token: position 57 consumed RightBrace
parse_statement: success - parsed function declaration
parse_program: parsing statement at position 58 (Ident(assert))
parse_statement: starting at position 58 (Ident(assert))
consume_token: position 58 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 58 (Ident(assert))
consume_token: position 58 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 59 consumed LeftParen
parse_expression: starting at position 60 (Ident(is_unit))
consume_token: position 60 consumed Ident
parse_primary: success - parsed identifier (is_unit)
consume_token: position 61 consumed LeftParen
parse_expression: starting at position 62 (Ident(side_effect))
consume_token: position 62 consumed Ident
parse_primary: success - parsed identifier (side_effect)
consume_token: position 63 consumed LeftParen
consume_token: position 64 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 65 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 66 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 67 consumed Semicolon
parse_program: parsed 7 statements
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
        0: Token(Ident, "assert"),
        1: Token(LeftParen),
        2: Token(Not),
        3: Token(Ident, "is_unit"),
        4: Token(LeftParen),
        5: Token(Int, "42"),
        6: Token(RightParen),
        7: Token(RightParen),
        8: Token(Semicolon),
        9: Token(Ident, "assert"),
        10: Token(LeftParen),
        11: Token(Not),
        12: Token(Ident, "is_unit"),
        13: Token(LeftParen),
        14: Token(String, "hello"),
        15: Token(RightParen),
        16: Token(RightParen),
        17: Token(Semicolon),
        18: Token(Ident, "assert"),
        19: Token(LeftParen),
        20: Token(Not),
        21: Token(Ident, "is_unit"),
        22: Token(LeftParen),
        23: Token(True),
        24: Token(RightParen),
        25: Token(RightParen),
        26: Token(Semicolon),
        27: Token(Ident, "assert"),
        28: Token(LeftParen),
        29: Token(Not),
        30: Token(Ident, "is_unit"),
        31: Token(LeftParen),
        32: Token(LeftBracket),
        33: Token(RightBracket),
        34: Token(RightParen),
        35: Token(RightParen),
        36: Token(Semicolon),
        37: Token(Ident, "assert"),
        38: Token(LeftParen),
        39: Token(Not),
        40: Token(Ident, "is_unit"),
        41: Token(LeftParen),
        42: Token(LeftBrace),
        43: Token(RightBrace),
        44: Token(RightParen),
        45: Token(RightParen),
        46: Token(Semicolon),
        47: Token(Fn),
        48: Token(Ident, "side_effect"),
        49: Token(LeftParen),
        50: Token(RightParen),
        51: Token(LeftBrace),
        52: Token(Var),
        53: Token(Ident, "x"),
        54: Token(Equal),
        55: Token(Int, "1"),
        56: Token(Semicolon),
        57: Token(RightBrace),
        58: Token(Ident, "assert"),
        59: Token(LeftParen),
        60: Token(Ident, "is_unit"),
        61: Token(LeftParen),
        62: Token(Ident, "side_effect"),
        63: Token(LeftParen),
        64: Token(RightParen),
        65: Token(RightParen),
        66: Token(RightParen),
        67: Token(Semicolon),
        68: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Not(
                            FunctionCall(
                                Identifier(
                                    "is_unit",
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
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Not(
                            FunctionCall(
                                Identifier(
                                    "is_unit",
                                ),
                                [
                                    Literal(
                                        String(
                                            "hello",
                                        ),
                                    ),
                                ],
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
                        Not(
                            FunctionCall(
                                Identifier(
                                    "is_unit",
                                ),
                                [
                                    Literal(
                                        Bool(
                                            true,
                                        ),
                                    ),
                                ],
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
                        Not(
                            FunctionCall(
                                Identifier(
                                    "is_unit",
                                ),
                                [
                                    List(
                                        [],
                                    ),
                                ],
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
                        Not(
                            FunctionCall(
                                Identifier(
                                    "is_unit",
                                ),
                                [
                                    Dict(
                                        [],
                                    ),
                                ],
                            ),
                        ),
                    ],
                ),
            ),
            FnDecl {
                name: "side_effect",
                params: [],
                body: Block {
                    statements: [
                        VarDecl {
                            pattern: Variable(
                                "x",
                            ),
                            value: Some(
                                Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                            ),
                        },
                    ],
                    final_expr: None,
                },
                is_pub: false,
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        FunctionCall(
                            Identifier(
                                "is_unit",
                            ),
                            [
                                FunctionCall(
                                    Identifier(
                                        "side_effect",
                                    ),
                                    [],
                                ),
                            ],
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