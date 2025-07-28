# Program
Status: 🟢
Assertions: 5

```rustleaf
var i = 0;
var result = loop {
    i = i + 1;
    if i < 3 {
        continue;
    }
    break i * 10;
};

var j = 0;
var count = 0;
var result2 = loop {
    j = j + 1;
    if j <= 5 {
        count = count + 1;
        continue;
    }
    break j + count;
};

assert(result == 30);
assert(i == 3);
assert(result2 == 11);  
assert(j == 6);
assert(count == 5);
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
        Token(Var),
        Token(Ident, "i"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result"),
        Token(Equal),
        Token(Loop),
        Token(LeftBrace),
        Token(Ident, "i"),
        Token(Equal),
        Token(Ident, "i"),
        Token(Plus),
        Token(Int, "1"),
        Token(Semicolon),
        Token(If),
        Token(Ident, "i"),
        Token(Less),
        Token(Int, "3"),
        Token(LeftBrace),
        Token(Continue),
        Token(Semicolon),
        Token(RightBrace),
        Token(Break),
        Token(Ident, "i"),
        Token(Star),
        Token(Int, "10"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "j"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "count"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result2"),
        Token(Equal),
        Token(Loop),
        Token(LeftBrace),
        Token(Ident, "j"),
        Token(Equal),
        Token(Ident, "j"),
        Token(Plus),
        Token(Int, "1"),
        Token(Semicolon),
        Token(If),
        Token(Ident, "j"),
        Token(LessEqual),
        Token(Int, "5"),
        Token(LeftBrace),
        Token(Ident, "count"),
        Token(Equal),
        Token(Ident, "count"),
        Token(Plus),
        Token(Int, "1"),
        Token(Semicolon),
        Token(Continue),
        Token(Semicolon),
        Token(RightBrace),
        Token(Break),
        Token(Ident, "j"),
        Token(Plus),
        Token(Ident, "count"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result"),
        Token(EqualEqual),
        Token(Int, "30"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "i"),
        Token(EqualEqual),
        Token(Int, "3"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result2"),
        Token(EqualEqual),
        Token(Int, "11"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "j"),
        Token(EqualEqual),
        Token(Int, "6"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "count"),
        Token(EqualEqual),
        Token(Int, "5"),
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
            VarDecl {
                pattern: Variable(
                    "i",
                ),
                value: Some(
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result",
                ),
                value: Some(
                    Loop {
                        body: Block {
                            statements: [
                                Assignment {
                                    target: Identifier(
                                        "i",
                                    ),
                                    op: Assign,
                                    value: Add(
                                        Identifier(
                                            "i",
                                        ),
                                        Literal(
                                            Int(
                                                1,
                                            ),
                                        ),
                                    ),
                                },
                                Expression(
                                    If {
                                        condition: Lt(
                                            Identifier(
                                                "i",
                                            ),
                                            Literal(
                                                Int(
                                                    3,
                                                ),
                                            ),
                                        ),
                                        then_expr: Block {
                                            statements: [
                                                Continue,
                                            ],
                                            final_expr: None,
                                        },
                                        else_expr: None,
                                    },
                                ),
                                Break(
                                    Some(
                                        Mul(
                                            Identifier(
                                                "i",
                                            ),
                                            Literal(
                                                Int(
                                                    10,
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            final_expr: None,
                        },
                    },
                ),
            },
            VarDecl {
                pattern: Variable(
                    "j",
                ),
                value: Some(
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "count",
                ),
                value: Some(
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "result2",
                ),
                value: Some(
                    Loop {
                        body: Block {
                            statements: [
                                Assignment {
                                    target: Identifier(
                                        "j",
                                    ),
                                    op: Assign,
                                    value: Add(
                                        Identifier(
                                            "j",
                                        ),
                                        Literal(
                                            Int(
                                                1,
                                            ),
                                        ),
                                    ),
                                },
                                Expression(
                                    If {
                                        condition: Le(
                                            Identifier(
                                                "j",
                                            ),
                                            Literal(
                                                Int(
                                                    5,
                                                ),
                                            ),
                                        ),
                                        then_expr: Block {
                                            statements: [
                                                Assignment {
                                                    target: Identifier(
                                                        "count",
                                                    ),
                                                    op: Assign,
                                                    value: Add(
                                                        Identifier(
                                                            "count",
                                                        ),
                                                        Literal(
                                                            Int(
                                                                1,
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Continue,
                                            ],
                                            final_expr: None,
                                        },
                                        else_expr: None,
                                    },
                                ),
                                Break(
                                    Some(
                                        Add(
                                            Identifier(
                                                "j",
                                            ),
                                            Identifier(
                                                "count",
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            final_expr: None,
                        },
                    },
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
                                "result",
                            ),
                            Literal(
                                Int(
                                    30,
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
                                "i",
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
                                Int(
                                    11,
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
                                "j",
                            ),
                            Literal(
                                Int(
                                    6,
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
                                "count",
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
        ],
    ),
)
```

# Eval
```rust
Ok(
    RustValueRef(
        RefCell {
            value: EvalProgram {
                statements: [
                    RustValueRef(
                        RefCell {
                            value: EvalDeclare {
                                name: "i",
                                init_expr: Some(
                                    RustValueRef(
                                        RefCell {
                                            value: EvalLiteral {
                                                value: Int(
                                                    0,
                                                ),
                                            },
                                        },
                                    ),
                                ),
                            },
                        },
                    ),
                    RustValueRef(
                        RefCell {
                            value: EvalDeclare {
                                name: "result",
                                init_expr: Some(
                                    RustValueRef(
                                        RefCell {
                                            value: EvalLoop {
                                                body: RustValueRef(
                                                    RefCell {
                                                        value: EvalBlock {
                                                            statements: [
                                                                RustValueRef(
                                                                    RefCell {
                                                                        value: EvalAssign {
                                                                            name: "i",
                                                                            expr: RustValueRef(
                                                                                RefCell {
                                                                                    value: EvalCall {
                                                                                        func_expr: RustValueRef(
                                                                                            RefCell {
                                                                                                value: EvalGetAttr {
                                                                                                    obj_expr: RustValueRef(
                                                                                                        RefCell {
                                                                                                            value: EvalVariable {
                                                                                                                name: "i",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    attr_name: "op_add",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        args: [
                                                                                            RustValueRef(
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
                                                                RustValueRef(
                                                                    RefCell {
                                                                        value: EvalIf {
                                                                            condition: RustValueRef(
                                                                                RefCell {
                                                                                    value: EvalCall {
                                                                                        func_expr: RustValueRef(
                                                                                            RefCell {
                                                                                                value: EvalGetAttr {
                                                                                                    obj_expr: RustValueRef(
                                                                                                        RefCell {
                                                                                                            value: EvalVariable {
                                                                                                                name: "i",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    attr_name: "op_lt",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        args: [
                                                                                            RustValueRef(
                                                                                                RefCell {
                                                                                                    value: EvalLiteral {
                                                                                                        value: Int(
                                                                                                            3,
                                                                                                        ),
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                    },
                                                                                },
                                                                            ),
                                                                            then_expr: RustValueRef(
                                                                                RefCell {
                                                                                    value: EvalBlock {
                                                                                        statements: [
                                                                                            RustValueRef(
                                                                                                RefCell {
                                                                                                    value: EvalContinue,
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                        final_expr: None,
                                                                                    },
                                                                                },
                                                                            ),
                                                                            else_expr: None,
                                                                        },
                                                                    },
                                                                ),
                                                                RustValueRef(
                                                                    RefCell {
                                                                        value: EvalBreak {
                                                                            expr: Some(
                                                                                RustValueRef(
                                                                                    RefCell {
                                                                                        value: EvalCall {
                                                                                            func_expr: RustValueRef(
                                                                                                RefCell {
                                                                                                    value: EvalGetAttr {
                                                                                                        obj_expr: RustValueRef(
                                                                                                            RefCell {
                                                                                                                value: EvalVariable {
                                                                                                                    name: "i",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                        attr_name: "op_mul",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            args: [
                                                                                                RustValueRef(
                                                                                                    RefCell {
                                                                                                        value: EvalLiteral {
                                                                                                            value: Int(
                                                                                                                10,
                                                                                                            ),
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
                                                            ],
                                                            final_expr: None,
                                                        },
                                                    },
                                                ),
                                            },
                                        },
                                    ),
                                ),
                            },
                        },
                    ),
                    RustValueRef(
                        RefCell {
                            value: EvalDeclare {
                                name: "j",
                                init_expr: Some(
                                    RustValueRef(
                                        RefCell {
                                            value: EvalLiteral {
                                                value: Int(
                                                    0,
                                                ),
                                            },
                                        },
                                    ),
                                ),
                            },
                        },
                    ),
                    RustValueRef(
                        RefCell {
                            value: EvalDeclare {
                                name: "count",
                                init_expr: Some(
                                    RustValueRef(
                                        RefCell {
                                            value: EvalLiteral {
                                                value: Int(
                                                    0,
                                                ),
                                            },
                                        },
                                    ),
                                ),
                            },
                        },
                    ),
                    RustValueRef(
                        RefCell {
                            value: EvalDeclare {
                                name: "result2",
                                init_expr: Some(
                                    RustValueRef(
                                        RefCell {
                                            value: EvalLoop {
                                                body: RustValueRef(
                                                    RefCell {
                                                        value: EvalBlock {
                                                            statements: [
                                                                RustValueRef(
                                                                    RefCell {
                                                                        value: EvalAssign {
                                                                            name: "j",
                                                                            expr: RustValueRef(
                                                                                RefCell {
                                                                                    value: EvalCall {
                                                                                        func_expr: RustValueRef(
                                                                                            RefCell {
                                                                                                value: EvalGetAttr {
                                                                                                    obj_expr: RustValueRef(
                                                                                                        RefCell {
                                                                                                            value: EvalVariable {
                                                                                                                name: "j",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    attr_name: "op_add",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        args: [
                                                                                            RustValueRef(
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
                                                                RustValueRef(
                                                                    RefCell {
                                                                        value: EvalIf {
                                                                            condition: RustValueRef(
                                                                                RefCell {
                                                                                    value: EvalCall {
                                                                                        func_expr: RustValueRef(
                                                                                            RefCell {
                                                                                                value: EvalGetAttr {
                                                                                                    obj_expr: RustValueRef(
                                                                                                        RefCell {
                                                                                                            value: EvalVariable {
                                                                                                                name: "j",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    attr_name: "op_le",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        args: [
                                                                                            RustValueRef(
                                                                                                RefCell {
                                                                                                    value: EvalLiteral {
                                                                                                        value: Int(
                                                                                                            5,
                                                                                                        ),
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                    },
                                                                                },
                                                                            ),
                                                                            then_expr: RustValueRef(
                                                                                RefCell {
                                                                                    value: EvalBlock {
                                                                                        statements: [
                                                                                            RustValueRef(
                                                                                                RefCell {
                                                                                                    value: EvalAssign {
                                                                                                        name: "count",
                                                                                                        expr: RustValueRef(
                                                                                                            RefCell {
                                                                                                                value: EvalCall {
                                                                                                                    func_expr: RustValueRef(
                                                                                                                        RefCell {
                                                                                                                            value: EvalGetAttr {
                                                                                                                                obj_expr: RustValueRef(
                                                                                                                                    RefCell {
                                                                                                                                        value: EvalVariable {
                                                                                                                                            name: "count",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                ),
                                                                                                                                attr_name: "op_add",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    args: [
                                                                                                                        RustValueRef(
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
                                                                                            RustValueRef(
                                                                                                RefCell {
                                                                                                    value: EvalContinue,
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                        final_expr: None,
                                                                                    },
                                                                                },
                                                                            ),
                                                                            else_expr: None,
                                                                        },
                                                                    },
                                                                ),
                                                                RustValueRef(
                                                                    RefCell {
                                                                        value: EvalBreak {
                                                                            expr: Some(
                                                                                RustValueRef(
                                                                                    RefCell {
                                                                                        value: EvalCall {
                                                                                            func_expr: RustValueRef(
                                                                                                RefCell {
                                                                                                    value: EvalGetAttr {
                                                                                                        obj_expr: RustValueRef(
                                                                                                            RefCell {
                                                                                                                value: EvalVariable {
                                                                                                                    name: "j",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                        attr_name: "op_add",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            args: [
                                                                                                RustValueRef(
                                                                                                    RefCell {
                                                                                                        value: EvalVariable {
                                                                                                            name: "count",
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
                                                            ],
                                                            final_expr: None,
                                                        },
                                                    },
                                                ),
                                            },
                                        },
                                    ),
                                ),
                            },
                        },
                    ),
                    RustValueRef(
                        RefCell {
                            value: EvalCall {
                                func_expr: RustValueRef(
                                    RefCell {
                                        value: EvalVariable {
                                            name: "assert",
                                        },
                                    },
                                ),
                                args: [
                                    RustValueRef(
                                        RefCell {
                                            value: EvalCall {
                                                func_expr: RustValueRef(
                                                    RefCell {
                                                        value: EvalGetAttr {
                                                            obj_expr: RustValueRef(
                                                                RefCell {
                                                                    value: EvalVariable {
                                                                        name: "result",
                                                                    },
                                                                },
                                                            ),
                                                            attr_name: "op_eq",
                                                        },
                                                    },
                                                ),
                                                args: [
                                                    RustValueRef(
                                                        RefCell {
                                                            value: EvalLiteral {
                                                                value: Int(
                                                                    30,
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
                    RustValueRef(
                        RefCell {
                            value: EvalCall {
                                func_expr: RustValueRef(
                                    RefCell {
                                        value: EvalVariable {
                                            name: "assert",
                                        },
                                    },
                                ),
                                args: [
                                    RustValueRef(
                                        RefCell {
                                            value: EvalCall {
                                                func_expr: RustValueRef(
                                                    RefCell {
                                                        value: EvalGetAttr {
                                                            obj_expr: RustValueRef(
                                                                RefCell {
                                                                    value: EvalVariable {
                                                                        name: "i",
                                                                    },
                                                                },
                                                            ),
                                                            attr_name: "op_eq",
                                                        },
                                                    },
                                                ),
                                                args: [
                                                    RustValueRef(
                                                        RefCell {
                                                            value: EvalLiteral {
                                                                value: Int(
                                                                    3,
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
                    RustValueRef(
                        RefCell {
                            value: EvalCall {
                                func_expr: RustValueRef(
                                    RefCell {
                                        value: EvalVariable {
                                            name: "assert",
                                        },
                                    },
                                ),
                                args: [
                                    RustValueRef(
                                        RefCell {
                                            value: EvalCall {
                                                func_expr: RustValueRef(
                                                    RefCell {
                                                        value: EvalGetAttr {
                                                            obj_expr: RustValueRef(
                                                                RefCell {
                                                                    value: EvalVariable {
                                                                        name: "result2",
                                                                    },
                                                                },
                                                            ),
                                                            attr_name: "op_eq",
                                                        },
                                                    },
                                                ),
                                                args: [
                                                    RustValueRef(
                                                        RefCell {
                                                            value: EvalLiteral {
                                                                value: Int(
                                                                    11,
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
                    RustValueRef(
                        RefCell {
                            value: EvalCall {
                                func_expr: RustValueRef(
                                    RefCell {
                                        value: EvalVariable {
                                            name: "assert",
                                        },
                                    },
                                ),
                                args: [
                                    RustValueRef(
                                        RefCell {
                                            value: EvalCall {
                                                func_expr: RustValueRef(
                                                    RefCell {
                                                        value: EvalGetAttr {
                                                            obj_expr: RustValueRef(
                                                                RefCell {
                                                                    value: EvalVariable {
                                                                        name: "j",
                                                                    },
                                                                },
                                                            ),
                                                            attr_name: "op_eq",
                                                        },
                                                    },
                                                ),
                                                args: [
                                                    RustValueRef(
                                                        RefCell {
                                                            value: EvalLiteral {
                                                                value: Int(
                                                                    6,
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
                    RustValueRef(
                        RefCell {
                            value: EvalCall {
                                func_expr: RustValueRef(
                                    RefCell {
                                        value: EvalVariable {
                                            name: "assert",
                                        },
                                    },
                                ),
                                args: [
                                    RustValueRef(
                                        RefCell {
                                            value: EvalCall {
                                                func_expr: RustValueRef(
                                                    RefCell {
                                                        value: EvalGetAttr {
                                                            obj_expr: RustValueRef(
                                                                RefCell {
                                                                    value: EvalVariable {
                                                                        name: "count",
                                                                    },
                                                                },
                                                            ),
                                                            attr_name: "op_eq",
                                                        },
                                                    },
                                                ),
                                                args: [
                                                    RustValueRef(
                                                        RefCell {
                                                            value: EvalLiteral {
                                                                value: Int(
                                                                    5,
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
)
```