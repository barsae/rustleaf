# Program
Status: 🟢
Assertions: 9

```rustleaf
// Test 'in' operator (op_contains)

// String contains
assert("hello" in "hello world");
assert(not ("xyz" in "hello world"));

// List contains
var my_list = [1, 2, 3, "hello"];
assert(2 in my_list);
assert("hello" in my_list);
assert(not (99 in my_list));

// Dict contains (check keys)
var my_dict = {"a": 1, "b": 2, 3: "three"};
assert("a" in my_dict);
assert("b" in my_dict);
assert(3 in my_dict);
assert(not ("z" in my_dict));
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
parse_expression: starting at position 2 (String(hello))
consume_token: position 2 consumed String
parse_primary: success - parsed numeric/string literal
consume_token: position 3 consumed In
consume_token: position 4 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 5 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 6 consumed Semicolon
parse_program: parsing statement at position 7 (Ident(assert))
parse_statement: starting at position 7 (Ident(assert))
consume_token: position 7 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 7 (Ident(assert))
consume_token: position 7 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 8 consumed LeftParen
parse_expression: starting at position 9 (Not)
consume_token: position 9 consumed Not
consume_token: position 10 consumed LeftParen
parse_primary: success - parsing parenthesized expression
parse_expression: starting at position 11 (String(xyz))
consume_token: position 11 consumed String
parse_primary: success - parsed numeric/string literal
consume_token: position 12 consumed In
consume_token: position 13 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 14 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 15 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 16 consumed Semicolon
parse_program: parsing statement at position 17 (Var)
parse_statement: starting at position 17 (Var)
consume_token: position 17 consumed Var
consume_token: position 18 consumed Ident
consume_token: position 19 consumed Equal
parse_expression: starting at position 20 (LeftBracket)
consume_token: position 20 consumed LeftBracket
parse_primary: success - parsing list literal
parse_list_literal: starting at position 21
parse_list_literal: parsing element at position 21
parse_expression: starting at position 21 (Int(1))
consume_token: position 21 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 22 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 23
parse_expression: starting at position 23 (Int(2))
consume_token: position 23 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 24 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 25
parse_expression: starting at position 25 (Int(3))
consume_token: position 25 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 26 consumed Comma
parse_list_literal: found comma, checking for more elements
parse_list_literal: parsing element at position 27
parse_expression: starting at position 27 (String(hello))
consume_token: position 27 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_list_literal: no comma, expecting ]
parse_list_literal: expecting ] at position 28
consume_token: position 28 consumed RightBracket
parse_list_literal: success
parse_expression: success - parsed precedence expression
consume_token: position 29 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 30 (Ident(assert))
parse_statement: starting at position 30 (Ident(assert))
consume_token: position 30 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 30 (Ident(assert))
consume_token: position 30 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 31 consumed LeftParen
parse_expression: starting at position 32 (Int(2))
consume_token: position 32 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 33 consumed In
consume_token: position 34 consumed Ident
parse_primary: success - parsed identifier (my_list)
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
parse_expression: starting at position 39 (String(hello))
consume_token: position 39 consumed String
parse_primary: success - parsed numeric/string literal
consume_token: position 40 consumed In
consume_token: position 41 consumed Ident
parse_primary: success - parsed identifier (my_list)
parse_expression: success - parsed precedence expression
consume_token: position 42 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 43 consumed Semicolon
parse_program: parsing statement at position 44 (Ident(assert))
parse_statement: starting at position 44 (Ident(assert))
consume_token: position 44 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 44 (Ident(assert))
consume_token: position 44 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 45 consumed LeftParen
parse_expression: starting at position 46 (Not)
consume_token: position 46 consumed Not
consume_token: position 47 consumed LeftParen
parse_primary: success - parsing parenthesized expression
parse_expression: starting at position 48 (Int(99))
consume_token: position 48 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 49 consumed In
consume_token: position 50 consumed Ident
parse_primary: success - parsed identifier (my_list)
parse_expression: success - parsed precedence expression
consume_token: position 51 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 52 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 53 consumed Semicolon
parse_program: parsing statement at position 54 (Var)
parse_statement: starting at position 54 (Var)
consume_token: position 54 consumed Var
consume_token: position 55 consumed Ident
consume_token: position 56 consumed Equal
parse_expression: starting at position 57 (LeftBrace)
parse_primary: success - parsing block or dict
consume_token: position 57 consumed LeftBrace
consume_token: position 58 consumed String
parse_primary: success - parsed numeric/string literal
consume_token: position 59 consumed Colon
parse_expression: starting at position 60 (Int(1))
consume_token: position 60 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 61 consumed Comma
consume_token: position 62 consumed String
parse_primary: success - parsed numeric/string literal
consume_token: position 63 consumed Colon
parse_expression: starting at position 64 (Int(2))
consume_token: position 64 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 65 consumed Comma
consume_token: position 66 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 67 consumed Colon
parse_expression: starting at position 68 (String(three))
consume_token: position 68 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 69 consumed RightBrace
parse_expression: success - parsed precedence expression
consume_token: position 70 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 71 (Ident(assert))
parse_statement: starting at position 71 (Ident(assert))
consume_token: position 71 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 71 (Ident(assert))
consume_token: position 71 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 72 consumed LeftParen
parse_expression: starting at position 73 (String(a))
consume_token: position 73 consumed String
parse_primary: success - parsed numeric/string literal
consume_token: position 74 consumed In
consume_token: position 75 consumed Ident
parse_primary: success - parsed identifier (my_dict)
parse_expression: success - parsed precedence expression
consume_token: position 76 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 77 consumed Semicolon
parse_program: parsing statement at position 78 (Ident(assert))
parse_statement: starting at position 78 (Ident(assert))
consume_token: position 78 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 78 (Ident(assert))
consume_token: position 78 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 79 consumed LeftParen
parse_expression: starting at position 80 (String(b))
consume_token: position 80 consumed String
parse_primary: success - parsed numeric/string literal
consume_token: position 81 consumed In
consume_token: position 82 consumed Ident
parse_primary: success - parsed identifier (my_dict)
parse_expression: success - parsed precedence expression
consume_token: position 83 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 84 consumed Semicolon
parse_program: parsing statement at position 85 (Ident(assert))
parse_statement: starting at position 85 (Ident(assert))
consume_token: position 85 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 85 (Ident(assert))
consume_token: position 85 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 86 consumed LeftParen
parse_expression: starting at position 87 (Int(3))
consume_token: position 87 consumed Int
parse_primary: success - parsed numeric/string literal
consume_token: position 88 consumed In
consume_token: position 89 consumed Ident
parse_primary: success - parsed identifier (my_dict)
parse_expression: success - parsed precedence expression
consume_token: position 90 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 91 consumed Semicolon
parse_program: parsing statement at position 92 (Ident(assert))
parse_statement: starting at position 92 (Ident(assert))
consume_token: position 92 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 92 (Ident(assert))
consume_token: position 92 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 93 consumed LeftParen
parse_expression: starting at position 94 (Not)
consume_token: position 94 consumed Not
consume_token: position 95 consumed LeftParen
parse_primary: success - parsing parenthesized expression
parse_expression: starting at position 96 (String(z))
consume_token: position 96 consumed String
parse_primary: success - parsed numeric/string literal
consume_token: position 97 consumed In
consume_token: position 98 consumed Ident
parse_primary: success - parsed identifier (my_dict)
parse_expression: success - parsed precedence expression
consume_token: position 99 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 100 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 101 consumed Semicolon
parse_program: parsed 11 statements
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
        2: Token(String, "hello"),
        3: Token(In),
        4: Token(String, "hello world"),
        5: Token(RightParen),
        6: Token(Semicolon),
        7: Token(Ident, "assert"),
        8: Token(LeftParen),
        9: Token(Not),
        10: Token(LeftParen),
        11: Token(String, "xyz"),
        12: Token(In),
        13: Token(String, "hello world"),
        14: Token(RightParen),
        15: Token(RightParen),
        16: Token(Semicolon),
        17: Token(Var),
        18: Token(Ident, "my_list"),
        19: Token(Equal),
        20: Token(LeftBracket),
        21: Token(Int, "1"),
        22: Token(Comma),
        23: Token(Int, "2"),
        24: Token(Comma),
        25: Token(Int, "3"),
        26: Token(Comma),
        27: Token(String, "hello"),
        28: Token(RightBracket),
        29: Token(Semicolon),
        30: Token(Ident, "assert"),
        31: Token(LeftParen),
        32: Token(Int, "2"),
        33: Token(In),
        34: Token(Ident, "my_list"),
        35: Token(RightParen),
        36: Token(Semicolon),
        37: Token(Ident, "assert"),
        38: Token(LeftParen),
        39: Token(String, "hello"),
        40: Token(In),
        41: Token(Ident, "my_list"),
        42: Token(RightParen),
        43: Token(Semicolon),
        44: Token(Ident, "assert"),
        45: Token(LeftParen),
        46: Token(Not),
        47: Token(LeftParen),
        48: Token(Int, "99"),
        49: Token(In),
        50: Token(Ident, "my_list"),
        51: Token(RightParen),
        52: Token(RightParen),
        53: Token(Semicolon),
        54: Token(Var),
        55: Token(Ident, "my_dict"),
        56: Token(Equal),
        57: Token(LeftBrace),
        58: Token(String, "a"),
        59: Token(Colon),
        60: Token(Int, "1"),
        61: Token(Comma),
        62: Token(String, "b"),
        63: Token(Colon),
        64: Token(Int, "2"),
        65: Token(Comma),
        66: Token(Int, "3"),
        67: Token(Colon),
        68: Token(String, "three"),
        69: Token(RightBrace),
        70: Token(Semicolon),
        71: Token(Ident, "assert"),
        72: Token(LeftParen),
        73: Token(String, "a"),
        74: Token(In),
        75: Token(Ident, "my_dict"),
        76: Token(RightParen),
        77: Token(Semicolon),
        78: Token(Ident, "assert"),
        79: Token(LeftParen),
        80: Token(String, "b"),
        81: Token(In),
        82: Token(Ident, "my_dict"),
        83: Token(RightParen),
        84: Token(Semicolon),
        85: Token(Ident, "assert"),
        86: Token(LeftParen),
        87: Token(Int, "3"),
        88: Token(In),
        89: Token(Ident, "my_dict"),
        90: Token(RightParen),
        91: Token(Semicolon),
        92: Token(Ident, "assert"),
        93: Token(LeftParen),
        94: Token(Not),
        95: Token(LeftParen),
        96: Token(String, "z"),
        97: Token(In),
        98: Token(Ident, "my_dict"),
        99: Token(RightParen),
        100: Token(RightParen),
        101: Token(Semicolon),
        102: Token(Eof)
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
                        In(
                            Literal(
                                String(
                                    "hello",
                                ),
                            ),
                            Literal(
                                String(
                                    "hello world",
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
                        Not(
                            In(
                                Literal(
                                    String(
                                        "xyz",
                                    ),
                                ),
                                Literal(
                                    String(
                                        "hello world",
                                    ),
                                ),
                            ),
                        ),
                    ],
                ),
            ),
            VarDecl {
                pattern: Variable(
                    "my_list",
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
                        In(
                            Literal(
                                Int(
                                    2,
                                ),
                            ),
                            Identifier(
                                "my_list",
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
                        In(
                            Literal(
                                String(
                                    "hello",
                                ),
                            ),
                            Identifier(
                                "my_list",
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
                            In(
                                Literal(
                                    Int(
                                        99,
                                    ),
                                ),
                                Identifier(
                                    "my_list",
                                ),
                            ),
                        ),
                    ],
                ),
            ),
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
                            (
                                Literal(
                                    Int(
                                        3,
                                    ),
                                ),
                                Literal(
                                    String(
                                        "three",
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
                        In(
                            Literal(
                                String(
                                    "a",
                                ),
                            ),
                            Identifier(
                                "my_dict",
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
                        In(
                            Literal(
                                String(
                                    "b",
                                ),
                            ),
                            Identifier(
                                "my_dict",
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
                        In(
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                            Identifier(
                                "my_dict",
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
                            In(
                                Literal(
                                    String(
                                        "z",
                                    ),
                                ),
                                Identifier(
                                    "my_dict",
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