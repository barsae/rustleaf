# Program
Status: 🟢
Assertions: 1

```rustleaf
var x = 0;
loop {
    x += 1;
    break;
    x += 1;
}
assert(x == 1);
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
parse_program: parsing statement at position 5 (Loop)
parse_statement: starting at position 5 (Loop)
parse_expression: starting at position 5 (Loop)
consume_token: position 5 consumed Loop
parse_primary: success - parsing loop expression
consume_token: position 6 consumed LeftBrace
parse_statement: starting at position 7 (Ident(x))
consume_token: position 7 consumed Ident
consume_token: position 8 consumed PlusEqual
parse_expression: starting at position 9 (Int(1))
consume_token: position 9 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 10 consumed Semicolon
parse_statement: success - parsed assignment
parse_statement: starting at position 11 (Break)
consume_token: position 11 consumed Break
consume_token: position 12 consumed Semicolon
parse_statement: success - parsed break statement
parse_statement: starting at position 13 (Ident(x))
consume_token: position 13 consumed Ident
consume_token: position 14 consumed PlusEqual
parse_expression: starting at position 15 (Int(1))
consume_token: position 15 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 16 consumed Semicolon
parse_statement: success - parsed assignment
consume_token: position 17 consumed RightBrace
parse_expression: success - parsed precedence expression
parse_statement: success - parsed block-like expression statement
parse_program: parsing statement at position 18 (Ident(assert))
parse_statement: starting at position 18 (Ident(assert))
consume_token: position 18 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 18 (Ident(assert))
consume_token: position 18 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 19 consumed LeftParen
parse_expression: starting at position 20 (Ident(x))
consume_token: position 20 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 21 consumed EqualEqual
consume_token: position 22 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 23 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 24 consumed Semicolon
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
        1: Token(Ident, "x"),
        2: Token(Equal),
        3: Token(Int, "0"),
        4: Token(Semicolon),
        5: Token(Loop),
        6: Token(LeftBrace),
        7: Token(Ident, "x"),
        8: Token(PlusEqual),
        9: Token(Int, "1"),
        10: Token(Semicolon),
        11: Token(Break),
        12: Token(Semicolon),
        13: Token(Ident, "x"),
        14: Token(PlusEqual),
        15: Token(Int, "1"),
        16: Token(Semicolon),
        17: Token(RightBrace),
        18: Token(Ident, "assert"),
        19: Token(LeftParen),
        20: Token(Ident, "x"),
        21: Token(EqualEqual),
        22: Token(Int, "1"),
        23: Token(RightParen),
        24: Token(Semicolon),
        25: Token(Eof)
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
            Expression(
                Loop {
                    body: Block {
                        statements: [
                            Assignment {
                                target: Identifier(
                                    "x",
                                ),
                                op: AddAssign,
                                value: Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                            },
                            Break(
                                None,
                            ),
                            Assignment {
                                target: Identifier(
                                    "x",
                                ),
                                op: AddAssign,
                                value: Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                            },
                        ],
                        final_expr: None,
                    },
                },
            ),
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
                                    1,
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