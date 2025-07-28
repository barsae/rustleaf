# Program
Status: 🟢
Assertions: 1

```rustleaf
class Greeter {
    var name;

    fn greet() {
        "Hello, ${self.name}"
    }
}

var greeter = Greeter();
greeter.name = "Eric";
var msg = greeter.greet();
assert(msg == "Hello, Eric");
```

# Output
None

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
        Token(Ident, "greeter"),
        Token(Dot),
        Token(Ident, "name"),
        Token(Equal),
        Token(String, "Eric"),
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
                            None,
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
            Assignment {
                target: GetAttr(
                    Identifier(
                        "greeter",
                    ),
                    "name",
                ),
                op: Assign,
                value: Literal(
                    String(
                        "Eric",
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
    Eval(
        EvalRef(
            EvalProgram {
                statements: [
                    EvalRef(
                        EvalClassDecl {
                            data: ClassDeclData {
                                name: "Greeter",
                                field_names: [
                                    "name",
                                ],
                                field_defaults: [
                                    None,
                                ],
                                methods: [
                                    ClassMethod {
                                        name: "greet",
                                        params: [
                                            "self",
                                        ],
                                        body: Eval(
                                            EvalRef(
                                                EvalBlock {
                                                    statements: [],
                                                    final_expr: Some(
                                                        EvalRef(
                                                            EvalCall {
                                                                func_expr: EvalRef(
                                                                    EvalGetAttr {
                                                                        obj_expr: EvalRef(
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
                                                                    EvalRef(
                                                                        EvalCall {
                                                                            func_expr: EvalRef(
                                                                                EvalVariable {
                                                                                    name: "str",
                                                                                },
                                                                            ),
                                                                            args: [
                                                                                EvalRef(
                                                                                    EvalGetAttr {
                                                                                        obj_expr: EvalRef(
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
                                        ),
                                        is_static: false,
                                    },
                                ],
                            },
                        },
                    ),
                    EvalRef(
                        EvalDeclare {
                            name: "greeter",
                            init_expr: Some(
                                EvalRef(
                                    EvalCall {
                                        func_expr: EvalRef(
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
                    EvalRef(
                        EvalSetAttr {
                            obj_expr: EvalRef(
                                EvalVariable {
                                    name: "greeter",
                                },
                            ),
                            attr_name: "name",
                            value_expr: EvalRef(
                                EvalLiteral {
                                    value: String(
                                        "Eric",
                                    ),
                                },
                            ),
                        },
                    ),
                    EvalRef(
                        EvalDeclare {
                            name: "msg",
                            init_expr: Some(
                                EvalRef(
                                    EvalCall {
                                        func_expr: EvalRef(
                                            EvalGetAttr {
                                                obj_expr: EvalRef(
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
                    EvalRef(
                        EvalCall {
                            func_expr: EvalRef(
                                EvalVariable {
                                    name: "assert",
                                },
                            ),
                            args: [
                                EvalRef(
                                    EvalCall {
                                        func_expr: EvalRef(
                                            EvalGetAttr {
                                                obj_expr: EvalRef(
                                                    EvalVariable {
                                                        name: "msg",
                                                    },
                                                ),
                                                attr_name: "op_eq",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
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
    ),
)
```