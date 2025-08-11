# Program
Status: 🟢
Assertions: 2

```rustleaf
// Test while loop as expression with proper variable initialization
var x = 0;
var result = while x < 5 {
    x = x + 1;
};
assert(x == 5);
assert(is_unit(result));
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
consume_token: position 0 consumed Var
consume_token: position 1 consumed Ident
consume_token: position 2 consumed Equal
parse_expression: starting at position 3 (Int(0))
consume_token: position 3 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 4 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 5 (Var)
parse_statement: starting at position 5 (Var)
consume_token: position 5 consumed Var
consume_token: position 6 consumed Ident
consume_token: position 7 consumed Equal
parse_expression: starting at position 8 (While)
consume_token: position 8 consumed While
parse_primary: success - parsing while expression
parse_expression: starting at position 9 (Ident(x))
consume_token: position 9 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 10 consumed Less
consume_token: position 11 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 12 consumed LeftBrace
parse_statement: starting at position 13 (Ident(x))
consume_token: position 13 consumed Ident
consume_token: position 14 consumed Equal
parse_expression: starting at position 15 (Ident(x))
consume_token: position 15 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 16 consumed Plus
consume_token: position 17 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 18 consumed Semicolon
parse_statement: success - parsed assignment
consume_token: position 19 consumed RightBrace
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
parse_expression: starting at position 23 (Ident(x))
consume_token: position 23 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 24 consumed EqualEqual
consume_token: position 25 consumed Int
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
parse_expression: starting at position 30 (Ident(is_unit))
consume_token: position 30 consumed Ident
parse_primary: success - parsed identifier (is_unit)
consume_token: position 31 consumed LeftParen
parse_expression: starting at position 32 (Ident(result))
consume_token: position 32 consumed Ident
parse_primary: success - parsed identifier (result)
parse_expression: success - parsed precedence expression
consume_token: position 33 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 34 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 35 consumed Semicolon
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
        1: Token(Ident, "x"),
        2: Token(Equal),
        3: Token(Int, "0"),
        4: Token(Semicolon),
        5: Token(Var),
        6: Token(Ident, "result"),
        7: Token(Equal),
        8: Token(While),
        9: Token(Ident, "x"),
        10: Token(Less),
        11: Token(Int, "5"),
        12: Token(LeftBrace),
        13: Token(Ident, "x"),
        14: Token(Equal),
        15: Token(Ident, "x"),
        16: Token(Plus),
        17: Token(Int, "1"),
        18: Token(Semicolon),
        19: Token(RightBrace),
        20: Token(Semicolon),
        21: Token(Ident, "assert"),
        22: Token(LeftParen),
        23: Token(Ident, "x"),
        24: Token(EqualEqual),
        25: Token(Int, "5"),
        26: Token(RightParen),
        27: Token(Semicolon),
        28: Token(Ident, "assert"),
        29: Token(LeftParen),
        30: Token(Ident, "is_unit"),
        31: Token(LeftParen),
        32: Token(Ident, "result"),
        33: Token(RightParen),
        34: Token(RightParen),
        35: Token(Semicolon),
        36: Token(Eof)
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
                    "x",
                ),
                value: Some(
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result",
                ),
                value: Some(
                    While {
                        condition: Lt(
                            Identifier(
                                "x",
                            ),
                            Literal(
                                Int(
                                    5,
                                ),
                            ),
                        ),
                        body: Block {
                            statements: [
                                Assignment {
                                    target: Identifier(
                                        "x",
                                    ),
                                    op: Assign,
                                    value: Add(
                                        Identifier(
                                            "x",
                                        ),
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
                    },
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
                                "x",
                            ),
                            Literal(
                                Int(
                                    5,
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
                        FunctionCall(
                            Identifier(
                                "is_unit",
                            ),
                            [
                                Identifier(
                                    "result",
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