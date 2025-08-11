# Program
Status: 🟢
Assertions: 1

```rustleaf
var name = "World";
var result = "Hello ${name}";
assert(result == "Hello World");
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
consume_token: position 0 consumed Var
consume_token: position 1 consumed Ident
consume_token: position 2 consumed Equal
parse_expression: starting at position 3 (String(World))
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
parse_expression: starting at position 8 (StringPart(Hello ))
parse_primary: success - parsing interpolated string
consume_token: position 8 consumed StringPart
consume_token: position 9 consumed InterpolationStart
parse_expression: starting at position 10 (Ident(name))
consume_token: position 10 consumed Ident
parse_primary: success - parsed identifier (name)
parse_expression: success - parsed precedence expression
consume_token: position 11 consumed InterpolationEnd
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
parse_expression: starting at position 15 (Ident(result))
consume_token: position 15 consumed Ident
parse_primary: success - parsed identifier (result)
consume_token: position 16 consumed EqualEqual
consume_token: position 17 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 18 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 19 consumed Semicolon
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
        1: Token(Ident, "name"),
        2: Token(Equal),
        3: Token(String, "World"),
        4: Token(Semicolon),
        5: Token(Var),
        6: Token(Ident, "result"),
        7: Token(Equal),
        8: Token(StringPart, "Hello "),
        9: Token(InterpolationStart),
        10: Token(Ident, "name"),
        11: Token(InterpolationEnd),
        12: Token(Semicolon),
        13: Token(Ident, "assert"),
        14: Token(LeftParen),
        15: Token(Ident, "result"),
        16: Token(EqualEqual),
        17: Token(String, "Hello World"),
        18: Token(RightParen),
        19: Token(Semicolon),
        20: Token(Eof)
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
                    "name",
                ),
                value: Some(
                    Literal(
                        String(
                            "World",
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result",
                ),
                value: Some(
                    InterpolatedString(
                        [
                            Text(
                                "Hello ",
                            ),
                            Expression(
                                Identifier(
                                    "name",
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
                                    "Hello World",
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