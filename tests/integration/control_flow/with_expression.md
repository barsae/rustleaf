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
consume_token: position 0 consumed Class
consume_token: position 1 consumed Ident
consume_token: position 2 consumed LeftParen
consume_token: position 3 consumed RightParen
consume_token: position 4 consumed LeftBrace
consume_token: position 5 consumed Var
consume_token: position 6 consumed Ident
consume_token: position 7 consumed Semicolon
consume_token: position 8 consumed Var
consume_token: position 9 consumed Ident
consume_token: position 10 consumed Semicolon
consume_token: position 11 consumed Fn
consume_token: position 12 consumed Ident
consume_token: position 13 consumed LeftParen
consume_token: position 14 consumed RightParen
consume_token: position 15 consumed LeftBrace
parse_statement: starting at position 16 (Ident(self))
consume_token: position 16 consumed Ident
consume_token: position 17 consumed Dot
consume_token: position 18 consumed Ident
consume_token: position 19 consumed PlusEqual
parse_expression: starting at position 20 (Int(1))
consume_token: position 20 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 21 consumed Semicolon
parse_statement: success - parsed assignment
consume_token: position 22 consumed RightBrace
consume_token: position 23 consumed Fn
consume_token: position 24 consumed Ident
consume_token: position 25 consumed LeftParen
consume_token: position 26 consumed RightParen
consume_token: position 27 consumed LeftBrace
parse_statement: starting at position 28 (Ident(self))
consume_token: position 28 consumed Ident
consume_token: position 29 consumed Dot
consume_token: position 30 consumed Ident
consume_token: position 31 consumed PlusEqual
parse_expression: starting at position 32 (Int(1))
consume_token: position 32 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 33 consumed Semicolon
parse_statement: success - parsed assignment
consume_token: position 34 consumed RightBrace
consume_token: position 35 consumed Fn
consume_token: position 36 consumed Ident
consume_token: position 37 consumed LeftParen
consume_token: position 38 consumed RightParen
consume_token: position 39 consumed LeftBrace
consume_token: position 40 consumed RightBrace
consume_token: position 41 consumed RightBrace
parse_statement: success - parsed class declaration
parse_program: parsing statement at position 42 (Var)
parse_statement: starting at position 42 (Var)
consume_token: position 42 consumed Var
consume_token: position 43 consumed Ident
consume_token: position 44 consumed Equal
parse_expression: starting at position 45 (Ident(File))
consume_token: position 45 consumed Ident
parse_primary: success - parsed identifier (File)
consume_token: position 46 consumed LeftParen
consume_token: position 47 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 48 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 49 (Ident(f))
parse_statement: starting at position 49 (Ident(f))
consume_token: position 49 consumed Ident
consume_token: position 50 consumed Dot
consume_token: position 51 consumed Ident
consume_token: position 52 consumed Equal
parse_expression: starting at position 53 (Int(0))
consume_token: position 53 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 54 consumed Semicolon
parse_statement: success - parsed assignment
parse_program: parsing statement at position 55 (Ident(f))
parse_statement: starting at position 55 (Ident(f))
consume_token: position 55 consumed Ident
consume_token: position 56 consumed Dot
consume_token: position 57 consumed Ident
consume_token: position 58 consumed Equal
parse_expression: starting at position 59 (Int(0))
consume_token: position 59 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 60 consumed Semicolon
parse_statement: success - parsed assignment
parse_program: parsing statement at position 61 (With)
parse_statement: starting at position 61 (With)
parse_expression: starting at position 61 (With)
consume_token: position 61 consumed With
parse_primary: success - parsing with expression
consume_token: position 62 consumed Ident
parse_primary: success - parsed identifier (f)
consume_token: position 63 consumed LeftBrace
parse_statement: starting at position 64 (Ident(f))
consume_token: position 64 consumed Ident
consume_token: position 65 consumed Dot
consume_token: position 66 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 64 (Ident(f))
consume_token: position 64 consumed Ident
parse_primary: success - parsed identifier (f)
consume_token: position 65 consumed Dot
consume_token: position 66 consumed Ident
consume_token: position 67 consumed LeftParen
consume_token: position 68 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 69 consumed Semicolon
consume_token: position 70 consumed RightBrace
parse_expression: success - parsed precedence expression
parse_statement: success - parsed block-like expression statement
parse_program: parsing statement at position 71 (Ident(assert))
parse_statement: starting at position 71 (Ident(assert))
consume_token: position 71 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 71 (Ident(assert))
consume_token: position 71 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 72 consumed LeftParen
parse_expression: starting at position 73 (Ident(f))
consume_token: position 73 consumed Ident
parse_primary: success - parsed identifier (f)
consume_token: position 74 consumed Dot
consume_token: position 75 consumed Ident
consume_token: position 76 consumed EqualEqual
consume_token: position 77 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 78 consumed Comma
parse_expression: starting at position 79 (String(opened should be 1))
consume_token: position 79 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 80 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 81 consumed Semicolon
parse_program: parsing statement at position 82 (Ident(assert))
parse_statement: starting at position 82 (Ident(assert))
consume_token: position 82 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 82 (Ident(assert))
consume_token: position 82 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 83 consumed LeftParen
parse_expression: starting at position 84 (Ident(f))
consume_token: position 84 consumed Ident
parse_primary: success - parsed identifier (f)
consume_token: position 85 consumed Dot
consume_token: position 86 consumed Ident
consume_token: position 87 consumed EqualEqual
consume_token: position 88 consumed Int
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 89 consumed Comma
parse_expression: starting at position 90 (String(closed should be 1))
consume_token: position 90 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 91 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 92 consumed Semicolon
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
        0: Token(Class),
        1: Token(Ident, "File"),
        2: Token(LeftParen),
        3: Token(RightParen),
        4: Token(LeftBrace),
        5: Token(Var),
        6: Token(Ident, "opened"),
        7: Token(Semicolon),
        8: Token(Var),
        9: Token(Ident, "closed"),
        10: Token(Semicolon),
        11: Token(Fn),
        12: Token(Ident, "op_open"),
        13: Token(LeftParen),
        14: Token(RightParen),
        15: Token(LeftBrace),
        16: Token(Ident, "self"),
        17: Token(Dot),
        18: Token(Ident, "opened"),
        19: Token(PlusEqual),
        20: Token(Int, "1"),
        21: Token(Semicolon),
        22: Token(RightBrace),
        23: Token(Fn),
        24: Token(Ident, "op_close"),
        25: Token(LeftParen),
        26: Token(RightParen),
        27: Token(LeftBrace),
        28: Token(Ident, "self"),
        29: Token(Dot),
        30: Token(Ident, "closed"),
        31: Token(PlusEqual),
        32: Token(Int, "1"),
        33: Token(Semicolon),
        34: Token(RightBrace),
        35: Token(Fn),
        36: Token(Ident, "read"),
        37: Token(LeftParen),
        38: Token(RightParen),
        39: Token(LeftBrace),
        40: Token(RightBrace),
        41: Token(RightBrace),
        42: Token(Var),
        43: Token(Ident, "f"),
        44: Token(Equal),
        45: Token(Ident, "File"),
        46: Token(LeftParen),
        47: Token(RightParen),
        48: Token(Semicolon),
        49: Token(Ident, "f"),
        50: Token(Dot),
        51: Token(Ident, "opened"),
        52: Token(Equal),
        53: Token(Int, "0"),
        54: Token(Semicolon),
        55: Token(Ident, "f"),
        56: Token(Dot),
        57: Token(Ident, "closed"),
        58: Token(Equal),
        59: Token(Int, "0"),
        60: Token(Semicolon),
        61: Token(With),
        62: Token(Ident, "f"),
        63: Token(LeftBrace),
        64: Token(Ident, "f"),
        65: Token(Dot),
        66: Token(Ident, "read"),
        67: Token(LeftParen),
        68: Token(RightParen),
        69: Token(Semicolon),
        70: Token(RightBrace),
        71: Token(Ident, "assert"),
        72: Token(LeftParen),
        73: Token(Ident, "f"),
        74: Token(Dot),
        75: Token(Ident, "opened"),
        76: Token(EqualEqual),
        77: Token(Int, "1"),
        78: Token(Comma),
        79: Token(String, "opened should be 1"),
        80: Token(RightParen),
        81: Token(Semicolon),
        82: Token(Ident, "assert"),
        83: Token(LeftParen),
        84: Token(Ident, "f"),
        85: Token(Dot),
        86: Token(Ident, "closed"),
        87: Token(EqualEqual),
        88: Token(Int, "1"),
        89: Token(Comma),
        90: Token(String, "closed should be 1"),
        91: Token(RightParen),
        92: Token(Semicolon),
        93: Token(Eof)
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