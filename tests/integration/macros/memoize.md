# Program
Status: 🔴
Assertions: 2

```rustleaf
#[macro]
fn memoize(eval_node) {
    if eval_node is not Eval.Function {
        raise("#[memoize] can only be applied to functions");
    }
    var func = eval_node;

    var param_list = join(func.params, ", ");

    parse("fn ${func.name}(${param_list}) {
        var cache = {};

        fn original(${param_list}) ${func.body}

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
```
177
```

# Result
```rust
Err(
    "Assertion failed: count",
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
    Program(
        [
            Macro(
                MacroData {
                    macro_fn: Variable(
                        "macro",
                    ),
                    target: Function(
                        FunctionData {
                            name: "memoize",
                            params: [
                                (
                                    "eval_node",
                                    None,
                                    Regular,
                                ),
                            ],
                            body: Block(
                                [
                                    If(
                                        LogicalNot(
                                            Is(
                                                Variable(
                                                    "eval_node",
                                                ),
                                                GetAttr(
                                                    Variable(
                                                        "Eval",
                                                    ),
                                                    "Function",
                                                ),
                                            ),
                                        ),
                                        Block(
                                            [
                                                Call(
                                                    Variable(
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
                                            ],
                                            None,
                                        ),
                                        None,
                                    ),
                                    Declare(
                                        "func",
                                        Some(
                                            Variable(
                                                "eval_node",
                                            ),
                                        ),
                                    ),
                                    Declare(
                                        "param_list",
                                        Some(
                                            Call(
                                                Variable(
                                                    "join",
                                                ),
                                                [
                                                    GetAttr(
                                                        Variable(
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
                                    ),
                                ],
                                Some(
                                    Call(
                                        Variable(
                                            "parse",
                                        ),
                                        [
                                            Call(
                                                GetAttr(
                                                    Literal(
                                                        String(
                                                            "fn ",
                                                        ),
                                                    ),
                                                    "op_add",
                                                ),
                                                [
                                                    Call(
                                                        Variable(
                                                            "str",
                                                        ),
                                                        [
                                                            GetAttr(
                                                                Variable(
                                                                    "func",
                                                                ),
                                                                "name",
                                                            ),
                                                        ],
                                                    ),
                                                    Literal(
                                                        String(
                                                            "(",
                                                        ),
                                                    ),
                                                    Call(
                                                        Variable(
                                                            "str",
                                                        ),
                                                        [
                                                            Variable(
                                                                "param_list",
                                                            ),
                                                        ],
                                                    ),
                                                    Literal(
                                                        String(
                                                            ") {\n        var cache = {};\n\n        fn original(",
                                                        ),
                                                    ),
                                                    Call(
                                                        Variable(
                                                            "str",
                                                        ),
                                                        [
                                                            Variable(
                                                                "param_list",
                                                            ),
                                                        ],
                                                    ),
                                                    Literal(
                                                        String(
                                                            ") ",
                                                        ),
                                                    ),
                                                    Call(
                                                        Variable(
                                                            "str",
                                                        ),
                                                        [
                                                            GetAttr(
                                                                Variable(
                                                                    "func",
                                                                ),
                                                                "body",
                                                            ),
                                                        ],
                                                    ),
                                                    Literal(
                                                        String(
                                                            "\n\n        fn cached(",
                                                        ),
                                                    ),
                                                    Call(
                                                        Variable(
                                                            "str",
                                                        ),
                                                        [
                                                            Variable(
                                                                "param_list",
                                                            ),
                                                        ],
                                                    ),
                                                    Literal(
                                                        String(
                                                            ") {\n            var args_key = str(",
                                                        ),
                                                    ),
                                                    Call(
                                                        Variable(
                                                            "str",
                                                        ),
                                                        [
                                                            Variable(
                                                                "param_list",
                                                            ),
                                                        ],
                                                    ),
                                                    Literal(
                                                        String(
                                                            ");\n            if args_key in cache {\n                cache[args_key]\n            } else {\n                var result = original(",
                                                        ),
                                                    ),
                                                    Call(
                                                        Variable(
                                                            "str",
                                                        ),
                                                        [
                                                            Variable(
                                                                "param_list",
                                                            ),
                                                        ],
                                                    ),
                                                    Literal(
                                                        String(
                                                            ");\n                cache[args_key] = result;\n                result\n            }\n        }\n\n        cached(",
                                                        ),
                                                    ),
                                                    Call(
                                                        Variable(
                                                            "str",
                                                        ),
                                                        [
                                                            Variable(
                                                                "param_list",
                                                            ),
                                                        ],
                                                    ),
                                                    Literal(
                                                        String(
                                                            ")\n    }",
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ],
                                    ),
                                ),
                            ),
                        },
                    ),
                    args: [],
                },
            ),
            Declare(
                "count",
                Some(
                    Literal(
                        Int(
                            0,
                        ),
                    ),
                ),
            ),
            Macro(
                MacroData {
                    macro_fn: Variable(
                        "memoize",
                    ),
                    target: Function(
                        FunctionData {
                            name: "fibonacci",
                            params: [
                                (
                                    "n",
                                    None,
                                    Regular,
                                ),
                            ],
                            body: Block(
                                [
                                    Assign(
                                        "count",
                                        Call(
                                            GetAttr(
                                                Variable(
                                                    "count",
                                                ),
                                                "op_add",
                                            ),
                                            [
                                                Literal(
                                                    Int(
                                                        1,
                                                    ),
                                                ),
                                            ],
                                        ),
                                    ),
                                ],
                                Some(
                                    If(
                                        Call(
                                            GetAttr(
                                                Variable(
                                                    "n",
                                                ),
                                                "op_le",
                                            ),
                                            [
                                                Literal(
                                                    Int(
                                                        1,
                                                    ),
                                                ),
                                            ],
                                        ),
                                        Block(
                                            [],
                                            Some(
                                                Variable(
                                                    "n",
                                                ),
                                            ),
                                        ),
                                        Some(
                                            Block(
                                                [],
                                                Some(
                                                    Call(
                                                        GetAttr(
                                                            Call(
                                                                Variable(
                                                                    "fibonacci",
                                                                ),
                                                                [
                                                                    Call(
                                                                        GetAttr(
                                                                            Variable(
                                                                                "n",
                                                                            ),
                                                                            "op_sub",
                                                                        ),
                                                                        [
                                                                            Literal(
                                                                                Int(
                                                                                    1,
                                                                                ),
                                                                            ),
                                                                        ],
                                                                    ),
                                                                ],
                                                            ),
                                                            "op_add",
                                                        ),
                                                        [
                                                            Call(
                                                                Variable(
                                                                    "fibonacci",
                                                                ),
                                                                [
                                                                    Call(
                                                                        GetAttr(
                                                                            Variable(
                                                                                "n",
                                                                            ),
                                                                            "op_sub",
                                                                        ),
                                                                        [
                                                                            Literal(
                                                                                Int(
                                                                                    2,
                                                                                ),
                                                                            ),
                                                                        ],
                                                                    ),
                                                                ],
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                    args: [],
                },
            ),
            Declare(
                "f",
                Some(
                    Call(
                        Variable(
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
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Variable(
                                "f",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    55,
                                ),
                            ),
                        ],
                    ),
                    Literal(
                        String(
                            "f",
                        ),
                    ),
                ],
            ),
            Call(
                Variable(
                    "print",
                ),
                [
                    Variable(
                        "count",
                    ),
                ],
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Variable(
                                "count",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    11,
                                ),
                            ),
                        ],
                    ),
                    Literal(
                        String(
                            "count",
                        ),
                    ),
                ],
            ),
        ],
    ),
)
```