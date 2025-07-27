# Program
Status: 🟢
Assertions: 9

```rustleaf
// Test 'in' operator (op_contains)

// String contains
assert("hello" in "hello world");
assert(not ("xyz" in "hello world"));

// List contains
var my_list = [1, 2, 3, "hello"];
assert(2 in my_list);
assert("hello" in my_list);
assert(not (99 in my_list));

// Dict contains (check keys)
var my_dict = {"a": 1, "b": 2, 3: "three"};
assert("a" in my_dict);
assert("b" in my_dict);
assert(3 in my_dict);
assert(not ("z" in my_dict));
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
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(String, "hello"),
        Token(In),
        Token(String, "hello world"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(LeftParen),
        Token(String, "xyz"),
        Token(In),
        Token(String, "hello world"),
        Token(RightParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "my_list"),
        Token(Equal),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(Comma),
        Token(String, "hello"),
        Token(RightBracket),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "2"),
        Token(In),
        Token(Ident, "my_list"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(String, "hello"),
        Token(In),
        Token(Ident, "my_list"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(LeftParen),
        Token(Int, "99"),
        Token(In),
        Token(Ident, "my_list"),
        Token(RightParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "my_dict"),
        Token(Equal),
        Token(LeftBrace),
        Token(String, "a"),
        Token(Colon),
        Token(Int, "1"),
        Token(Comma),
        Token(String, "b"),
        Token(Colon),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(Colon),
        Token(String, "three"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(String, "a"),
        Token(In),
        Token(Ident, "my_dict"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(String, "b"),
        Token(In),
        Token(Ident, "my_dict"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "3"),
        Token(In),
        Token(Ident, "my_dict"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(LeftParen),
        Token(String, "z"),
        Token(In),
        Token(Ident, "my_dict"),
        Token(RightParen),
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        In(
                            Literal(
                                String(
                                    "hello",
                                ),
                            ),
                            Literal(
                                String(
                                    "hello world",
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
                        Not(
                            In(
                                Literal(
                                    String(
                                        "xyz",
                                    ),
                                ),
                                Literal(
                                    String(
                                        "hello world",
                                    ),
                                ),
                            ),
                        ),
                    ],
                ),
            ),
            VarDecl {
                pattern: Variable(
                    "my_list",
                ),
                value: Some(
                    List(
                        [
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                            Literal(
                                Int(
                                    2,
                                ),
                            ),
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                            Literal(
                                String(
                                    "hello",
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
                        In(
                            Literal(
                                Int(
                                    2,
                                ),
                            ),
                            Identifier(
                                "my_list",
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
                        In(
                            Literal(
                                String(
                                    "hello",
                                ),
                            ),
                            Identifier(
                                "my_list",
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
                        Not(
                            In(
                                Literal(
                                    Int(
                                        99,
                                    ),
                                ),
                                Identifier(
                                    "my_list",
                                ),
                            ),
                        ),
                    ],
                ),
            ),
            VarDecl {
                pattern: Variable(
                    "my_dict",
                ),
                value: Some(
                    Dict(
                        [
                            (
                                Literal(
                                    String(
                                        "a",
                                    ),
                                ),
                                Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                            ),
                            (
                                Literal(
                                    String(
                                        "b",
                                    ),
                                ),
                                Literal(
                                    Int(
                                        2,
                                    ),
                                ),
                            ),
                            (
                                Literal(
                                    Int(
                                        3,
                                    ),
                                ),
                                Literal(
                                    String(
                                        "three",
                                    ),
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
                        In(
                            Literal(
                                String(
                                    "a",
                                ),
                            ),
                            Identifier(
                                "my_dict",
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
                        In(
                            Literal(
                                String(
                                    "b",
                                ),
                            ),
                            Identifier(
                                "my_dict",
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
                        In(
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                            Identifier(
                                "my_dict",
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
                        Not(
                            In(
                                Literal(
                                    String(
                                        "z",
                                    ),
                                ),
                                Identifier(
                                    "my_dict",
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
    Program(
        [
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Literal(
                                String(
                                    "hello world",
                                ),
                            ),
                            "op_contains",
                        ),
                        [
                            Literal(
                                String(
                                    "hello",
                                ),
                            ),
                        ],
                    ),
                ],
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    LogicalNot(
                        Call(
                            GetAttr(
                                Literal(
                                    String(
                                        "hello world",
                                    ),
                                ),
                                "op_contains",
                            ),
                            [
                                Literal(
                                    String(
                                        "xyz",
                                    ),
                                ),
                            ],
                        ),
                    ),
                ],
            ),
            Declare(
                "my_list",
                Some(
                    List(
                        [
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                            Literal(
                                Int(
                                    2,
                                ),
                            ),
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                            Literal(
                                String(
                                    "hello",
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
                                "my_list",
                            ),
                            "op_contains",
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
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Variable(
                                "my_list",
                            ),
                            "op_contains",
                        ),
                        [
                            Literal(
                                String(
                                    "hello",
                                ),
                            ),
                        ],
                    ),
                ],
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    LogicalNot(
                        Call(
                            GetAttr(
                                Variable(
                                    "my_list",
                                ),
                                "op_contains",
                            ),
                            [
                                Literal(
                                    Int(
                                        99,
                                    ),
                                ),
                            ],
                        ),
                    ),
                ],
            ),
            Declare(
                "my_dict",
                Some(
                    Dict(
                        [
                            (
                                Literal(
                                    String(
                                        "a",
                                    ),
                                ),
                                Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                            ),
                            (
                                Literal(
                                    String(
                                        "b",
                                    ),
                                ),
                                Literal(
                                    Int(
                                        2,
                                    ),
                                ),
                            ),
                            (
                                Literal(
                                    Int(
                                        3,
                                    ),
                                ),
                                Literal(
                                    String(
                                        "three",
                                    ),
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
                                "my_dict",
                            ),
                            "op_contains",
                        ),
                        [
                            Literal(
                                String(
                                    "a",
                                ),
                            ),
                        ],
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
                                "my_dict",
                            ),
                            "op_contains",
                        ),
                        [
                            Literal(
                                String(
                                    "b",
                                ),
                            ),
                        ],
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
                                "my_dict",
                            ),
                            "op_contains",
                        ),
                        [
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                        ],
                    ),
                ],
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    LogicalNot(
                        Call(
                            GetAttr(
                                Variable(
                                    "my_dict",
                                ),
                                "op_contains",
                            ),
                            [
                                Literal(
                                    String(
                                        "z",
                                    ),
                                ),
                            ],
                        ),
                    ),
                ],
            ),
        ],
    ),
)
```