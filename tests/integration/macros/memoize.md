# Program
Status: 🔴
Assertions: 0

```rustleaf
fn memoize(f) {
    var param_list = join(f.params, ", ");

    parse("var ${f.name} = {
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

        cached
    };")
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
print(count);
assert(count == 11, "count");
```

# Output
None

# Result
```rust
Err(
    "Failed to parse: Expected expression, found Var:\nvar fibonacci = {\n        var cache = {};\n\n        fn original(n) {\n    count = count.op_add(1);\n    if n.op_le(1) {\n    n\n} else {\n    fibonacci(n.op_sub(1)).op_add(fibonacci(n.op_sub(2)))\n};\n}\n\n        fn cached(n) {\n            var args_key = str(n);\n            if args_key in cache {\n                cache[args_key]\n            } else {\n                var result = original(n);\n                cache[args_key] = result;\n                result\n            }\n        }\n\n        cached\n    };",
)
```

# Lex
```rust
Ok(
    [
        Token(Fn),
        Token(Ident, "memoize"),
        Token(LeftParen),
        Token(Ident, "f"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "param_list"),
        Token(Equal),
        Token(Ident, "join"),
        Token(LeftParen),
        Token(Ident, "f"),
        Token(Dot),
        Token(Ident, "params"),
        Token(Comma),
        Token(String, ", "),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "parse"),
        Token(LeftParen),
        Token(StringPart, "var "),
        Token(InterpolationStart),
        Token(Ident, "f"),
        Token(Dot),
        Token(Ident, "name"),
        Token(InterpolationEnd),
        Token(StringPart, " = {\n        var cache = {};\n\n        fn original("),
        Token(InterpolationStart),
        Token(Ident, "param_list"),
        Token(InterpolationEnd),
        Token(StringPart, ") "),
        Token(InterpolationStart),
        Token(Ident, "f"),
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
        Token(StringPart, ");\n                cache[args_key] = result;\n                result\n            }\n        }\n\n        cached\n    };"),
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
        Token(Ident, "print"),
        Token(LeftParen),
        Token(Ident, "count"),
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
            FnDecl {
                name: "memoize",
                params: [
                    Parameter {
                        name: "f",
                        default: None,
                        kind: Regular,
                    },
                ],
                body: Block {
                    statements: [
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
                                                "f",
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
                                            "var ",
                                        ),
                                        Expression(
                                            GetAttr(
                                                Identifier(
                                                    "f",
                                                ),
                                                "name",
                                            ),
                                        ),
                                        Text(
                                            " = {\n        var cache = {};\n\n        fn original(",
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
                                                    "f",
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
                                            ");\n                cache[args_key] = result;\n                result\n            }\n        }\n\n        cached\n    };",
                                        ),
                                    ],
                                ),
                            ],
                        ),
                    ),
                },
                is_pub: false,
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
                            Expression(
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
                        ],
                        final_expr: None,
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
                        "print",
                    ),
                    [
                        Identifier(
                            "count",
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
    RustValue(
        EvalProgram {
            statements: [
                RustValue(
                    EvalFunction {
                        data: FunctionData {
                            name: "memoize",
                            params: [
                                (
                                    "f",
                                    None,
                                    Regular,
                                ),
                            ],
                            body: RustValue(
                                EvalBlock {
                                    statements: [
                                        RustValue(
                                            EvalDeclare {
                                                name: "param_list",
                                                init_expr: Some(
                                                    RustValue(
                                                        EvalCall {
                                                            func_expr: RustValue(
                                                                EvalVariable {
                                                                    name: "join",
                                                                },
                                                            ),
                                                            args: [
                                                                RustValue(
                                                                    EvalGetAttr {
                                                                        obj_expr: RustValue(
                                                                            EvalVariable {
                                                                                name: "f",
                                                                            },
                                                                        ),
                                                                        attr_name: "params",
                                                                    },
                                                                ),
                                                                RustValue(
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
                                        RustValue(
                                            EvalCall {
                                                func_expr: RustValue(
                                                    EvalVariable {
                                                        name: "parse",
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
                                                                                "var ",
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
                                                                                            name: "f",
                                                                                        },
                                                                                    ),
                                                                                    attr_name: "name",
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                ),
                                                                RustValue(
                                                                    EvalLiteral {
                                                                        value: String(
                                                                            " = {\n        var cache = {};\n\n        fn original(",
                                                                        ),
                                                                    },
                                                                ),
                                                                RustValue(
                                                                    EvalCall {
                                                                        func_expr: RustValue(
                                                                            EvalVariable {
                                                                                name: "str",
                                                                            },
                                                                        ),
                                                                        args: [
                                                                            RustValue(
                                                                                EvalVariable {
                                                                                    name: "param_list",
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                ),
                                                                RustValue(
                                                                    EvalLiteral {
                                                                        value: String(
                                                                            ") ",
                                                                        ),
                                                                    },
                                                                ),
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
                                                                                            name: "f",
                                                                                        },
                                                                                    ),
                                                                                    attr_name: "body",
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                ),
                                                                RustValue(
                                                                    EvalLiteral {
                                                                        value: String(
                                                                            "\n\n        fn cached(",
                                                                        ),
                                                                    },
                                                                ),
                                                                RustValue(
                                                                    EvalCall {
                                                                        func_expr: RustValue(
                                                                            EvalVariable {
                                                                                name: "str",
                                                                            },
                                                                        ),
                                                                        args: [
                                                                            RustValue(
                                                                                EvalVariable {
                                                                                    name: "param_list",
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                ),
                                                                RustValue(
                                                                    EvalLiteral {
                                                                        value: String(
                                                                            ") {\n            var args_key = str(",
                                                                        ),
                                                                    },
                                                                ),
                                                                RustValue(
                                                                    EvalCall {
                                                                        func_expr: RustValue(
                                                                            EvalVariable {
                                                                                name: "str",
                                                                            },
                                                                        ),
                                                                        args: [
                                                                            RustValue(
                                                                                EvalVariable {
                                                                                    name: "param_list",
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                ),
                                                                RustValue(
                                                                    EvalLiteral {
                                                                        value: String(
                                                                            ");\n            if args_key in cache {\n                cache[args_key]\n            } else {\n                var result = original(",
                                                                        ),
                                                                    },
                                                                ),
                                                                RustValue(
                                                                    EvalCall {
                                                                        func_expr: RustValue(
                                                                            EvalVariable {
                                                                                name: "str",
                                                                            },
                                                                        ),
                                                                        args: [
                                                                            RustValue(
                                                                                EvalVariable {
                                                                                    name: "param_list",
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                ),
                                                                RustValue(
                                                                    EvalLiteral {
                                                                        value: String(
                                                                            ");\n                cache[args_key] = result;\n                result\n            }\n        }\n\n        cached\n    };",
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
                        },
                    },
                ),
                RustValue(
                    EvalDeclare {
                        name: "count",
                        init_expr: Some(
                            RustValue(
                                EvalLiteral {
                                    value: Int(
                                        0,
                                    ),
                                },
                            ),
                        ),
                    },
                ),
                RustValue(
                    EvalMacro {
                        data: MacroData {
                            macro_fn: RustValue(
                                EvalVariable {
                                    name: "memoize",
                                },
                            ),
                            target: RustValue(
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
                                        body: RustValue(
                                            EvalBlock {
                                                statements: [
                                                    RustValue(
                                                        EvalAssign {
                                                            name: "count",
                                                            expr: RustValue(
                                                                EvalCall {
                                                                    func_expr: RustValue(
                                                                        EvalGetAttr {
                                                                            obj_expr: RustValue(
                                                                                EvalVariable {
                                                                                    name: "count",
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
                                                    RustValue(
                                                        EvalIf {
                                                            condition: RustValue(
                                                                EvalCall {
                                                                    func_expr: RustValue(
                                                                        EvalGetAttr {
                                                                            obj_expr: RustValue(
                                                                                EvalVariable {
                                                                                    name: "n",
                                                                                },
                                                                            ),
                                                                            attr_name: "op_le",
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
                                                            then_expr: RustValue(
                                                                EvalBlock {
                                                                    statements: [],
                                                                    final_expr: Some(
                                                                        RustValue(
                                                                            EvalVariable {
                                                                                name: "n",
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                            else_expr: Some(
                                                                RustValue(
                                                                    EvalBlock {
                                                                        statements: [],
                                                                        final_expr: Some(
                                                                            RustValue(
                                                                                EvalCall {
                                                                                    func_expr: RustValue(
                                                                                        EvalGetAttr {
                                                                                            obj_expr: RustValue(
                                                                                                EvalCall {
                                                                                                    func_expr: RustValue(
                                                                                                        EvalVariable {
                                                                                                            name: "fibonacci",
                                                                                                        },
                                                                                                    ),
                                                                                                    args: [
                                                                                                        RustValue(
                                                                                                            EvalCall {
                                                                                                                func_expr: RustValue(
                                                                                                                    EvalGetAttr {
                                                                                                                        obj_expr: RustValue(
                                                                                                                            EvalVariable {
                                                                                                                                name: "n",
                                                                                                                            },
                                                                                                                        ),
                                                                                                                        attr_name: "op_sub",
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
                                                                                                    ],
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
                                                                                                        name: "fibonacci",
                                                                                                    },
                                                                                                ),
                                                                                                args: [
                                                                                                    RustValue(
                                                                                                        EvalCall {
                                                                                                            func_expr: RustValue(
                                                                                                                EvalGetAttr {
                                                                                                                    obj_expr: RustValue(
                                                                                                                        EvalVariable {
                                                                                                                            name: "n",
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    attr_name: "op_sub",
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
                                                                                    ],
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ],
                                                final_expr: None,
                                            },
                                        ),
                                    },
                                },
                            ),
                            args: [],
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
                                            name: "fibonacci",
                                        },
                                    ),
                                    args: [
                                        RustValue(
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
                                                    name: "f",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    55,
                                                ),
                                            },
                                        ),
                                    ],
                                },
                            ),
                            RustValue(
                                EvalLiteral {
                                    value: String(
                                        "f",
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
                                name: "print",
                            },
                        ),
                        args: [
                            RustValue(
                                EvalVariable {
                                    name: "count",
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
                                                    name: "count",
                                                },
                                            ),
                                            attr_name: "op_eq",
                                        },
                                    ),
                                    args: [
                                        RustValue(
                                            EvalLiteral {
                                                value: Int(
                                                    11,
                                                ),
                                            },
                                        ),
                                    ],
                                },
                            ),
                            RustValue(
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
)
```