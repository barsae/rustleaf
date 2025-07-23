# Program 🟢
```rustleaf
// List indexing and operations
var my_list = [1, 2, 3];
assert(my_list[0] == 1);
assert(my_list[2] == 3);

// Dict access with different key types
var my_dict = {"a": 1, "b": 2};
assert(my_dict["a"] == 1);

// Mixed key types
var mixed = {1: "one", "two": 2, true: "yes"};
assert(mixed[1] == "one");
assert(mixed["two"] == 2);
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
        Token(Ident, "my_list"),
        Token(Equal),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightBracket),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "my_list"),
        Token(LeftBracket),
        Token(Int, "0"),
        Token(RightBracket),
        Token(EqualEqual),
        Token(Int, "1"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "my_list"),
        Token(LeftBracket),
        Token(Int, "2"),
        Token(RightBracket),
        Token(EqualEqual),
        Token(Int, "3"),
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
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "my_dict"),
        Token(LeftBracket),
        Token(String, "a"),
        Token(RightBracket),
        Token(EqualEqual),
        Token(Int, "1"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "mixed"),
        Token(Equal),
        Token(LeftBrace),
        Token(Int, "1"),
        Token(Colon),
        Token(String, "one"),
        Token(Comma),
        Token(String, "two"),
        Token(Colon),
        Token(Int, "2"),
        Token(Comma),
        Token(True),
        Token(Colon),
        Token(String, "yes"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "mixed"),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(RightBracket),
        Token(EqualEqual),
        Token(String, "one"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "mixed"),
        Token(LeftBracket),
        Token(String, "two"),
        Token(RightBracket),
        Token(EqualEqual),
        Token(Int, "2"),
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
                            GetItem(
                                Identifier(
                                    "my_list",
                                ),
                                Literal(
                                    Int(
                                        0,
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    1,
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
                            GetItem(
                                Identifier(
                                    "my_list",
                                ),
                                Literal(
                                    Int(
                                        2,
                                    ),
                                ),
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
                            GetItem(
                                Identifier(
                                    "my_dict",
                                ),
                                Literal(
                                    String(
                                        "a",
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                        ),
                    ],
                ),
            ),
            VarDecl {
                pattern: Variable(
                    "mixed",
                ),
                value: Some(
                    Dict(
                        [
                            (
                                Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                                Literal(
                                    String(
                                        "one",
                                    ),
                                ),
                            ),
                            (
                                Literal(
                                    String(
                                        "two",
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
                                    Bool(
                                        true,
                                    ),
                                ),
                                Literal(
                                    String(
                                        "yes",
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
                        Eq(
                            GetItem(
                                Identifier(
                                    "mixed",
                                ),
                                Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                            ),
                            Literal(
                                String(
                                    "one",
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
                            GetItem(
                                Identifier(
                                    "mixed",
                                ),
                                Literal(
                                    String(
                                        "two",
                                    ),
                                ),
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
                            GetItem(
                                Variable(
                                    "my_list",
                                ),
                                Literal(
                                    Int(
                                        0,
                                    ),
                                ),
                            ),
                            "op_eq",
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
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            GetItem(
                                Variable(
                                    "my_list",
                                ),
                                Literal(
                                    Int(
                                        2,
                                    ),
                                ),
                            ),
                            "op_eq",
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
                            GetItem(
                                Variable(
                                    "my_dict",
                                ),
                                Literal(
                                    String(
                                        "a",
                                    ),
                                ),
                            ),
                            "op_eq",
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
            Declare(
                "mixed",
                Some(
                    Dict(
                        [
                            (
                                Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                                Literal(
                                    String(
                                        "one",
                                    ),
                                ),
                            ),
                            (
                                Literal(
                                    String(
                                        "two",
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
                                    Bool(
                                        true,
                                    ),
                                ),
                                Literal(
                                    String(
                                        "yes",
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
                            GetItem(
                                Variable(
                                    "mixed",
                                ),
                                Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                String(
                                    "one",
                                ),
                            ),
                        ],
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
                            GetItem(
                                Variable(
                                    "mixed",
                                ),
                                Literal(
                                    String(
                                        "two",
                                    ),
                                ),
                            ),
                            "op_eq",
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
        ),
    ),
)
```