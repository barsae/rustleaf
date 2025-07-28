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
    Eval(
        EvalRef(
            RefCell {
                value: EvalProgram {
                    statements: [
                        EvalRef(
                            RefCell {
                                value: EvalClassDecl {
                                    data: ClassDeclData {
                                        name: "Greeter",
                                        field_names: [
                                            "name",
                                        ],
                                        field_defaults: [
                                            Some(
                                                Eval(
                                                    EvalRef(
                                                        RefCell {
                                                            value: EvalLiteral {
                                                                value: String(
                                                                    "Eric",
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ],
                                        methods: [
                                            ClassMethod {
                                                name: "greet",
                                                params: [
                                                    "self",
                                                ],
                                                body: Eval(
                                                    EvalRef(
                                                        RefCell {
                                                            value: EvalBlock {
                                                                statements: [],
                                                                final_expr: Some(
                                                                    EvalRef(
                                                                        RefCell {
                                                                            value: EvalCall {
                                                                                func_expr: EvalRef(
                                                                                    RefCell {
                                                                                        value: EvalGetAttr {
                                                                                            obj_expr: EvalRef(
                                                                                                RefCell {
                                                                                                    value: EvalLiteral {
                                                                                                        value: String(
                                                                                                            "Hello, ",
                                                                                                        ),
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            attr_name: "op_add",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                args: [
                                                                                    EvalRef(
                                                                                        RefCell {
                                                                                            value: EvalCall {
                                                                                                func_expr: EvalRef(
                                                                                                    RefCell {
                                                                                                        value: EvalVariable {
                                                                                                            name: "str",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                args: [
                                                                                                    EvalRef(
                                                                                                        RefCell {
                                                                                                            value: EvalGetAttr {
                                                                                                                obj_expr: EvalRef(
                                                                                                                    RefCell {
                                                                                                                        value: EvalVariable {
                                                                                                                            name: "self",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                                attr_name: "name",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                ],
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                            },
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                                is_static: false,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalDeclare {
                                    name: "greeter",
                                    init_expr: Some(
                                        EvalRef(
                                            RefCell {
                                                value: EvalCall {
                                                    func_expr: EvalRef(
                                                        RefCell {
                                                            value: EvalVariable {
                                                                name: "Greeter",
                                                            },
                                                        },
                                                    ),
                                                    args: [],
                                                },
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalDeclare {
                                    name: "msg",
                                    init_expr: Some(
                                        EvalRef(
                                            RefCell {
                                                value: EvalCall {
                                                    func_expr: EvalRef(
                                                        RefCell {
                                                            value: EvalGetAttr {
                                                                obj_expr: EvalRef(
                                                                    RefCell {
                                                                        value: EvalVariable {
                                                                            name: "greeter",
                                                                        },
                                                                    },
                                                                ),
                                                                attr_name: "greet",
                                                            },
                                                        },
                                                    ),
                                                    args: [],
                                                },
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalCall {
                                    func_expr: EvalRef(
                                        RefCell {
                                            value: EvalVariable {
                                                name: "assert",
                                            },
                                        },
                                    ),
                                    args: [
                                        EvalRef(
                                            RefCell {
                                                value: EvalCall {
                                                    func_expr: EvalRef(
                                                        RefCell {
                                                            value: EvalGetAttr {
                                                                obj_expr: EvalRef(
                                                                    RefCell {
                                                                        value: EvalVariable {
                                                                            name: "msg",
                                                                        },
                                                                    },
                                                                ),
                                                                attr_name: "op_eq",
                                                            },
                                                        },
                                                    ),
                                                    args: [
                                                        EvalRef(
                                                            RefCell {
                                                                value: EvalLiteral {
                                                                    value: String(
                                                                        "Hello, Eric",
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                },
                                            },
                                        ),
                                    ],
                                },
                            },
                        ),
                    ],
                },
            },
        ),
    ),
)
```