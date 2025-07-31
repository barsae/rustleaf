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
```
parse_program: starting
parse_program: parsing statement at position 0 (Class)
parse_statement: starting at position 0 (Class)
parse_statement: starting at position 16 (Ident(self))
parse_expression: starting at position 20 (Int(1))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: success - parsed assignment
parse_statement: starting at position 28 (Ident(self))
parse_expression: starting at position 32 (Int(1))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: success - parsed assignment
parse_statement: success - parsed class declaration
parse_program: parsing statement at position 42 (Var)
parse_statement: starting at position 42 (Var)
parse_expression: starting at position 45 (Ident(File))
parse_primary: success - parsed identifier (File)
parse_expression: success - parsed precedence expression
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 49 (Ident(f))
parse_statement: starting at position 49 (Ident(f))
parse_expression: starting at position 53 (Int(0))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: success - parsed assignment
parse_program: parsing statement at position 55 (Ident(f))
parse_statement: starting at position 55 (Ident(f))
parse_expression: starting at position 59 (Int(0))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_statement: success - parsed assignment
parse_program: parsing statement at position 61 (With)
parse_statement: starting at position 61 (With)
parse_expression: starting at position 61 (With)
parse_primary: success - parsing with expression
parse_primary: success - parsed identifier (f)
parse_statement: starting at position 64 (Ident(f))
parse_statement: falling back to expression statement
parse_expression: starting at position 64 (Ident(f))
parse_primary: success - parsed identifier (f)
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_statement: success - parsed block-like expression statement
parse_program: parsing statement at position 71 (Ident(assert))
parse_statement: starting at position 71 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 71 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 73 (Ident(f))
parse_primary: success - parsed identifier (f)
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: starting at position 79 (String(opened should be 1))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_program: parsing statement at position 82 (Ident(assert))
parse_statement: starting at position 82 (Ident(assert))
parse_statement: falling back to expression statement
parse_expression: starting at position 82 (Ident(assert))
parse_primary: success - parsed identifier (assert)
parse_expression: starting at position 84 (Ident(f))
parse_primary: success - parsed identifier (f)
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: starting at position 90 (String(closed should be 1))
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
parse_expression: success - parsed precedence expression
parse_program: parsed 7 statements
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
                            name: "",
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
    RustValue(
        EvalProgram {
            statements: [
                RustValue(
                    EvalClassDecl {
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
                                    body: RustValue(
                                        EvalBlock {
                                            statements: [
                                                RustValue(
                                                    EvalSetAttr {
                                                        obj_expr: RustValue(
                                                            EvalVariable {
                                                                name: "self",
                                                            },
                                                        ),
                                                        attr_name: "opened",
                                                        value_expr: RustValue(
                                                            EvalCall {
                                                                func_expr: RustValue(
                                                                    EvalGetAttr {
                                                                        obj_expr: RustValue(
                                                                            EvalGetAttr {
                                                                                obj_expr: RustValue(
                                                                                    EvalVariable {
                                                                                        name: "self",
                                                                                    },
                                                                                ),
                                                                                attr_name: "opened",
                                                                            },
                                                                        ),
                                                                        attr_name: "op_add",
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
                                                                ],
                                                            },
                                                        ),
                                                    },
                                                ),
                                            ],
                                            final_expr: None,
                                        },
                                    ),
                                    is_static: false,
                                },
                                ClassMethod {
                                    name: "op_close",
                                    params: [
                                        "self",
                                    ],
                                    body: RustValue(
                                        EvalBlock {
                                            statements: [
                                                RustValue(
                                                    EvalSetAttr {
                                                        obj_expr: RustValue(
                                                            EvalVariable {
                                                                name: "self",
                                                            },
                                                        ),
                                                        attr_name: "closed",
                                                        value_expr: RustValue(
                                                            EvalCall {
                                                                func_expr: RustValue(
                                                                    EvalGetAttr {
                                                                        obj_expr: RustValue(
                                                                            EvalGetAttr {
                                                                                obj_expr: RustValue(
                                                                                    EvalVariable {
                                                                                        name: "self",
                                                                                    },
                                                                                ),
                                                                                attr_name: "closed",
                                                                            },
                                                                        ),
                                                                        attr_name: "op_add",
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
                                                                ],
                                                            },
                                                        ),
                                                    },
                                                ),
                                            ],
                                            final_expr: None,
                                        },
                                    ),
                                    is_static: false,
                                },
                                ClassMethod {
                                    name: "read",
                                    params: [
                                        "self",
                                    ],
                                    body: RustValue(
                                        EvalBlock {
                                            statements: [],
                                            final_expr: None,
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
                        name: "f",
                        init_expr: Some(
                            RustValue(
                                EvalCall {
                                    func_expr: RustValue(
                                        EvalVariable {
                                            name: "File",
                                        },
                                    ),
                                    args: [],
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalSetAttr {
                        obj_expr: RustValue(
                            EvalVariable {
                                name: "f",
                            },
                        ),
                        attr_name: "opened",
                        value_expr: RustValue(
                            EvalLiteral {
                                value: Int(
                                    0,
                                ),
                            },
                        ),
                    },
                ),
                RustValue(
                    EvalSetAttr {
                        obj_expr: RustValue(
                            EvalVariable {
                                name: "f",
                            },
                        ),
                        attr_name: "closed",
                        value_expr: RustValue(
                            EvalLiteral {
                                value: Int(
                                    0,
                                ),
                            },
                        ),
                    },
                ),
                RustValue(
                    EvalWith {
                        data: WithData {
                            resources: [
                                (
                                    "",
                                    RustValue(
                                        EvalVariable {
                                            name: "f",
                                        },
                                    ),
                                ),
                            ],
                            body: RustValue(
                                EvalBlock {
                                    statements: [
                                        RustValue(
                                            EvalCall {
                                                func_expr: RustValue(
                                                    EvalGetAttr {
                                                        obj_expr: RustValue(
                                                            EvalVariable {
                                                                name: "f",
                                                            },
                                                        ),
                                                        attr_name: "read",
                                                    },
                                                ),
                                                args: [],
                                            },
                                        ),
                                    ],
                                    final_expr: None,
                                },
                            ),
                        },
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
                                                EvalGetAttr {
                                                    obj_expr: RustValue(
                                                        EvalVariable {
                                                            name: "f",
                                                        },
                                                    ),
                                                    attr_name: "opened",
                                                },
                                            ),
                                            attr_name: "op_eq",
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
                                    ],
                                },
                            ),
                            RustValue(
                                EvalLiteral {
                                    value: String(
                                        "opened should be 1",
                                    ),
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
                                                EvalGetAttr {
                                                    obj_expr: RustValue(
                                                        EvalVariable {
                                                            name: "f",
                                                        },
                                                    ),
                                                    attr_name: "closed",
                                                },
                                            ),
                                            attr_name: "op_eq",
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
                                    ],
                                },
                            ),
                            RustValue(
                                EvalLiteral {
                                    value: String(
                                        "closed should be 1",
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