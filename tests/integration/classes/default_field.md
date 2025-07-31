# Program
Status: 🟢
Assertions: 1

```rustleaf
class Greeter {
    var name = "Eric";

    fn greet() {
        "Hello, ${self.name}"
    }
}

var greeter = Greeter();
var msg = greeter.greet();
assert(msg == "Hello, Eric");
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0
parse_statement: starting at position 0
parse_expression: starting at position 6
parse_expression: success
parse_statement: starting at position 13
parse_statement: falling back to expression statement
parse_expression: starting at position 13
parse_expression: starting at position 15
parse_expression: success
parse_expression: success
parse_expression: starting at position 13
parse_expression: starting at position 15
parse_expression: success
parse_expression: success
parse_statement: parsed class declaration
parse_program: parsing statement at position 21
parse_statement: starting at position 21
parse_expression: starting at position 24
parse_expression: success
parse_statement: parsed var declaration
parse_program: parsing statement at position 28
parse_statement: starting at position 28
parse_expression: starting at position 31
parse_expression: success
parse_statement: parsed var declaration
parse_program: parsing statement at position 37
parse_statement: starting at position 37
parse_statement: falling back to expression statement
parse_expression: starting at position 37
parse_expression: starting at position 39
parse_expression: success
parse_expression: success
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
        Token(Class),
        Token(Ident, "Greeter"),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "name"),
        Token(Equal),
        Token(String, "Eric"),
        Token(Semicolon),
        Token(Fn),
        Token(Ident, "greet"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(StringPart, "Hello, "),
        Token(InterpolationStart),
        Token(Ident, "self"),
        Token(Dot),
        Token(Ident, "name"),
        Token(InterpolationEnd),
        Token(RightBrace),
        Token(RightBrace),
        Token(Var),
        Token(Ident, "greeter"),
        Token(Equal),
        Token(Ident, "Greeter"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "msg"),
        Token(Equal),
        Token(Ident, "greeter"),
        Token(Dot),
        Token(Ident, "greet"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "msg"),
        Token(EqualEqual),
        Token(String, "Hello, Eric"),
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
            ClassDecl {
                name: "Greeter",
                members: [
                    ClassMember {
                        name: "name",
                        kind: Field(
                            Some(
                                Literal(
                                    String(
                                        "Eric",
                                    ),
                                ),
                            ),
                        ),
                    },
                    ClassMember {
                        name: "greet",
                        kind: Method {
                            params: [],
                            body: Block {
                                statements: [],
                                final_expr: Some(
                                    InterpolatedString(
                                        [
                                            Text(
                                                "Hello, ",
                                            ),
                                            Expression(
                                                GetAttr(
                                                    Identifier(
                                                        "self",
                                                    ),
                                                    "name",
                                                ),
                                            ),
                                        ],
                                    ),
                                ),
                            },
                        },
                    },
                ],
                is_pub: false,
            },
            VarDecl {
                pattern: Variable(
                    "greeter",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "Greeter",
                        ),
                        [],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "msg",
                ),
                value: Some(
                    MethodCall(
                        Identifier(
                            "greeter",
                        ),
                        "greet",
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
                                "msg",
                            ),
                            Literal(
                                String(
                                    "Hello, Eric",
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
                    EvalClassDecl {
                        data: ClassDeclData {
                            name: "Greeter",
                            field_names: [
                                "name",
                            ],
                            field_defaults: [
                                Some(
                                    RustValue(
                                        EvalLiteral {
                                            value: String(
                                                "Eric",
                                            ),
                                        },
                                    ),
                                ),
                            ],
                            methods: [
                                ClassMethod {
                                    name: "greet",
                                    params: [
                                        "self",
                                    ],
                                    body: RustValue(
                                        EvalBlock {
                                            statements: [],
                                            final_expr: Some(
                                                RustValue(
                                                    EvalCall {
                                                        func_expr: RustValue(
                                                            EvalGetAttr {
                                                                obj_expr: RustValue(
                                                                    EvalLiteral {
                                                                        value: String(
                                                                            "Hello, ",
                                                                        ),
                                                                    },
                                                                ),
                                                                attr_name: "op_add",
                                                            },
                                                        ),
                                                        args: [
                                                            RustValue(
                                                                EvalCall {
                                                                    func_expr: RustValue(
                                                                        EvalVariable {
                                                                            name: "str",
                                                                        },
                                                                    ),
                                                                    args: [
                                                                        RustValue(
                                                                            EvalGetAttr {
                                                                                obj_expr: RustValue(
                                                                                    EvalVariable {
                                                                                        name: "self",
                                                                                    },
                                                                                ),
                                                                                attr_name: "name",
                                                                            },
                                                                        ),
                                                                    ],
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                    is_static: false,
                                },
                            ],
                        },
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "greeter",
                        init_expr: Some(
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalVariable {
                                            name: "Greeter",
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
                        name: "msg",
                        init_expr: Some(
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalGetAttr {
                                            obj_expr: RustValue(
                                                EvalVariable {
                                                    name: "greeter",
                                                },
                                            ),
                                            attr_name: "greet",
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
                                                    name: "msg",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: String(
                                                    "Hello, Eric",
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