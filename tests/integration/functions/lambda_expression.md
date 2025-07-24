# Program
Status: 🟢
Assertions: 0

```rustleaf
// #[fail_quietly]
|x| x + 1;
```

# Output
None

# Result
```rust
Ok(
    RustValue(
        RustValueRef(
            RefCell {
                value: RustLeafFunction {
                    params: [
                        "x",
                    ],
                    body: Call(
                        GetAttr(
                            Variable(
                                "x",
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
                    closure_env: ScopeRef(
                        RefCell {
                            value: Scope {
                                vars: {},
                                parent: Some(
                                    ScopeRef(
                                        RefCell {
                                            value: Scope {
                                                vars: {
                                                    "assert": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: RustFunction {
                                                                    name: "assert",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    "Int": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: TypeConstant {
                                                                    type_name: "Int",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    "str": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: RustFunction {
                                                                    name: "str",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    "Null": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: TypeConstant {
                                                                    type_name: "Null",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    "String": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: TypeConstant {
                                                                    type_name: "String",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    "List": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: TypeConstant {
                                                                    type_name: "List",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    "Function": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: TypeConstant {
                                                                    type_name: "Function",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    "Dict": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: TypeConstant {
                                                                    type_name: "Dict",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    "is_unit": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: RustFunction {
                                                                    name: "is_unit",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    "Float": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: TypeConstant {
                                                                    type_name: "Float",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    "Bool": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: TypeConstant {
                                                                    type_name: "Bool",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    "raise": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: RustFunction {
                                                                    name: "raise",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    "print": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: RustFunction {
                                                                    name: "print",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    "Range": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: TypeConstant {
                                                                    type_name: "Range",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    "Unit": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: TypeConstant {
                                                                    type_name: "Unit",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                },
                                                parent: None,
                                            },
                                        },
                                    ),
                                ),
                            },
                        },
                    ),
                },
            },
        ),
    ),
)
```

# Lex
```rust
Ok(
    [
        Token(Pipe),
        Token(Ident, "x"),
        Token(Pipe),
        Token(Ident, "x"),
        Token(Plus),
        Token(Int, "1"),
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
                Lambda {
                    params: [
                        "x",
                    ],
                    body: Expression(
                        Add(
                            Identifier(
                                "x",
                            ),
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                        ),
                    ),
                },
            ),
        ],
    ),
)
```

# Eval
```rust
Ok(
    Block(
        [],
        Some(
            Lambda(
                [
                    "x",
                ],
                Call(
                    GetAttr(
                        Variable(
                            "x",
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
        ),
    ),
)
```