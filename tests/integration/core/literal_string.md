# Program
Status: 🟢
Assertions: 3

```rustleaf
var greeting = "hello";
var empty = "";
assert(greeting == "hello");
assert(empty == "");
assert(greeting + " world" == "hello world");
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
consume_token: position 0 consumed Var
consume_token: position 1 consumed Ident
consume_token: position 2 consumed Equal
parse_expression: starting at position 3 (String(hello))
consume_token: position 3 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 4 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 5 (Var)
parse_statement: starting at position 5 (Var)
consume_token: position 5 consumed Var
consume_token: position 6 consumed Ident
consume_token: position 7 consumed Equal
parse_expression: starting at position 8 (String())
consume_token: position 8 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 9 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 10 (Ident(assert))
parse_statement: starting at position 10 (Ident(assert))
consume_token: position 10 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 10 (Ident(assert))
consume_token: position 10 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 11 consumed LeftParen
parse_expression: starting at position 12 (Ident(greeting))
consume_token: position 12 consumed Ident
parse_primary: success - parsed identifier (greeting)
consume_token: position 13 consumed EqualEqual
consume_token: position 14 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 15 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 16 consumed Semicolon
parse_program: parsing statement at position 17 (Ident(assert))
parse_statement: starting at position 17 (Ident(assert))
consume_token: position 17 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 17 (Ident(assert))
consume_token: position 17 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 18 consumed LeftParen
parse_expression: starting at position 19 (Ident(empty))
consume_token: position 19 consumed Ident
parse_primary: success - parsed identifier (empty)
consume_token: position 20 consumed EqualEqual
consume_token: position 21 consumed String
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
parse_expression: starting at position 26 (Ident(greeting))
consume_token: position 26 consumed Ident
parse_primary: success - parsed identifier (greeting)
consume_token: position 27 consumed Plus
consume_token: position 28 consumed String
parse_primary: success - parsed numeric/string literal
consume_token: position 29 consumed EqualEqual
consume_token: position 30 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 31 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 32 consumed Semicolon
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
        1: Token(Ident, "greeting"),
        2: Token(Equal),
        3: Token(String, "hello"),
        4: Token(Semicolon),
        5: Token(Var),
        6: Token(Ident, "empty"),
        7: Token(Equal),
        8: Token(String, ""),
        9: Token(Semicolon),
        10: Token(Ident, "assert"),
        11: Token(LeftParen),
        12: Token(Ident, "greeting"),
        13: Token(EqualEqual),
        14: Token(String, "hello"),
        15: Token(RightParen),
        16: Token(Semicolon),
        17: Token(Ident, "assert"),
        18: Token(LeftParen),
        19: Token(Ident, "empty"),
        20: Token(EqualEqual),
        21: Token(String, ""),
        22: Token(RightParen),
        23: Token(Semicolon),
        24: Token(Ident, "assert"),
        25: Token(LeftParen),
        26: Token(Ident, "greeting"),
        27: Token(Plus),
        28: Token(String, " world"),
        29: Token(EqualEqual),
        30: Token(String, "hello world"),
        31: Token(RightParen),
        32: Token(Semicolon),
        33: Token(Eof)
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
                    "greeting",
                ),
                value: Some(
                    Literal(
                        String(
                            "hello",
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "empty",
                ),
                value: Some(
                    Literal(
                        String(
                            "",
                        ),
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
                                "greeting",
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
                            Identifier(
                                "empty",
                            ),
                            Literal(
                                String(
                                    "",
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
                            Add(
                                Identifier(
                                    "greeting",
                                ),
                                Literal(
                                    String(
                                        " world",
                                    ),
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