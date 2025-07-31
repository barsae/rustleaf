# Program
Status: 🟢
Assertions: 4

```rustleaf
fn hello() { 42 }
fn get_pi() { 3.14 }
fn get_greeting() { "Hello, World!" }

var result1 = hello();
var result2 = get_pi();
var result3 = get_greeting();

assert(result1 == 42);
assert(result2 == 3.14);
assert(result3 == "Hello, World!");
assert(hello() == 42);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Fn)
parse_statement: starting at position 0 (Fn)
consume_token: position 0 consumed Fn
consume_token: position 1 consumed Ident
consume_token: position 2 consumed LeftParen
consume_token: position 3 consumed RightParen
consume_token: position 4 consumed LeftBrace
parse_statement: starting at position 5 (Int(42))
parse_statement: falling back to expression statement
parse_expression: starting at position 5 (Int(42))
consume_token: position 5 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 6
parse_expression: starting at position 5 (Int(42))
consume_token: position 5 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 6 consumed RightBrace
parse_statement: success - parsed function declaration
parse_program: parsing statement at position 7 (Fn)
parse_statement: starting at position 7 (Fn)
consume_token: position 7 consumed Fn
consume_token: position 8 consumed Ident
consume_token: position 9 consumed LeftParen
consume_token: position 10 consumed RightParen
consume_token: position 11 consumed LeftBrace
parse_statement: starting at position 12 (Float(3.14))
parse_statement: falling back to expression statement
parse_expression: starting at position 12 (Float(3.14))
consume_token: position 12 consumed Float
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 13
parse_expression: starting at position 12 (Float(3.14))
consume_token: position 12 consumed Float
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 13 consumed RightBrace
parse_statement: success - parsed function declaration
parse_program: parsing statement at position 14 (Fn)
parse_statement: starting at position 14 (Fn)
consume_token: position 14 consumed Fn
consume_token: position 15 consumed Ident
consume_token: position 16 consumed LeftParen
consume_token: position 17 consumed RightParen
consume_token: position 18 consumed LeftBrace
parse_statement: starting at position 19 (String(Hello, World!))
parse_statement: falling back to expression statement
parse_expression: starting at position 19 (String(Hello, World!))
consume_token: position 19 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 20
parse_expression: starting at position 19 (String(Hello, World!))
consume_token: position 19 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 20 consumed RightBrace
parse_statement: success - parsed function declaration
parse_program: parsing statement at position 21 (Var)
parse_statement: starting at position 21 (Var)
consume_token: position 21 consumed Var
consume_token: position 22 consumed Ident
consume_token: position 23 consumed Equal
parse_expression: starting at position 24 (Ident(hello))
consume_token: position 24 consumed Ident
parse_primary: success - parsed identifier (hello)
consume_token: position 25 consumed LeftParen
consume_token: position 26 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 27 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 28 (Var)
parse_statement: starting at position 28 (Var)
consume_token: position 28 consumed Var
consume_token: position 29 consumed Ident
consume_token: position 30 consumed Equal
parse_expression: starting at position 31 (Ident(get_pi))
consume_token: position 31 consumed Ident
parse_primary: success - parsed identifier (get_pi)
consume_token: position 32 consumed LeftParen
consume_token: position 33 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 34 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 35 (Var)
parse_statement: starting at position 35 (Var)
consume_token: position 35 consumed Var
consume_token: position 36 consumed Ident
consume_token: position 37 consumed Equal
parse_expression: starting at position 38 (Ident(get_greeting))
consume_token: position 38 consumed Ident
parse_primary: success - parsed identifier (get_greeting)
consume_token: position 39 consumed LeftParen
consume_token: position 40 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 41 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 42 (Ident(assert))
parse_statement: starting at position 42 (Ident(assert))
consume_token: position 42 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 42 (Ident(assert))
consume_token: position 42 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 43 consumed LeftParen
parse_expression: starting at position 44 (Ident(result1))
consume_token: position 44 consumed Ident
parse_primary: success - parsed identifier (result1)
consume_token: position 45 consumed EqualEqual
consume_token: position 46 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 47 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 48 consumed Semicolon
parse_program: parsing statement at position 49 (Ident(assert))
parse_statement: starting at position 49 (Ident(assert))
consume_token: position 49 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 49 (Ident(assert))
consume_token: position 49 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 50 consumed LeftParen
parse_expression: starting at position 51 (Ident(result2))
consume_token: position 51 consumed Ident
parse_primary: success - parsed identifier (result2)
consume_token: position 52 consumed EqualEqual
consume_token: position 53 consumed Float
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 54 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 55 consumed Semicolon
parse_program: parsing statement at position 56 (Ident(assert))
parse_statement: starting at position 56 (Ident(assert))
consume_token: position 56 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 56 (Ident(assert))
consume_token: position 56 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 57 consumed LeftParen
parse_expression: starting at position 58 (Ident(result3))
consume_token: position 58 consumed Ident
parse_primary: success - parsed identifier (result3)
consume_token: position 59 consumed EqualEqual
consume_token: position 60 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 61 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 62 consumed Semicolon
parse_program: parsing statement at position 63 (Ident(assert))
parse_statement: starting at position 63 (Ident(assert))
consume_token: position 63 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 63 (Ident(assert))
consume_token: position 63 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 64 consumed LeftParen
parse_expression: starting at position 65 (Ident(hello))
consume_token: position 65 consumed Ident
parse_primary: success - parsed identifier (hello)
consume_token: position 66 consumed LeftParen
consume_token: position 67 consumed RightParen
consume_token: position 68 consumed EqualEqual
consume_token: position 69 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 70 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 71 consumed Semicolon
parse_program: parsed 10 statements
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
        0: Token(Fn),
        1: Token(Ident, "hello"),
        2: Token(LeftParen),
        3: Token(RightParen),
        4: Token(LeftBrace),
        5: Token(Int, "42"),
        6: Token(RightBrace),
        7: Token(Fn),
        8: Token(Ident, "get_pi"),
        9: Token(LeftParen),
        10: Token(RightParen),
        11: Token(LeftBrace),
        12: Token(Float, "3.14"),
        13: Token(RightBrace),
        14: Token(Fn),
        15: Token(Ident, "get_greeting"),
        16: Token(LeftParen),
        17: Token(RightParen),
        18: Token(LeftBrace),
        19: Token(String, "Hello, World!"),
        20: Token(RightBrace),
        21: Token(Var),
        22: Token(Ident, "result1"),
        23: Token(Equal),
        24: Token(Ident, "hello"),
        25: Token(LeftParen),
        26: Token(RightParen),
        27: Token(Semicolon),
        28: Token(Var),
        29: Token(Ident, "result2"),
        30: Token(Equal),
        31: Token(Ident, "get_pi"),
        32: Token(LeftParen),
        33: Token(RightParen),
        34: Token(Semicolon),
        35: Token(Var),
        36: Token(Ident, "result3"),
        37: Token(Equal),
        38: Token(Ident, "get_greeting"),
        39: Token(LeftParen),
        40: Token(RightParen),
        41: Token(Semicolon),
        42: Token(Ident, "assert"),
        43: Token(LeftParen),
        44: Token(Ident, "result1"),
        45: Token(EqualEqual),
        46: Token(Int, "42"),
        47: Token(RightParen),
        48: Token(Semicolon),
        49: Token(Ident, "assert"),
        50: Token(LeftParen),
        51: Token(Ident, "result2"),
        52: Token(EqualEqual),
        53: Token(Float, "3.14"),
        54: Token(RightParen),
        55: Token(Semicolon),
        56: Token(Ident, "assert"),
        57: Token(LeftParen),
        58: Token(Ident, "result3"),
        59: Token(EqualEqual),
        60: Token(String, "Hello, World!"),
        61: Token(RightParen),
        62: Token(Semicolon),
        63: Token(Ident, "assert"),
        64: Token(LeftParen),
        65: Token(Ident, "hello"),
        66: Token(LeftParen),
        67: Token(RightParen),
        68: Token(EqualEqual),
        69: Token(Int, "42"),
        70: Token(RightParen),
        71: Token(Semicolon),
        72: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            FnDecl {
                name: "hello",
                params: [],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Literal(
                            Int(
                                42,
                            ),
                        ),
                    ),
                },
                is_pub: false,
            },
            FnDecl {
                name: "get_pi",
                params: [],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Literal(
                            Float(
                                3.14,
                            ),
                        ),
                    ),
                },
                is_pub: false,
            },
            FnDecl {
                name: "get_greeting",
                params: [],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Literal(
                            String(
                                "Hello, World!",
                            ),
                        ),
                    ),
                },
                is_pub: false,
            },
            VarDecl {
                pattern: Variable(
                    "result1",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "hello",
                        ),
                        [],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result2",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "get_pi",
                        ),
                        [],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result3",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "get_greeting",
                        ),
                        [],
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
                                "result1",
                            ),
                            Literal(
                                Int(
                                    42,
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
                                "result2",
                            ),
                            Literal(
                                Float(
                                    3.14,
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
                                "result3",
                            ),
                            Literal(
                                String(
                                    "Hello, World!",
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
                                    "hello",
                                ),
                                [],
                            ),
                            Literal(
                                Int(
                                    42,
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
                    EvalFunction {
                        data: FunctionData {
                            name: "hello",
                            params: [],
                            body: RustValue(
                                EvalBlock {
                                    statements: [],
                                    final_expr: Some(
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    42,
                                                ),
                                            },
                                        ),
                                    ),
                                },
                            ),
                        },
                    },
                ),
                RustValue(
                    EvalFunction {
                        data: FunctionData {
                            name: "get_pi",
                            params: [],
                            body: RustValue(
                                EvalBlock {
                                    statements: [],
                                    final_expr: Some(
                                        RustValue(
                                            EvalLiteral {
                                                value: Float(
                                                    3.14,
                                                ),
                                            },
                                        ),
                                    ),
                                },
                            ),
                        },
                    },
                ),
                RustValue(
                    EvalFunction {
                        data: FunctionData {
                            name: "get_greeting",
                            params: [],
                            body: RustValue(
                                EvalBlock {
                                    statements: [],
                                    final_expr: Some(
                                        RustValue(
                                            EvalLiteral {
                                                value: String(
                                                    "Hello, World!",
                                                ),
                                            },
                                        ),
                                    ),
                                },
                            ),
                        },
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "result1",
                        init_expr: Some(
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalVariable {
                                            name: "hello",
                                        },
                                    ),
                                    args: [],
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "result2",
                        init_expr: Some(
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalVariable {
                                            name: "get_pi",
                                        },
                                    ),
                                    args: [],
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "result3",
                        init_expr: Some(
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalVariable {
                                            name: "get_greeting",
                                        },
                                    ),
                                    args: [],
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
                                                    name: "result1",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    42,
                                                ),
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ],
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
                                                    name: "result2",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Float(
                                                    3.14,
                                                ),
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ],
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
                                                    name: "result3",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: String(
                                                    "Hello, World!",
                                                ),
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ],
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
                                                EvalCall {
                                                    func_expr: RustValue(
                                                        EvalVariable {
                                                            name: "hello",
                                                        },
                                                    ),
                                                    args: [],
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    42,
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