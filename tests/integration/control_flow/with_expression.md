# Program
Status: 🟢
Assertions: 2

```rustleaf
class File() {
    var opened;
    var closed;

    fn op_open() {
        self.opened += 1;
    }

    fn op_close() {
        self.closed += 1;
    }

    fn read() {}
}

var f = File();
f.opened = 0;
f.closed = 0;
with f {
    f.read();
}
assert(f.opened == 1, "opened should be 1");
assert(f.closed == 1, "closed should be 1");
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
        Token(Ident, "File"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "opened"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "closed"),
        Token(Semicolon),
        Token(Fn),
        Token(Ident, "op_open"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "self"),
        Token(Dot),
        Token(Ident, "opened"),
        Token(PlusEqual),
        Token(Int, "1"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Fn),
        Token(Ident, "op_close"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "self"),
        Token(Dot),
        Token(Ident, "closed"),
        Token(PlusEqual),
        Token(Int, "1"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Fn),
        Token(Ident, "read"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(RightBrace),
        Token(RightBrace),
        Token(Var),
        Token(Ident, "f"),
        Token(Equal),
        Token(Ident, "File"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "f"),
        Token(Dot),
        Token(Ident, "opened"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(Ident, "f"),
        Token(Dot),
        Token(Ident, "closed"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(With),
        Token(Ident, "f"),
        Token(LeftBrace),
        Token(Ident, "f"),
        Token(Dot),
        Token(Ident, "read"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(RightBrace),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "f"),
        Token(Dot),
        Token(Ident, "opened"),
        Token(EqualEqual),
        Token(Int, "1"),
        Token(Comma),
        Token(String, "opened should be 1"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "f"),
        Token(Dot),
        Token(Ident, "closed"),
        Token(EqualEqual),
        Token(Int, "1"),
        Token(Comma),
        Token(String, "closed should be 1"),
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
                name: "File",
                members: [
                    ClassMember {
                        name: "opened",
                        kind: Field(
                            None,
                        ),
                    },
                    ClassMember {
                        name: "closed",
                        kind: Field(
                            None,
                        ),
                    },
                    ClassMember {
                        name: "op_open",
                        kind: Method {
                            params: [],
                            body: Block {
                                statements: [
                                    Assignment {
                                        target: GetAttr(
                                            Identifier(
                                                "self",
                                            ),
                                            "opened",
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
                    },
                    ClassMember {
                        name: "op_close",
                        kind: Method {
                            params: [],
                            body: Block {
                                statements: [
                                    Assignment {
                                        target: GetAttr(
                                            Identifier(
                                                "self",
                                            ),
                                            "closed",
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
                    },
                    ClassMember {
                        name: "read",
                        kind: Method {
                            params: [],
                            body: Block {
                                statements: [],
                                final_expr: None,
                            },
                        },
                    },
                ],
                is_pub: false,
            },
            VarDecl {
                pattern: Variable(
                    "f",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "File",
                        ),
                        [],
                    ),
                ),
            },
            Assignment {
                target: GetAttr(
                    Identifier(
                        "f",
                    ),
                    "opened",
                ),
                op: Assign,
                value: Literal(
                    Int(
                        0,
                    ),
                ),
            },
            Assignment {
                target: GetAttr(
                    Identifier(
                        "f",
                    ),
                    "closed",
                ),
                op: Assign,
                value: Literal(
                    Int(
                        0,
                    ),
                ),
            },
            Expression(
                With {
                    resources: [
                        WithResource {
                            name: "f",
                            value: Identifier(
                                "f",
                            ),
                        },
                    ],
                    body: Block {
                        statements: [
                            Expression(
                                MethodCall(
                                    Identifier(
                                        "f",
                                    ),
                                    "read",
                                    [],
                                ),
                            ),
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
                            GetAttr(
                                Identifier(
                                    "f",
                                ),
                                "opened",
                            ),
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                        ),
                        Literal(
                            String(
                                "opened should be 1",
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
                            GetAttr(
                                Identifier(
                                    "f",
                                ),
                                "closed",
                            ),
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                        ),
                        Literal(
                            String(
                                "closed should be 1",
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
                                        name: "File",
                                        field_names: [
                                            "opened",
                                            "closed",
                                        ],
                                        field_defaults: [
                                            None,
                                            None,
                                        ],
                                        methods: [
                                            ClassMethod {
                                                name: "op_open",
                                                params: [
                                                    "self",
                                                ],
                                                body: Eval(
                                                    EvalRef(
                                                        RefCell {
                                                            value: EvalBlock {
                                                                statements: [
                                                                    EvalRef(
                                                                        RefCell {
                                                                            value: EvalSetAttr {
                                                                                obj_expr: EvalRef(
                                                                                    RefCell {
                                                                                        value: EvalVariable {
                                                                                            name: "self",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                attr_name: "opened",
                                                                                value_expr: EvalRef(
                                                                                    RefCell {
                                                                                        value: EvalCall {
                                                                                            func_expr: EvalRef(
                                                                                                RefCell {
                                                                                                    value: EvalGetAttr {
                                                                                                        obj_expr: EvalRef(
                                                                                                            RefCell {
                                                                                                                value: EvalGetAttr {
                                                                                                                    obj_expr: EvalRef(
                                                                                                                        RefCell {
                                                                                                                            value: EvalVariable {
                                                                                                                                name: "self",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    attr_name: "opened",
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
                                                                                                        value: EvalLiteral {
                                                                                                            value: Int(
                                                                                                                1,
                                                                                                            ),
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            ],
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_expr: None,
                                                            },
                                                        },
                                                    ),
                                                ),
                                                is_static: false,
                                            },
                                            ClassMethod {
                                                name: "op_close",
                                                params: [
                                                    "self",
                                                ],
                                                body: Eval(
                                                    EvalRef(
                                                        RefCell {
                                                            value: EvalBlock {
                                                                statements: [
                                                                    EvalRef(
                                                                        RefCell {
                                                                            value: EvalSetAttr {
                                                                                obj_expr: EvalRef(
                                                                                    RefCell {
                                                                                        value: EvalVariable {
                                                                                            name: "self",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                attr_name: "closed",
                                                                                value_expr: EvalRef(
                                                                                    RefCell {
                                                                                        value: EvalCall {
                                                                                            func_expr: EvalRef(
                                                                                                RefCell {
                                                                                                    value: EvalGetAttr {
                                                                                                        obj_expr: EvalRef(
                                                                                                            RefCell {
                                                                                                                value: EvalGetAttr {
                                                                                                                    obj_expr: EvalRef(
                                                                                                                        RefCell {
                                                                                                                            value: EvalVariable {
                                                                                                                                name: "self",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    attr_name: "closed",
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
                                                                                                        value: EvalLiteral {
                                                                                                            value: Int(
                                                                                                                1,
                                                                                                            ),
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            ],
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_expr: None,
                                                            },
                                                        },
                                                    ),
                                                ),
                                                is_static: false,
                                            },
                                            ClassMethod {
                                                name: "read",
                                                params: [
                                                    "self",
                                                ],
                                                body: Eval(
                                                    EvalRef(
                                                        RefCell {
                                                            value: EvalBlock {
                                                                statements: [],
                                                                final_expr: None,
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
                                    name: "f",
                                    init_expr: Some(
                                        EvalRef(
                                            RefCell {
                                                value: EvalCall {
                                                    func_expr: EvalRef(
                                                        RefCell {
                                                            value: EvalVariable {
                                                                name: "File",
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
                                value: EvalSetAttr {
                                    obj_expr: EvalRef(
                                        RefCell {
                                            value: EvalVariable {
                                                name: "f",
                                            },
                                        },
                                    ),
                                    attr_name: "opened",
                                    value_expr: EvalRef(
                                        RefCell {
                                            value: EvalLiteral {
                                                value: Int(
                                                    0,
                                                ),
                                            },
                                        },
                                    ),
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalSetAttr {
                                    obj_expr: EvalRef(
                                        RefCell {
                                            value: EvalVariable {
                                                name: "f",
                                            },
                                        },
                                    ),
                                    attr_name: "closed",
                                    value_expr: EvalRef(
                                        RefCell {
                                            value: EvalLiteral {
                                                value: Int(
                                                    0,
                                                ),
                                            },
                                        },
                                    ),
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalWith {
                                    data: WithData {
                                        resources: [
                                            (
                                                "f",
                                                Eval(
                                                    EvalRef(
                                                        RefCell {
                                                            value: EvalVariable {
                                                                name: "f",
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ],
                                        body: Eval(
                                            EvalRef(
                                                RefCell {
                                                    value: EvalBlock {
                                                        statements: [
                                                            EvalRef(
                                                                RefCell {
                                                                    value: EvalCall {
                                                                        func_expr: EvalRef(
                                                                            RefCell {
                                                                                value: EvalGetAttr {
                                                                                    obj_expr: EvalRef(
                                                                                        RefCell {
                                                                                            value: EvalVariable {
                                                                                                name: "f",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    attr_name: "read",
                                                                                },
                                                                            },
                                                                        ),
                                                                        args: [],
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                        final_expr: None,
                                                    },
                                                },
                                            ),
                                        ),
                                    },
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
                                                                        value: EvalGetAttr {
                                                                            obj_expr: EvalRef(
                                                                                RefCell {
                                                                                    value: EvalVariable {
                                                                                        name: "f",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            attr_name: "opened",
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
                                                                    value: Int(
                                                                        1,
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                },
                                            },
                                        ),
                                        EvalRef(
                                            RefCell {
                                                value: EvalLiteral {
                                                    value: String(
                                                        "opened should be 1",
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
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
                                                                        value: EvalGetAttr {
                                                                            obj_expr: EvalRef(
                                                                                RefCell {
                                                                                    value: EvalVariable {
                                                                                        name: "f",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            attr_name: "closed",
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
                                                                    value: Int(
                                                                        1,
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                },
                                            },
                                        ),
                                        EvalRef(
                                            RefCell {
                                                value: EvalLiteral {
                                                    value: String(
                                                        "closed should be 1",
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
    ),
)
```