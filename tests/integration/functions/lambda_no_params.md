# Program
Status: 🟢
Assertions: 0

```rustleaf
// #[fail_quietly]
|| 42;
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
                    params: [],
                    body: Literal(
                        Int(
                            42,
                        ),
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
                                                    "print": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: RustFunction {
                                                                    name: "print",
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
                                                    "Function": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: TypeConstant {
                                                                    type_name: "Function",
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
                                                    "Dict": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: TypeConstant {
                                                                    type_name: "Dict",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    "assert": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: RustFunction {
                                                                    name: "assert",
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
                                                    "Null": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: TypeConstant {
                                                                    type_name: "Null",
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
                                                    "Float": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: TypeConstant {
                                                                    type_name: "Float",
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
                                                    "List": RustValue(
                                                        RustValueRef(
                                                            RefCell {
                                                                value: TypeConstant {
                                                                    type_name: "List",
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
        Token(Pipe),
        Token(Int, "42"),
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
                    params: [],
                    body: Expression(
                        Literal(
                            Int(
                                42,
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
                [],
                Literal(
                    Int(
                        42,
                    ),
                ),
            ),
        ),
    ),
)
```