# Program
Status: 🔴
Assertions: 0

```rustleaf
#[macro]
fn memoize(f) {
    var param_list = join(f.params, ", ");

    parse("fn ${f.name}(${param_list}) {
        var cache = {};

        fn original(${param_list}) ${f.body}

        fn cached(${param_list}) {
            var args_key = str(${param_list});
            if args_key in cache {
                cache[args_key]
            } else {
                var result = original(${param_list});
                cache[args_key] = result;
                result
            }
        }

        cached(${param_list})
    }")
}

var count = 0;
#[memoize]
fn fibonacci(n) {
    count += 1;
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

var f = fibonacci(10);
assert(f == 55, "f");
assert(count == 11, "count");
```

# Output
None

# Result
```rust
Err(
    "Undefined variable: memoize",
)
```

# Lex
```rust
Ok(
    [
        Token(Hash),
        Token(LeftBracket),
        Token(Macro),
        Token(RightBracket),
        Token(Fn),
        Token(Ident, "memoize"),
        Token(LeftParen),
        Token(Ident, "eval_node"),
        Token(RightParen),
        Token(LeftBrace),
        Token(If),
        Token(Ident, "eval_node"),
        Token(IsNot),
        Token(Ident, "Eval"),
        Token(Dot),
        Token(Ident, "Function"),
        Token(LeftBrace),
        Token(Ident, "raise"),
        Token(LeftParen),
        Token(String, "#[memoize] can only be applied to functions"),
        Token(RightParen),
        Token(Semicolon),
        Token(RightBrace),
        Token(Var),
        Token(Ident, "func"),
        Token(Equal),
        Token(Ident, "eval_node"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "param_list"),
        Token(Equal),
        Token(Ident, "join"),
        Token(LeftParen),
        Token(Ident, "func"),
        Token(Dot),
        Token(Ident, "params"),
        Token(Comma),
        Token(String, ", "),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "parse"),
        Token(LeftParen),
        Token(StringPart, "fn "),
        Token(InterpolationStart),
        Token(Ident, "func"),
        Token(Dot),
        Token(Ident, "name"),
        Token(InterpolationEnd),
        Token(StringPart, "("),
        Token(InterpolationStart),
        Token(Ident, "param_list"),
        Token(InterpolationEnd),
        Token(StringPart, ") {\n        var cache = {};\n\n        fn original("),
        Token(InterpolationStart),
        Token(Ident, "param_list"),
        Token(InterpolationEnd),
        Token(StringPart, ") "),
        Token(InterpolationStart),
        Token(Ident, "func"),
        Token(Dot),
        Token(Ident, "body"),
        Token(InterpolationEnd),
        Token(StringPart, "\n\n        fn cached("),
        Token(InterpolationStart),
        Token(Ident, "param_list"),
        Token(InterpolationEnd),
        Token(StringPart, ") {\n            var args_key = str("),
        Token(InterpolationStart),
        Token(Ident, "param_list"),
        Token(InterpolationEnd),
        Token(StringPart, ");\n            if args_key in cache {\n                cache[args_key]\n            } else {\n                var result = original("),
        Token(InterpolationStart),
        Token(Ident, "param_list"),
        Token(InterpolationEnd),
        Token(StringPart, ");\n                cache[args_key] = result;\n                result\n            }\n        }\n\n        cached("),
        Token(InterpolationStart),
        Token(Ident, "param_list"),
        Token(InterpolationEnd),
        Token(StringPart, ")\n    }"),
        Token(RightParen),
        Token(RightBrace),
        Token(Var),
        Token(Ident, "count"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(Hash),
        Token(LeftBracket),
        Token(Ident, "memoize"),
        Token(RightBracket),
        Token(Fn),
        Token(Ident, "fibonacci"),
        Token(LeftParen),
        Token(Ident, "n"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "count"),
        Token(PlusEqual),
        Token(Int, "1"),
        Token(Semicolon),
        Token(If),
        Token(Ident, "n"),
        Token(LessEqual),
        Token(Int, "1"),
        Token(LeftBrace),
        Token(Ident, "n"),
        Token(RightBrace),
        Token(Else),
        Token(LeftBrace),
        Token(Ident, "fibonacci"),
        Token(LeftParen),
        Token(Ident, "n"),
        Token(Minus),
        Token(Int, "1"),
        Token(RightParen),
        Token(Plus),
        Token(Ident, "fibonacci"),
        Token(LeftParen),
        Token(Ident, "n"),
        Token(Minus),
        Token(Int, "2"),
        Token(RightParen),
        Token(RightBrace),
        Token(RightBrace),
        Token(Var),
        Token(Ident, "f"),
        Token(Equal),
        Token(Ident, "fibonacci"),
        Token(LeftParen),
        Token(Int, "10"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "f"),
        Token(EqualEqual),
        Token(Int, "55"),
        Token(Comma),
        Token(String, "f"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "count"),
        Token(EqualEqual),
        Token(Int, "11"),
        Token(Comma),
        Token(String, "count"),
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
            Macro {
                name: "macro",
                args: [],
                statement: FnDecl {
                    name: "memoize",
                    params: [
                        Parameter {
                            name: "eval_node",
                            default: None,
                            kind: Regular,
                        },
                    ],
                    body: Block {
                        statements: [
                            Expression(
                                If {
                                    condition: IsNot(
                                        Identifier(
                                            "eval_node",
                                        ),
                                        GetAttr(
                                            Identifier(
                                                "Eval",
                                            ),
                                            "Function",
                                        ),
                                    ),
                                    then_expr: Block {
                                        statements: [
                                            Expression(
                                                FunctionCall(
                                                    Identifier(
                                                        "raise",
                                                    ),
                                                    [
                                                        Literal(
                                                            String(
                                                                "#[memoize] can only be applied to functions",
                                                            ),
                                                        ),
                                                    ],
                                                ),
                                            ),
                                        ],
                                        final_expr: None,
                                    },
                                    else_expr: None,
                                },
                            ),
                            VarDecl {
                                pattern: Variable(
                                    "func",
                                ),
                                value: Some(
                                    Identifier(
                                        "eval_node",
                                    ),
                                ),
                            },
                            VarDecl {
                                pattern: Variable(
                                    "param_list",
                                ),
                                value: Some(
                                    FunctionCall(
                                        Identifier(
                                            "join",
                                        ),
                                        [
                                            GetAttr(
                                                Identifier(
                                                    "func",
                                                ),
                                                "params",
                                            ),
                                            Literal(
                                                String(
                                                    ", ",
                                                ),
                                            ),
                                        ],
                                    ),
                                ),
                            },
                        ],
                        final_expr: Some(
                            FunctionCall(
                                Identifier(
                                    "parse",
                                ),
                                [
                                    InterpolatedString(
                                        [
                                            Text(
                                                "fn ",
                                            ),
                                            Expression(
                                                GetAttr(
                                                    Identifier(
                                                        "func",
                                                    ),
                                                    "name",
                                                ),
                                            ),
                                            Text(
                                                "(",
                                            ),
                                            Expression(
                                                Identifier(
                                                    "param_list",
                                                ),
                                            ),
                                            Text(
                                                ") {\n        var cache = {};\n\n        fn original(",
                                            ),
                                            Expression(
                                                Identifier(
                                                    "param_list",
                                                ),
                                            ),
                                            Text(
                                                ") ",
                                            ),
                                            Expression(
                                                GetAttr(
                                                    Identifier(
                                                        "func",
                                                    ),
                                                    "body",
                                                ),
                                            ),
                                            Text(
                                                "\n\n        fn cached(",
                                            ),
                                            Expression(
                                                Identifier(
                                                    "param_list",
                                                ),
                                            ),
                                            Text(
                                                ") {\n            var args_key = str(",
                                            ),
                                            Expression(
                                                Identifier(
                                                    "param_list",
                                                ),
                                            ),
                                            Text(
                                                ");\n            if args_key in cache {\n                cache[args_key]\n            } else {\n                var result = original(",
                                            ),
                                            Expression(
                                                Identifier(
                                                    "param_list",
                                                ),
                                            ),
                                            Text(
                                                ");\n                cache[args_key] = result;\n                result\n            }\n        }\n\n        cached(",
                                            ),
                                            Expression(
                                                Identifier(
                                                    "param_list",
                                                ),
                                            ),
                                            Text(
                                                ")\n    }",
                                            ),
                                        ],
                                    ),
                                ],
                            ),
                        ),
                    },
                    is_pub: false,
                },
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
            Macro {
                name: "memoize",
                args: [],
                statement: FnDecl {
                    name: "fibonacci",
                    params: [
                        Parameter {
                            name: "n",
                            default: None,
                            kind: Regular,
                        },
                    ],
                    body: Block {
                        statements: [
                            Assignment {
                                target: Identifier(
                                    "count",
                                ),
                                op: AddAssign,
                                value: Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                            },
                        ],
                        final_expr: Some(
                            If {
                                condition: Le(
                                    Identifier(
                                        "n",
                                    ),
                                    Literal(
                                        Int(
                                            1,
                                        ),
                                    ),
                                ),
                                then_expr: Block {
                                    statements: [],
                                    final_expr: Some(
                                        Identifier(
                                            "n",
                                        ),
                                    ),
                                },
                                else_expr: Some(
                                    Block {
                                        statements: [],
                                        final_expr: Some(
                                            Add(
                                                FunctionCall(
                                                    Identifier(
                                                        "fibonacci",
                                                    ),
                                                    [
                                                        Sub(
                                                            Identifier(
                                                                "n",
                                                            ),
                                                            Literal(
                                                                Int(
                                                                    1,
                                                                ),
                                                            ),
                                                        ),
                                                    ],
                                                ),
                                                FunctionCall(
                                                    Identifier(
                                                        "fibonacci",
                                                    ),
                                                    [
                                                        Sub(
                                                            Identifier(
                                                                "n",
                                                            ),
                                                            Literal(
                                                                Int(
                                                                    2,
                                                                ),
                                                            ),
                                                        ),
                                                    ],
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                    is_pub: false,
                },
            },
            VarDecl {
                pattern: Variable(
                    "f",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "fibonacci",
                        ),
                        [
                            Literal(
                                Int(
                                    10,
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
                                "f",
                            ),
                            Literal(
                                Int(
                                    55,
                                ),
                            ),
                        ),
                        Literal(
                            String(
                                "f",
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
                                    11,
                                ),
                            ),
                        ),
                        Literal(
                            String(
                                "count",
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
                        EvalMacro {
                            data: MacroData {
                                macro_fn: Eval(
                                    EvalRef(
                                        EvalVariable {
                                            name: "macro",
                                        },
                                    ),
                                ),
                                target: Eval(
                                    EvalRef(
                                        EvalFunction {
                                            data: FunctionData {
                                                name: "memoize",
                                                params: [
                                                    (
                                                        "eval_node",
                                                        None,
                                                        Regular,
                                                    ),
                                                ],
                                                body: Eval(
                                                    EvalRef(
                                                        EvalBlock {
                                                            statements: [
                                                                EvalRef(
                                                                    EvalIf {
                                                                        condition: EvalRef(
                                                                            EvalLogicalNot {
                                                                                expr: EvalRef(
                                                                                    EvalIs {
                                                                                        left: EvalRef(
                                                                                            EvalVariable {
                                                                                                name: "eval_node",
                                                                                            },
                                                                                        ),
                                                                                        right: EvalRef(
                                                                                            EvalGetAttr {
                                                                                                obj_expr: EvalRef(
                                                                                                    EvalVariable {
                                                                                                        name: "Eval",
                                                                                                    },
                                                                                                ),
                                                                                                attr_name: "Function",
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                ),
                                                                            },
                                                                        ),
                                                                        then_expr: EvalRef(
                                                                            EvalBlock {
                                                                                statements: [
                                                                                    EvalRef(
                                                                                        EvalCall {
                                                                                            func_expr: EvalRef(
                                                                                                EvalVariable {
                                                                                                    name: "raise",
                                                                                                },
                                                                                            ),
                                                                                            args: [
                                                                                                EvalRef(
                                                                                                    EvalLiteral {
                                                                                                        value: String(
                                                                                                            "#[memoize] can only be applied to functions",
                                                                                                        ),
                                                                                                    },
                                                                                                ),
                                                                                            ],
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                final_expr: None,
                                                                            },
                                                                        ),
                                                                        else_expr: None,
                                                                    },
                                                                ),
                                                                EvalRef(
                                                                    EvalDeclare {
                                                                        name: "func",
                                                                        init_expr: Some(
                                                                            EvalRef(
                                                                                EvalVariable {
                                                                                    name: "eval_node",
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                                EvalRef(
                                                                    EvalDeclare {
                                                                        name: "param_list",
                                                                        init_expr: Some(
                                                                            EvalRef(
                                                                                EvalCall {
                                                                                    func_expr: EvalRef(
                                                                                        EvalVariable {
                                                                                            name: "join",
                                                                                        },
                                                                                    ),
                                                                                    args: [
                                                                                        EvalRef(
                                                                                            EvalGetAttr {
                                                                                                obj_expr: EvalRef(
                                                                                                    EvalVariable {
                                                                                                        name: "func",
                                                                                                    },
                                                                                                ),
                                                                                                attr_name: "params",
                                                                                            },
                                                                                        ),
                                                                                        EvalRef(
                                                                                            EvalLiteral {
                                                                                                value: String(
                                                                                                    ", ",
                                                                                                ),
                                                                                            },
                                                                                        ),
                                                                                    ],
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                            ],
                                                            final_expr: Some(
                                                                EvalRef(
                                                                    EvalCall {
                                                                        func_expr: EvalRef(
                                                                            EvalVariable {
                                                                                name: "parse",
                                                                            },
                                                                        ),
                                                                        args: [
                                                                            EvalRef(
                                                                                EvalCall {
                                                                                    func_expr: EvalRef(
                                                                                        EvalGetAttr {
                                                                                            obj_expr: EvalRef(
                                                                                                EvalLiteral {
                                                                                                    value: String(
                                                                                                        "fn ",
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
                                                                                                                    name: "func",
                                                                                                                },
                                                                                                            ),
                                                                                                            attr_name: "name",
                                                                                                        },
                                                                                                    ),
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                        EvalRef(
                                                                                            EvalLiteral {
                                                                                                value: String(
                                                                                                    "(",
                                                                                                ),
                                                                                            },
                                                                                        ),
                                                                                        EvalRef(
                                                                                            EvalCall {
                                                                                                func_expr: EvalRef(
                                                                                                    EvalVariable {
                                                                                                        name: "str",
                                                                                                    },
                                                                                                ),
                                                                                                args: [
                                                                                                    EvalRef(
                                                                                                        EvalVariable {
                                                                                                            name: "param_list",
                                                                                                        },
                                                                                                    ),
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                        EvalRef(
                                                                                            EvalLiteral {
                                                                                                value: String(
                                                                                                    ") {\n        var cache = {};\n\n        fn original(",
                                                                                                ),
                                                                                            },
                                                                                        ),
                                                                                        EvalRef(
                                                                                            EvalCall {
                                                                                                func_expr: EvalRef(
                                                                                                    EvalVariable {
                                                                                                        name: "str",
                                                                                                    },
                                                                                                ),
                                                                                                args: [
                                                                                                    EvalRef(
                                                                                                        EvalVariable {
                                                                                                            name: "param_list",
                                                                                                        },
                                                                                                    ),
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                        EvalRef(
                                                                                            EvalLiteral {
                                                                                                value: String(
                                                                                                    ") ",
                                                                                                ),
                                                                                            },
                                                                                        ),
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
                                                                                                                    name: "func",
                                                                                                                },
                                                                                                            ),
                                                                                                            attr_name: "body",
                                                                                                        },
                                                                                                    ),
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                        EvalRef(
                                                                                            EvalLiteral {
                                                                                                value: String(
                                                                                                    "\n\n        fn cached(",
                                                                                                ),
                                                                                            },
                                                                                        ),
                                                                                        EvalRef(
                                                                                            EvalCall {
                                                                                                func_expr: EvalRef(
                                                                                                    EvalVariable {
                                                                                                        name: "str",
                                                                                                    },
                                                                                                ),
                                                                                                args: [
                                                                                                    EvalRef(
                                                                                                        EvalVariable {
                                                                                                            name: "param_list",
                                                                                                        },
                                                                                                    ),
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                        EvalRef(
                                                                                            EvalLiteral {
                                                                                                value: String(
                                                                                                    ") {\n            var args_key = str(",
                                                                                                ),
                                                                                            },
                                                                                        ),
                                                                                        EvalRef(
                                                                                            EvalCall {
                                                                                                func_expr: EvalRef(
                                                                                                    EvalVariable {
                                                                                                        name: "str",
                                                                                                    },
                                                                                                ),
                                                                                                args: [
                                                                                                    EvalRef(
                                                                                                        EvalVariable {
                                                                                                            name: "param_list",
                                                                                                        },
                                                                                                    ),
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                        EvalRef(
                                                                                            EvalLiteral {
                                                                                                value: String(
                                                                                                    ");\n            if args_key in cache {\n                cache[args_key]\n            } else {\n                var result = original(",
                                                                                                ),
                                                                                            },
                                                                                        ),
                                                                                        EvalRef(
                                                                                            EvalCall {
                                                                                                func_expr: EvalRef(
                                                                                                    EvalVariable {
                                                                                                        name: "str",
                                                                                                    },
                                                                                                ),
                                                                                                args: [
                                                                                                    EvalRef(
                                                                                                        EvalVariable {
                                                                                                            name: "param_list",
                                                                                                        },
                                                                                                    ),
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                        EvalRef(
                                                                                            EvalLiteral {
                                                                                                value: String(
                                                                                                    ");\n                cache[args_key] = result;\n                result\n            }\n        }\n\n        cached(",
                                                                                                ),
                                                                                            },
                                                                                        ),
                                                                                        EvalRef(
                                                                                            EvalCall {
                                                                                                func_expr: EvalRef(
                                                                                                    EvalVariable {
                                                                                                        name: "str",
                                                                                                    },
                                                                                                ),
                                                                                                args: [
                                                                                                    EvalRef(
                                                                                                        EvalVariable {
                                                                                                            name: "param_list",
                                                                                                        },
                                                                                                    ),
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                        EvalRef(
                                                                                            EvalLiteral {
                                                                                                value: String(
                                                                                                    ")\n    }",
                                                                                                ),
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
                                            },
                                        },
                                    ),
                                ),
                                args: [],
                            },
                        },
                    ),
                    EvalRef(
                        EvalDeclare {
                            name: "count",
                            init_expr: Some(
                                EvalRef(
                                    EvalLiteral {
                                        value: Int(
                                            0,
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                    EvalRef(
                        EvalMacro {
                            data: MacroData {
                                macro_fn: Eval(
                                    EvalRef(
                                        EvalVariable {
                                            name: "memoize",
                                        },
                                    ),
                                ),
                                target: Eval(
                                    EvalRef(
                                        EvalFunction {
                                            data: FunctionData {
                                                name: "fibonacci",
                                                params: [
                                                    (
                                                        "n",
                                                        None,
                                                        Regular,
                                                    ),
                                                ],
                                                body: Eval(
                                                    EvalRef(
                                                        EvalBlock {
                                                            statements: [
                                                                EvalRef(
                                                                    EvalAssign {
                                                                        name: "count",
                                                                        expr: EvalRef(
                                                                            EvalCall {
                                                                                func_expr: EvalRef(
                                                                                    EvalGetAttr {
                                                                                        obj_expr: EvalRef(
                                                                                            EvalVariable {
                                                                                                name: "count",
                                                                                            },
                                                                                        ),
                                                                                        attr_name: "op_add",
                                                                                    },
                                                                                ),
                                                                                args: [
                                                                                    EvalRef(
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
                                                            final_expr: Some(
                                                                EvalRef(
                                                                    EvalIf {
                                                                        condition: EvalRef(
                                                                            EvalCall {
                                                                                func_expr: EvalRef(
                                                                                    EvalGetAttr {
                                                                                        obj_expr: EvalRef(
                                                                                            EvalVariable {
                                                                                                name: "n",
                                                                                            },
                                                                                        ),
                                                                                        attr_name: "op_le",
                                                                                    },
                                                                                ),
                                                                                args: [
                                                                                    EvalRef(
                                                                                        EvalLiteral {
                                                                                            value: Int(
                                                                                                1,
                                                                                            ),
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                            },
                                                                        ),
                                                                        then_expr: EvalRef(
                                                                            EvalBlock {
                                                                                statements: [],
                                                                                final_expr: Some(
                                                                                    EvalRef(
                                                                                        EvalVariable {
                                                                                            name: "n",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                        else_expr: Some(
                                                                            EvalRef(
                                                                                EvalBlock {
                                                                                    statements: [],
                                                                                    final_expr: Some(
                                                                                        EvalRef(
                                                                                            EvalCall {
                                                                                                func_expr: EvalRef(
                                                                                                    EvalGetAttr {
                                                                                                        obj_expr: EvalRef(
                                                                                                            EvalCall {
                                                                                                                func_expr: EvalRef(
                                                                                                                    EvalVariable {
                                                                                                                        name: "fibonacci",
                                                                                                                    },
                                                                                                                ),
                                                                                                                args: [
                                                                                                                    EvalRef(
                                                                                                                        EvalCall {
                                                                                                                            func_expr: EvalRef(
                                                                                                                                EvalGetAttr {
                                                                                                                                    obj_expr: EvalRef(
                                                                                                                                        EvalVariable {
                                                                                                                                            name: "n",
                                                                                                                                        },
                                                                                                                                    ),
                                                                                                                                    attr_name: "op_sub",
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            args: [
                                                                                                                                EvalRef(
                                                                                                                                    EvalLiteral {
                                                                                                                                        value: Int(
                                                                                                                                            1,
                                                                                                                                        ),
                                                                                                                                    },
                                                                                                                                ),
                                                                                                                            ],
                                                                                                                        },
                                                                                                                    ),
                                                                                                                ],
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
                                                                                                                    name: "fibonacci",
                                                                                                                },
                                                                                                            ),
                                                                                                            args: [
                                                                                                                EvalRef(
                                                                                                                    EvalCall {
                                                                                                                        func_expr: EvalRef(
                                                                                                                            EvalGetAttr {
                                                                                                                                obj_expr: EvalRef(
                                                                                                                                    EvalVariable {
                                                                                                                                        name: "n",
                                                                                                                                    },
                                                                                                                                ),
                                                                                                                                attr_name: "op_sub",
                                                                                                                            },
                                                                                                                        ),
                                                                                                                        args: [
                                                                                                                            EvalRef(
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
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            },
                                        },
                                    ),
                                ),
                                args: [],
                            },
                        },
                    ),
                    EvalRef(
                        EvalDeclare {
                            name: "f",
                            init_expr: Some(
                                EvalRef(
                                    EvalCall {
                                        func_expr: EvalRef(
                                            EvalVariable {
                                                name: "fibonacci",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
                                                EvalLiteral {
                                                    value: Int(
                                                        10,
                                                    ),
                                                },
                                            ),
                                        ],
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
                                                        name: "f",
                                                    },
                                                ),
                                                attr_name: "op_eq",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
                                                EvalLiteral {
                                                    value: Int(
                                                        55,
                                                    ),
                                                },
                                            ),
                                        ],
                                    },
                                ),
                                EvalRef(
                                    EvalLiteral {
                                        value: String(
                                            "f",
                                        ),
                                    },
                                ),
                            ],
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
                                                        name: "count",
                                                    },
                                                ),
                                                attr_name: "op_eq",
                                            },
                                        ),
                                        args: [
                                            EvalRef(
                                                EvalLiteral {
                                                    value: Int(
                                                        11,
                                                    ),
                                                },
                                            ),
                                        ],
                                    },
                                ),
                                EvalRef(
                                    EvalLiteral {
                                        value: String(
                                            "count",
                                        ),
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