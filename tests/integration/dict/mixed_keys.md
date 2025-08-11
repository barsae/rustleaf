# Program
Status: 🟢
Assertions: 3

```rustleaf
// Dict access with different key types
var my_dict = {"a": 1, "b": 2};
assert(my_dict["a"] == 1);

// Mixed key types
var mixed = {1: "one", "two": 2, true: "yes"};
assert(mixed[1] == "one");
assert(mixed["two"] == 2);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
consume_token: position 0 consumed Var
consume_token: position 1 consumed Ident
consume_token: position 2 consumed Equal
parse_expression: starting at position 3 (LeftBrace)
parse_primary: success - parsing block or dict
consume_token: position 3 consumed LeftBrace
consume_token: position 4 consumed String
parse_primary: success - parsed numeric/string literal
consume_token: position 5 consumed Colon
parse_expression: starting at position 6 (Int(1))
consume_token: position 6 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 7 consumed Comma
consume_token: position 8 consumed String
parse_primary: success - parsed numeric/string literal
consume_token: position 9 consumed Colon
parse_expression: starting at position 10 (Int(2))
consume_token: position 10 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 11 consumed RightBrace
parse_expression: success - parsed precedence expression
consume_token: position 12 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 13 (Ident(assert))
parse_statement: starting at position 13 (Ident(assert))
consume_token: position 13 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 13 (Ident(assert))
consume_token: position 13 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 14 consumed LeftParen
parse_expression: starting at position 15 (Ident(my_dict))
consume_token: position 15 consumed Ident
parse_primary: success - parsed identifier (my_dict)
consume_token: position 16 consumed LeftBracket
parse_expression: starting at position 17 (String(a))
consume_token: position 17 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 18 consumed RightBracket
consume_token: position 19 consumed EqualEqual
consume_token: position 20 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 21 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 22 consumed Semicolon
parse_program: parsing statement at position 23 (Var)
parse_statement: starting at position 23 (Var)
consume_token: position 23 consumed Var
consume_token: position 24 consumed Ident
consume_token: position 25 consumed Equal
parse_expression: starting at position 26 (LeftBrace)
parse_primary: success - parsing block or dict
consume_token: position 26 consumed LeftBrace
consume_token: position 27 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 28 consumed Colon
parse_expression: starting at position 29 (String(one))
consume_token: position 29 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 30 consumed Comma
consume_token: position 31 consumed String
parse_primary: success - parsed numeric/string literal
consume_token: position 32 consumed Colon
parse_expression: starting at position 33 (Int(2))
consume_token: position 33 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 34 consumed Comma
consume_token: position 35 consumed True
parse_primary: success - parsed boolean literal (true)
consume_token: position 36 consumed Colon
parse_expression: starting at position 37 (String(yes))
consume_token: position 37 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 38 consumed RightBrace
parse_expression: success - parsed precedence expression
consume_token: position 39 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 40 (Ident(assert))
parse_statement: starting at position 40 (Ident(assert))
consume_token: position 40 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 40 (Ident(assert))
consume_token: position 40 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 41 consumed LeftParen
parse_expression: starting at position 42 (Ident(mixed))
consume_token: position 42 consumed Ident
parse_primary: success - parsed identifier (mixed)
consume_token: position 43 consumed LeftBracket
parse_expression: starting at position 44 (Int(1))
consume_token: position 44 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 45 consumed RightBracket
consume_token: position 46 consumed EqualEqual
consume_token: position 47 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 48 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 49 consumed Semicolon
parse_program: parsing statement at position 50 (Ident(assert))
parse_statement: starting at position 50 (Ident(assert))
consume_token: position 50 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 50 (Ident(assert))
consume_token: position 50 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 51 consumed LeftParen
parse_expression: starting at position 52 (Ident(mixed))
consume_token: position 52 consumed Ident
parse_primary: success - parsed identifier (mixed)
consume_token: position 53 consumed LeftBracket
parse_expression: starting at position 54 (String(two))
consume_token: position 54 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 55 consumed RightBracket
consume_token: position 56 consumed EqualEqual
consume_token: position 57 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 58 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 59 consumed Semicolon
parse_program: parsed 5 statements
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
        1: Token(Ident, "my_dict"),
        2: Token(Equal),
        3: Token(LeftBrace),
        4: Token(String, "a"),
        5: Token(Colon),
        6: Token(Int, "1"),
        7: Token(Comma),
        8: Token(String, "b"),
        9: Token(Colon),
        10: Token(Int, "2"),
        11: Token(RightBrace),
        12: Token(Semicolon),
        13: Token(Ident, "assert"),
        14: Token(LeftParen),
        15: Token(Ident, "my_dict"),
        16: Token(LeftBracket),
        17: Token(String, "a"),
        18: Token(RightBracket),
        19: Token(EqualEqual),
        20: Token(Int, "1"),
        21: Token(RightParen),
        22: Token(Semicolon),
        23: Token(Var),
        24: Token(Ident, "mixed"),
        25: Token(Equal),
        26: Token(LeftBrace),
        27: Token(Int, "1"),
        28: Token(Colon),
        29: Token(String, "one"),
        30: Token(Comma),
        31: Token(String, "two"),
        32: Token(Colon),
        33: Token(Int, "2"),
        34: Token(Comma),
        35: Token(True),
        36: Token(Colon),
        37: Token(String, "yes"),
        38: Token(RightBrace),
        39: Token(Semicolon),
        40: Token(Ident, "assert"),
        41: Token(LeftParen),
        42: Token(Ident, "mixed"),
        43: Token(LeftBracket),
        44: Token(Int, "1"),
        45: Token(RightBracket),
        46: Token(EqualEqual),
        47: Token(String, "one"),
        48: Token(RightParen),
        49: Token(Semicolon),
        50: Token(Ident, "assert"),
        51: Token(LeftParen),
        52: Token(Ident, "mixed"),
        53: Token(LeftBracket),
        54: Token(String, "two"),
        55: Token(RightBracket),
        56: Token(EqualEqual),
        57: Token(Int, "2"),
        58: Token(RightParen),
        59: Token(Semicolon),
        60: Token(Eof)
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
                    "my_dict",
                ),
                value: Some(
                    Dict(
                        [
                            (
                                Literal(
                                    String(
                                        "a",
                                    ),
                                ),
                                Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                            ),
                            (
                                Literal(
                                    String(
                                        "b",
                                    ),
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
                                    "my_dict",
                                ),
                                Literal(
                                    String(
                                        "a",
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
            VarDecl {
                pattern: Variable(
                    "mixed",
                ),
                value: Some(
                    Dict(
                        [
                            (
                                Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                                Literal(
                                    String(
                                        "one",
                                    ),
                                ),
                            ),
                            (
                                Literal(
                                    String(
                                        "two",
                                    ),
                                ),
                                Literal(
                                    Int(
                                        2,
                                    ),
                                ),
                            ),
                            (
                                Literal(
                                    Bool(
                                        true,
                                    ),
                                ),
                                Literal(
                                    String(
                                        "yes",
                                    ),
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
                                    "mixed",
                                ),
                                Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                            ),
                            Literal(
                                String(
                                    "one",
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
                                    "mixed",
                                ),
                                Literal(
                                    String(
                                        "two",
                                    ),
                                ),
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