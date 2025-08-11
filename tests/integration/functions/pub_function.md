# Program
Status: 🟢
Assertions: 2

```rustleaf
pub fn greet(name) {
    return "Hello, " + name;
}

var result = greet("World");
assert(result == "Hello, World");
assert(greet("Alice") == "Hello, Alice");
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Pub)
parse_statement: starting at position 0 (Pub)
consume_token: position 0 consumed Pub
consume_token: position 1 consumed Fn
consume_token: position 2 consumed Ident
consume_token: position 3 consumed LeftParen
consume_token: position 4 consumed Ident
consume_token: position 5 consumed RightParen
consume_token: position 6 consumed LeftBrace
parse_statement: starting at position 7 (Return)
consume_token: position 7 consumed Return
parse_expression: starting at position 8 (String(Hello, ))
consume_token: position 8 consumed String
parse_primary: success - parsed numeric/string literal
consume_token: position 9 consumed Plus
consume_token: position 10 consumed Ident
parse_primary: success - parsed identifier (name)
parse_expression: success - parsed precedence expression
consume_token: position 11 consumed Semicolon
parse_statement: success - parsed return statement
consume_token: position 12 consumed RightBrace
parse_statement: success - parsed function declaration
parse_program: parsing statement at position 13 (Var)
parse_statement: starting at position 13 (Var)
consume_token: position 13 consumed Var
consume_token: position 14 consumed Ident
consume_token: position 15 consumed Equal
parse_expression: starting at position 16 (Ident(greet))
consume_token: position 16 consumed Ident
parse_primary: success - parsed identifier (greet)
consume_token: position 17 consumed LeftParen
parse_expression: starting at position 18 (String(World))
consume_token: position 18 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 19 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 20 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 21 (Ident(assert))
parse_statement: starting at position 21 (Ident(assert))
consume_token: position 21 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 21 (Ident(assert))
consume_token: position 21 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 22 consumed LeftParen
parse_expression: starting at position 23 (Ident(result))
consume_token: position 23 consumed Ident
parse_primary: success - parsed identifier (result)
consume_token: position 24 consumed EqualEqual
consume_token: position 25 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 26 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 27 consumed Semicolon
parse_program: parsing statement at position 28 (Ident(assert))
parse_statement: starting at position 28 (Ident(assert))
consume_token: position 28 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 28 (Ident(assert))
consume_token: position 28 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 29 consumed LeftParen
parse_expression: starting at position 30 (Ident(greet))
consume_token: position 30 consumed Ident
parse_primary: success - parsed identifier (greet)
consume_token: position 31 consumed LeftParen
parse_expression: starting at position 32 (String(Alice))
consume_token: position 32 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 33 consumed RightParen
consume_token: position 34 consumed EqualEqual
consume_token: position 35 consumed String
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
        0: Token(Pub),
        1: Token(Fn),
        2: Token(Ident, "greet"),
        3: Token(LeftParen),
        4: Token(Ident, "name"),
        5: Token(RightParen),
        6: Token(LeftBrace),
        7: Token(Return),
        8: Token(String, "Hello, "),
        9: Token(Plus),
        10: Token(Ident, "name"),
        11: Token(Semicolon),
        12: Token(RightBrace),
        13: Token(Var),
        14: Token(Ident, "result"),
        15: Token(Equal),
        16: Token(Ident, "greet"),
        17: Token(LeftParen),
        18: Token(String, "World"),
        19: Token(RightParen),
        20: Token(Semicolon),
        21: Token(Ident, "assert"),
        22: Token(LeftParen),
        23: Token(Ident, "result"),
        24: Token(EqualEqual),
        25: Token(String, "Hello, World"),
        26: Token(RightParen),
        27: Token(Semicolon),
        28: Token(Ident, "assert"),
        29: Token(LeftParen),
        30: Token(Ident, "greet"),
        31: Token(LeftParen),
        32: Token(String, "Alice"),
        33: Token(RightParen),
        34: Token(EqualEqual),
        35: Token(String, "Hello, Alice"),
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
            FnDecl {
                name: "greet",
                params: [
                    Parameter {
                        name: "name",
                        default: None,
                        kind: Regular,
                    },
                ],
                body: Block {
                    statements: [
                        Return(
                            Some(
                                Add(
                                    Literal(
                                        String(
                                            "Hello, ",
                                        ),
                                    ),
                                    Identifier(
                                        "name",
                                    ),
                                ),
                            ),
                        ),
                    ],
                    final_expr: None,
                },
                is_pub: true,
            },
            VarDecl {
                pattern: Variable(
                    "result",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "greet",
                        ),
                        [
                            Literal(
                                String(
                                    "World",
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
                                "result",
                            ),
                            Literal(
                                String(
                                    "Hello, World",
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
                            FunctionCall(
                                Identifier(
                                    "greet",
                                ),
                                [
                                    Literal(
                                        String(
                                            "Alice",
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                String(
                                    "Hello, Alice",
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