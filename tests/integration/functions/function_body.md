# Program
Status: 🟢
Assertions: 2

```rustleaf
var z = 0;
fn add(x, y) {
    z += 1;
    x + y
}
assert(add(2, 3) == 5);
assert(z == 1);
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
parse_program: parsing statement at position 5 (Fn)
parse_statement: starting at position 5 (Fn)
consume_token: position 5 consumed Fn
consume_token: position 6 consumed Ident
consume_token: position 7 consumed LeftParen
consume_token: position 8 consumed Ident
consume_token: position 9 consumed Comma
consume_token: position 10 consumed Ident
consume_token: position 11 consumed RightParen
consume_token: position 12 consumed LeftBrace
parse_statement: starting at position 13 (Ident(z))
consume_token: position 13 consumed Ident
consume_token: position 14 consumed PlusEqual
parse_expression: starting at position 15 (Int(1))
consume_token: position 15 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 16 consumed Semicolon
parse_statement: success - parsed assignment
parse_statement: starting at position 17 (Ident(x))
consume_token: position 17 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 17 (Ident(x))
consume_token: position 17 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 18 consumed Plus
consume_token: position 19 consumed Ident
parse_primary: success - parsed identifier (y)
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 20
parse_expression: starting at position 17 (Ident(x))
consume_token: position 17 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 18 consumed Plus
consume_token: position 19 consumed Ident
parse_primary: success - parsed identifier (y)
parse_expression: success - parsed precedence expression
consume_token: position 20 consumed RightBrace
parse_statement: success - parsed function declaration
parse_program: parsing statement at position 21 (Ident(assert))
parse_statement: starting at position 21 (Ident(assert))
consume_token: position 21 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 21 (Ident(assert))
consume_token: position 21 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 22 consumed LeftParen
parse_expression: starting at position 23 (Ident(add))
consume_token: position 23 consumed Ident
parse_primary: success - parsed identifier (add)
consume_token: position 24 consumed LeftParen
parse_expression: starting at position 25 (Int(2))
consume_token: position 25 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 26 consumed Comma
parse_expression: starting at position 27 (Int(3))
consume_token: position 27 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 28 consumed RightParen
consume_token: position 29 consumed EqualEqual
consume_token: position 30 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 31 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 32 consumed Semicolon
parse_program: parsing statement at position 33 (Ident(assert))
parse_statement: starting at position 33 (Ident(assert))
consume_token: position 33 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 33 (Ident(assert))
consume_token: position 33 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 34 consumed LeftParen
parse_expression: starting at position 35 (Ident(z))
consume_token: position 35 consumed Ident
parse_primary: success - parsed identifier (z)
consume_token: position 36 consumed EqualEqual
consume_token: position 37 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 38 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 39 consumed Semicolon
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
        1: Token(Ident, "z"),
        2: Token(Equal),
        3: Token(Int, "0"),
        4: Token(Semicolon),
        5: Token(Fn),
        6: Token(Ident, "add"),
        7: Token(LeftParen),
        8: Token(Ident, "x"),
        9: Token(Comma),
        10: Token(Ident, "y"),
        11: Token(RightParen),
        12: Token(LeftBrace),
        13: Token(Ident, "z"),
        14: Token(PlusEqual),
        15: Token(Int, "1"),
        16: Token(Semicolon),
        17: Token(Ident, "x"),
        18: Token(Plus),
        19: Token(Ident, "y"),
        20: Token(RightBrace),
        21: Token(Ident, "assert"),
        22: Token(LeftParen),
        23: Token(Ident, "add"),
        24: Token(LeftParen),
        25: Token(Int, "2"),
        26: Token(Comma),
        27: Token(Int, "3"),
        28: Token(RightParen),
        29: Token(EqualEqual),
        30: Token(Int, "5"),
        31: Token(RightParen),
        32: Token(Semicolon),
        33: Token(Ident, "assert"),
        34: Token(LeftParen),
        35: Token(Ident, "z"),
        36: Token(EqualEqual),
        37: Token(Int, "1"),
        38: Token(RightParen),
        39: Token(Semicolon),
        40: Token(Eof)
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
                    "z",
                ),
                value: Some(
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
            },
            FnDecl {
                name: "add",
                params: [
                    Parameter {
                        name: "x",
                        default: None,
                        kind: Regular,
                    },
                    Parameter {
                        name: "y",
                        default: None,
                        kind: Regular,
                    },
                ],
                body: Block {
                    statements: [
                        Assignment {
                            target: Identifier(
                                "z",
                            ),
                            op: AddAssign,
                            value: Literal(
                                Int(
                                    1,
                                ),
                            ),
                        },
                    ],
                    final_expr: Some(
                        Add(
                            Identifier(
                                "x",
                            ),
                            Identifier(
                                "y",
                            ),
                        ),
                    ),
                },
                is_pub: false,
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            FunctionCall(
                                Identifier(
                                    "add",
                                ),
                                [
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
                                ],
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
                        Eq(
                            Identifier(
                                "z",
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