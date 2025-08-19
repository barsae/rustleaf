# Program
Status: 🟢
Assertions: 2

```rustleaf
fn memoize(f) {
    var param_list = join(f.params, ", ");

    parse("var ${f.name} = {
        var cache = {};

        fn original(${param_list}) ${f.body}

        fn cached(${param_list}) {
            var args_key = str(${param_list});
            if args_key in cache {
                return cache[args_key];
            } else {
                var result = original(${param_list});
                cache[args_key] = result;
                return result;
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
        return n;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

var f = fibonacci(10);
assert(f == 55, "f");
print(count);
assert(count == 11, "count");
```

# Output
```
11
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
        0: Token(Fn),
        1: Token(Ident, "memoize"),
        2: Token(LeftParen),
        3: Token(Ident, "f"),
        4: Token(RightParen),
        5: Token(LeftBrace),
        6: Token(Var),
        7: Token(Ident, "param_list"),
        8: Token(Equal),
        9: Token(Ident, "join"),
        10: Token(LeftParen),
        11: Token(Ident, "f"),
        12: Token(Dot),
        13: Token(Ident, "params"),
        14: Token(Comma),
        15: Token(String, ", "),
        16: Token(RightParen),
        17: Token(Semicolon),
        18: Token(Ident, "parse"),
        19: Token(LeftParen),
        20: Token(StringPart, "var "),
        21: Token(InterpolationStart),
        22: Token(Ident, "f"),
        23: Token(Dot),
        24: Token(Ident, "name"),
        25: Token(InterpolationEnd),
        26: Token(StringPart, " = {\n        var cache = {};\n\n        fn original("),
        27: Token(InterpolationStart),
        28: Token(Ident, "param_list"),
        29: Token(InterpolationEnd),
        30: Token(StringPart, ") "),
        31: Token(InterpolationStart),
        32: Token(Ident, "f"),
        33: Token(Dot),
        34: Token(Ident, "body"),
        35: Token(InterpolationEnd),
        36: Token(StringPart, "\n\n        fn cached("),
        37: Token(InterpolationStart),
        38: Token(Ident, "param_list"),
        39: Token(InterpolationEnd),
        40: Token(StringPart, ") {\n            var args_key = str("),
        41: Token(InterpolationStart),
        42: Token(Ident, "param_list"),
        43: Token(InterpolationEnd),
        44: Token(StringPart, ");\n            if args_key in cache {\n                return cache[args_key];\n            } else {\n                var result = original("),
        45: Token(InterpolationStart),
        46: Token(Ident, "param_list"),
        47: Token(InterpolationEnd),
        48: Token(StringPart, ");\n                cache[args_key] = result;\n                return result;\n            }\n        }\n\n        cached\n    };"),
        49: Token(RightParen),
        50: Token(RightBrace),
        51: Token(Var),
        52: Token(Ident, "count"),
        53: Token(Equal),
        54: Token(Int, "0"),
        55: Token(Semicolon),
        56: Token(Hash),
        57: Token(LeftBracket),
        58: Token(Ident, "memoize"),
        59: Token(RightBracket),
        60: Token(Fn),
        61: Token(Ident, "fibonacci"),
        62: Token(LeftParen),
        63: Token(Ident, "n"),
        64: Token(RightParen),
        65: Token(LeftBrace),
        66: Token(Ident, "count"),
        67: Token(PlusEqual),
        68: Token(Int, "1"),
        69: Token(Semicolon),
        70: Token(If),
        71: Token(Ident, "n"),
        72: Token(LessEqual),
        73: Token(Int, "1"),
        74: Token(LeftBrace),
        75: Token(Return),
        76: Token(Ident, "n"),
        77: Token(Semicolon),
        78: Token(RightBrace),
        79: Token(Else),
        80: Token(LeftBrace),
        81: Token(Return),
        82: Token(Ident, "fibonacci"),
        83: Token(LeftParen),
        84: Token(Ident, "n"),
        85: Token(Minus),
        86: Token(Int, "1"),
        87: Token(RightParen),
        88: Token(Plus),
        89: Token(Ident, "fibonacci"),
        90: Token(LeftParen),
        91: Token(Ident, "n"),
        92: Token(Minus),
        93: Token(Int, "2"),
        94: Token(RightParen),
        95: Token(Semicolon),
        96: Token(RightBrace),
        97: Token(RightBrace),
        98: Token(Var),
        99: Token(Ident, "f"),
        100: Token(Equal),
        101: Token(Ident, "fibonacci"),
        102: Token(LeftParen),
        103: Token(Int, "10"),
        104: Token(RightParen),
        105: Token(Semicolon),
        106: Token(Ident, "assert"),
        107: Token(LeftParen),
        108: Token(Ident, "f"),
        109: Token(EqualEqual),
        110: Token(Int, "55"),
        111: Token(Comma),
        112: Token(String, "f"),
        113: Token(RightParen),
        114: Token(Semicolon),
        115: Token(Ident, "print"),
        116: Token(LeftParen),
        117: Token(Ident, "count"),
        118: Token(RightParen),
        119: Token(Semicolon),
        120: Token(Ident, "assert"),
        121: Token(LeftParen),
        122: Token(Ident, "count"),
        123: Token(EqualEqual),
        124: Token(Int, "11"),
        125: Token(Comma),
        126: Token(String, "count"),
        127: Token(RightParen),
        128: Token(Semicolon),
        129: Token(Eof)
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
                                            ");\n            if args_key in cache {\n                return cache[args_key];\n            } else {\n                var result = original(",
                                        ),
                                        Expression(
                                            Identifier(
                                                "param_list",
                                            ),
                                        ),
                                        Text(
                                            ");\n                cache[args_key] = result;\n                return result;\n            }\n        }\n\n        cached\n    };",
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
                                        statements: [
                                            Return(
                                                Some(
                                                    Identifier(
                                                        "n",
                                                    ),
                                                ),
                                            ),
                                        ],
                                        final_expr: None,
                                    },
                                    else_expr: Some(
                                        Block {
                                            statements: [
                                                Return(
                                                    Some(
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
                                                ),
                                            ],
                                            final_expr: None,
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
    RustValue(<unknown>),
)
```