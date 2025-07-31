# Program
Status: 🟢
Assertions: 1

```rustleaf
use math::adder;
var x = adder(1, 2);
assert(x == 3);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Use)
parse_statement: starting at position 0 (Use)
consume_token: position 0 consumed Use
consume_token: position 1 consumed Ident
consume_token: position 2 consumed DoubleColon
consume_token: position 3 consumed Ident
consume_token: position 4 consumed Semicolon
parse_statement: success - parsed import statement
parse_program: parsing statement at position 5 (Var)
parse_statement: starting at position 5 (Var)
consume_token: position 5 consumed Var
consume_token: position 6 consumed Ident
consume_token: position 7 consumed Equal
parse_expression: starting at position 8 (Ident(adder))
consume_token: position 8 consumed Ident
parse_primary: success - parsed identifier (adder)
consume_token: position 9 consumed LeftParen
parse_expression: starting at position 10 (Int(1))
consume_token: position 10 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 11 consumed Comma
parse_expression: starting at position 12 (Int(2))
consume_token: position 12 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 13 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 14 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 15 (Ident(assert))
parse_statement: starting at position 15 (Ident(assert))
consume_token: position 15 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 15 (Ident(assert))
consume_token: position 15 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 16 consumed LeftParen
parse_expression: starting at position 17 (Ident(x))
consume_token: position 17 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 18 consumed EqualEqual
consume_token: position 19 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 20 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 21 consumed Semicolon
parse_program: parsed 3 statements
parse_program: starting
parse_program: parsing statement at position 0 (Pub)
parse_statement: starting at position 0 (Pub)
consume_token: position 0 consumed Pub
consume_token: position 1 consumed Fn
consume_token: position 2 consumed Ident
consume_token: position 3 consumed LeftParen
consume_token: position 4 consumed Ident
consume_token: position 5 consumed Comma
consume_token: position 6 consumed Ident
consume_token: position 7 consumed RightParen
consume_token: position 8 consumed LeftBrace
parse_statement: starting at position 9 (Ident(x))
consume_token: position 9 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 9 (Ident(x))
consume_token: position 9 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 10 consumed Plus
consume_token: position 11 consumed Ident
parse_primary: success - parsed identifier (y)
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 12
parse_expression: starting at position 9 (Ident(x))
consume_token: position 9 consumed Ident
parse_primary: success - parsed identifier (x)
consume_token: position 10 consumed Plus
consume_token: position 11 consumed Ident
parse_primary: success - parsed identifier (y)
parse_expression: success - parsed precedence expression
consume_token: position 12 consumed RightBrace
parse_statement: success - parsed function declaration
parse_program: parsed 1 statements
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
        0: Token(Use),
        1: Token(Ident, "math"),
        2: Token(DoubleColon),
        3: Token(Ident, "adder"),
        4: Token(Semicolon),
        5: Token(Var),
        6: Token(Ident, "x"),
        7: Token(Equal),
        8: Token(Ident, "adder"),
        9: Token(LeftParen),
        10: Token(Int, "1"),
        11: Token(Comma),
        12: Token(Int, "2"),
        13: Token(RightParen),
        14: Token(Semicolon),
        15: Token(Ident, "assert"),
        16: Token(LeftParen),
        17: Token(Ident, "x"),
        18: Token(EqualEqual),
        19: Token(Int, "3"),
        20: Token(RightParen),
        21: Token(Semicolon),
        22: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            Import(
                ImportSpec {
                    module: "math",
                    items: Specific(
                        [
                            ImportItem {
                                name: "adder",
                                alias: None,
                            },
                        ],
                    ),
                },
            ),
            VarDecl {
                pattern: Variable(
                    "x",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "adder",
                        ),
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
                                "x",
                            ),
                            Literal(
                                Int(
                                    3,
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
    RustValue(
        EvalProgram {
            statements: [
                RustValue(
                    EvalImport {
                        data: ImportData {
                            module: "math",
                            items: Specific(
                                [
                                    ImportItem {
                                        name: "adder",
                                        alias: None,
                                    },
                                ],
                            ),
                        },
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "x",
                        init_expr: Some(
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalVariable {
                                            name: "adder",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    1,
                                                ),
                                            },
                                        ),
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    2,
                                                ),
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalCall {
                        func_expr: RustValue(
                            EvalVariable {
                                name: "assert",
                            },
                        ),
                        args: [
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalGetAttr {
                                            obj_expr: RustValue(
                                                EvalVariable {
                                                    name: "x",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    3,
                                                ),
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ],
                    },
                ),
            ],
        },
    ),
)
```