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
parse_statement: falling back to expression statement
parse_expression: starting at position 0 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 2 (String(hello))
parse_primary: success - parsed numeric/string literal
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_program: parsing statement at position 7 (Ident(assert))
parse_statement: starting at position 7 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 7 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 9 (Not)
parse_primary: success - parsing parenthesized expression
parse_expression: starting at position 11 (String(xyz))
parse_primary: success - parsed numeric/string literal
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_program: parsing statement at position 17 (Var)
parse_statement: starting at position 17 (Var)
parse_expression: starting at position 20 (LeftBracket)
parse_primary: success - parsing list literal
parse_expression: starting at position 21 (Int(1))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: starting at position 23 (Int(2))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: starting at position 25 (Int(3))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: starting at position 27 (String(hello))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 30 (Ident(assert))
parse_statement: starting at position 30 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 30 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 32 (Int(2))
parse_primary: success - parsed numeric/string literal
parse_primary: success - parsed identifier (my_list)
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_program: parsing statement at position 37 (Ident(assert))
parse_statement: starting at position 37 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 37 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 39 (String(hello))
parse_primary: success - parsed numeric/string literal
parse_primary: success - parsed identifier (my_list)
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_program: parsing statement at position 44 (Ident(assert))
parse_statement: starting at position 44 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 44 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 46 (Not)
parse_primary: success - parsing parenthesized expression
parse_expression: starting at position 48 (Int(99))
parse_primary: success - parsed numeric/string literal
parse_primary: success - parsed identifier (my_list)
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_program: parsing statement at position 54 (Var)
parse_statement: starting at position 54 (Var)
parse_expression: starting at position 57 (LeftBrace)
parse_primary: success - parsing block or dict
parse_primary: success - parsed numeric/string literal
parse_expression: starting at position 60 (Int(1))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_primary: success - parsed numeric/string literal
parse_expression: starting at position 64 (Int(2))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_primary: success - parsed numeric/string literal
parse_expression: starting at position 68 (String(three))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 71 (Ident(assert))
parse_statement: starting at position 71 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 71 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 73 (String(a))
parse_primary: success - parsed numeric/string literal
parse_primary: success - parsed identifier (my_dict)
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_program: parsing statement at position 78 (Ident(assert))
parse_statement: starting at position 78 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 78 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 80 (String(b))
parse_primary: success - parsed numeric/string literal
parse_primary: success - parsed identifier (my_dict)
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_program: parsing statement at position 85 (Ident(assert))
parse_statement: starting at position 85 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 85 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 87 (Int(3))
parse_primary: success - parsed numeric/string literal
parse_primary: success - parsed identifier (my_dict)
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_program: parsing statement at position 92 (Ident(assert))
parse_statement: starting at position 92 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 92 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 94 (Not)
parse_primary: success - parsing parenthesized expression
parse_expression: starting at position 96 (String(z))
parse_primary: success - parsed numeric/string literal
parse_primary: success - parsed identifier (my_dict)
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
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
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(String, "hello"),
        Token(In),
        Token(String, "hello world"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(LeftParen),
        Token(String, "xyz"),
        Token(In),
        Token(String, "hello world"),
        Token(RightParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "my_list"),
        Token(Equal),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(Comma),
        Token(String, "hello"),
        Token(RightBracket),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "2"),
        Token(In),
        Token(Ident, "my_list"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(String, "hello"),
        Token(In),
        Token(Ident, "my_list"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(LeftParen),
        Token(Int, "99"),
        Token(In),
        Token(Ident, "my_list"),
        Token(RightParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "my_dict"),
        Token(Equal),
        Token(LeftBrace),
        Token(String, "a"),
        Token(Colon),
        Token(Int, "1"),
        Token(Comma),
        Token(String, "b"),
        Token(Colon),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(Colon),
        Token(String, "three"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(String, "a"),
        Token(In),
        Token(Ident, "my_dict"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(String, "b"),
        Token(In),
        Token(Ident, "my_dict"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "3"),
        Token(In),
        Token(Ident, "my_dict"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(LeftParen),
        Token(String, "z"),
        Token(In),
        Token(Ident, "my_dict"),
        Token(RightParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Eof),
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
    RustValue(
        EvalProgram {
            statements: [
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
                                                EvalLiteral {
                                                    value: String(
                                                        "hello world",
                                                    ),
                                                },
                                            ),
                                            attr_name: "op_contains",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: String(
                                                    "hello",
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
                                EvalLogicalNot {
                                    expr: RustValue(
                                        EvalCall {
                                            func_expr: RustValue(
                                                EvalGetAttr {
                                                    obj_expr: RustValue(
                                                        EvalLiteral {
                                                            value: String(
                                                                "hello world",
                                                            ),
                                                        },
                                                    ),
                                                    attr_name: "op_contains",
                                                },
                                            ),
                                            args: [
                                                RustValue(
                                                    EvalLiteral {
                                                        value: String(
                                                            "xyz",
                                                        ),
                                                    },
                                                ),
                                            ],
                                        },
                                    ),
                                },
                            ),
                        ],
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "my_list",
                        init_expr: Some(
                            RustValue(
                                EvalList {
                                    elements: [
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
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    3,
                                                ),
                                            },
                                        ),
                                        RustValue(
                                            EvalLiteral {
                                                value: String(
                                                    "hello",
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
                                                    name: "my_list",
                                                },
                                            ),
                                            attr_name: "op_contains",
                                        },
                                    ),
                                    args: [
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
                                                    name: "my_list",
                                                },
                                            ),
                                            attr_name: "op_contains",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: String(
                                                    "hello",
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
                                EvalLogicalNot {
                                    expr: RustValue(
                                        EvalCall {
                                            func_expr: RustValue(
                                                EvalGetAttr {
                                                    obj_expr: RustValue(
                                                        EvalVariable {
                                                            name: "my_list",
                                                        },
                                                    ),
                                                    attr_name: "op_contains",
                                                },
                                            ),
                                            args: [
                                                RustValue(
                                                    EvalLiteral {
                                                        value: Int(
                                                            99,
                                                        ),
                                                    },
                                                ),
                                            ],
                                        },
                                    ),
                                },
                            ),
                        ],
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "my_dict",
                        init_expr: Some(
                            RustValue(
                                EvalDict {
                                    pairs: [
                                        (
                                            RustValue(
                                                EvalLiteral {
                                                    value: String(
                                                        "a",
                                                    ),
                                                },
                                            ),
                                            RustValue(
                                                EvalLiteral {
                                                    value: Int(
                                                        1,
                                                    ),
                                                },
                                            ),
                                        ),
                                        (
                                            RustValue(
                                                EvalLiteral {
                                                    value: String(
                                                        "b",
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
                                        ),
                                        (
                                            RustValue(
                                                EvalLiteral {
                                                    value: Int(
                                                        3,
                                                    ),
                                                },
                                            ),
                                            RustValue(
                                                EvalLiteral {
                                                    value: String(
                                                        "three",
                                                    ),
                                                },
                                            ),
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
                                                    name: "my_dict",
                                                },
                                            ),
                                            attr_name: "op_contains",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: String(
                                                    "a",
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
                                                    name: "my_dict",
                                                },
                                            ),
                                            attr_name: "op_contains",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: String(
                                                    "b",
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
                                                    name: "my_dict",
                                                },
                                            ),
                                            attr_name: "op_contains",
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
                RustValue(
                    EvalCall {
                        func_expr: RustValue(
                            EvalVariable {
                                name: "assert",
                            },
                        ),
                        args: [
                            RustValue(
                                EvalLogicalNot {
                                    expr: RustValue(
                                        EvalCall {
                                            func_expr: RustValue(
                                                EvalGetAttr {
                                                    obj_expr: RustValue(
                                                        EvalVariable {
                                                            name: "my_dict",
                                                        },
                                                    ),
                                                    attr_name: "op_contains",
                                                },
                                            ),
                                            args: [
                                                RustValue(
                                                    EvalLiteral {
                                                        value: String(
                                                            "z",
                                                        ),
                                                    },
                                                ),
                                            ],
                                        },
                                    ),
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