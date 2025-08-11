# Program
Status: 🟢
Assertions: 2

```rustleaf
var user = {"name": "Alice", "age": 30};
var {name, age: user_age} = user;
assert(name == "Alice");
assert(user_age == 30);
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
parse_expression: starting at position 6 (String(Alice))
consume_token: position 6 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 7 consumed Comma
consume_token: position 8 consumed String
parse_primary: success - parsed numeric/string literal
consume_token: position 9 consumed Colon
parse_expression: starting at position 10 (Int(30))
consume_token: position 10 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 11 consumed RightBrace
parse_expression: success - parsed precedence expression
consume_token: position 12 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 13 (Var)
parse_statement: starting at position 13 (Var)
consume_token: position 13 consumed Var
consume_token: position 14 consumed LeftBrace
consume_token: position 15 consumed Ident
consume_token: position 16 consumed Comma
consume_token: position 17 consumed Ident
consume_token: position 18 consumed Colon
consume_token: position 19 consumed Ident
consume_token: position 20 consumed RightBrace
consume_token: position 21 consumed Equal
parse_expression: starting at position 22 (Ident(user))
consume_token: position 22 consumed Ident
parse_primary: success - parsed identifier (user)
parse_expression: success - parsed precedence expression
consume_token: position 23 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 24 (Ident(assert))
parse_statement: starting at position 24 (Ident(assert))
consume_token: position 24 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 24 (Ident(assert))
consume_token: position 24 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 25 consumed LeftParen
parse_expression: starting at position 26 (Ident(name))
consume_token: position 26 consumed Ident
parse_primary: success - parsed identifier (name)
consume_token: position 27 consumed EqualEqual
consume_token: position 28 consumed String
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
parse_expression: starting at position 33 (Ident(user_age))
consume_token: position 33 consumed Ident
parse_primary: success - parsed identifier (user_age)
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
        1: Token(Ident, "user"),
        2: Token(Equal),
        3: Token(LeftBrace),
        4: Token(String, "name"),
        5: Token(Colon),
        6: Token(String, "Alice"),
        7: Token(Comma),
        8: Token(String, "age"),
        9: Token(Colon),
        10: Token(Int, "30"),
        11: Token(RightBrace),
        12: Token(Semicolon),
        13: Token(Var),
        14: Token(LeftBrace),
        15: Token(Ident, "name"),
        16: Token(Comma),
        17: Token(Ident, "age"),
        18: Token(Colon),
        19: Token(Ident, "user_age"),
        20: Token(RightBrace),
        21: Token(Equal),
        22: Token(Ident, "user"),
        23: Token(Semicolon),
        24: Token(Ident, "assert"),
        25: Token(LeftParen),
        26: Token(Ident, "name"),
        27: Token(EqualEqual),
        28: Token(String, "Alice"),
        29: Token(RightParen),
        30: Token(Semicolon),
        31: Token(Ident, "assert"),
        32: Token(LeftParen),
        33: Token(Ident, "user_age"),
        34: Token(EqualEqual),
        35: Token(Int, "30"),
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
                pattern: Variable(
                    "user",
                ),
                value: Some(
                    Dict(
                        [
                            (
                                Literal(
                                    String(
                                        "name",
                                    ),
                                ),
                                Literal(
                                    String(
                                        "Alice",
                                    ),
                                ),
                            ),
                            (
                                Literal(
                                    String(
                                        "age",
                                    ),
                                ),
                                Literal(
                                    Int(
                                        30,
                                    ),
                                ),
                            ),
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Dict(
                    [
                        DictPattern {
                            key: "name",
                            alias: None,
                        },
                        DictPattern {
                            key: "age",
                            alias: Some(
                                "user_age",
                            ),
                        },
                    ],
                ),
                value: Some(
                    Identifier(
                        "user",
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
                                "name",
                            ),
                            Literal(
                                String(
                                    "Alice",
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
                                "user_age",
                            ),
                            Literal(
                                Int(
                                    30,
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