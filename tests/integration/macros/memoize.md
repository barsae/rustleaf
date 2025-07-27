# Program
Status: 🔴
Assertions: 0

```rustleaf
// #[macro]
// fn memoize(eval_node) {
    // if eval_node is not Eval.Function {
        // raise("#[memoize] can only be applied to functions");
    // }
    // var func = eval_node;

    // var param_list = join(func.params, ", ");

    // parse("fn ${func.name}(${param_list}) {
        // var cache = {};

        // fn original(${param_list}) ${func.body}

        // fn cached(${param_list}) {
            // var args_key = str(${param_list});
            // if cache.has(args_key) {
                // cache[args_key]
            // } else {
                // var result = original(${param_list});
                // cache[args_key] = result;
                // result
            // }
        // }

        // cached(${param_list})
    // }")
// }

var count = 0;
fn fibonacci(n) {
    count += 1;
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

// var block = parse("fn f(x) { n + 1 }");
// print(block.statements[0]);

var f = fibonacci(10);
assert(f == 55, "f");
assert(count == 11, "count");
```

# Output
None

# Result
```rust
Err(
    "No attribute 'op_add' on value Unit",
)
```

# Lex
```rust
Ok(
    [
        Token(Var),
        Token(Ident, "count"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
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
            FnDecl {
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
    Block(
        [
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
            Function(
                "fibonacci",
                [
                    (
                        "n",
                        None,
                        Regular,
                    ),
                ],
                Block(
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
                    ],
                    None,
                ),
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
        ],
        Some(
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
        ),
    ),
)
```