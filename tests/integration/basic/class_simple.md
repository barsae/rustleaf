# Program 🔴
```rustleaf
// #[fail_quietly]
class Person {
    var name;
    
    fn greet() {
        print("Hello");
    }
}
```

# Output
None

# Result
```rust
Err(
    "Statement not yet implemented: ClassDecl { name: \"Person\", members: [ClassMember { name: \"name\", kind: Field(None) }, ClassMember { name: \"greet\", kind: Method { params: [], body: Block { statements: [Expression(FunctionCall(Identifier(\"print\"), [Literal(String(\"Hello\"))]))], final_expr: None } } }], is_pub: false }",
)
```

# Lex
```rust
Ok(
    [
        Token(Class),
        Token(Ident, "Person"),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "name"),
        Token(Semicolon),
        Token(Fn),
        Token(Ident, "greet"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "print"),
        Token(LeftParen),
        Token(String, "Hello"),
        Token(RightParen),
        Token(Semicolon),
        Token(RightBrace),
        Token(RightBrace),
        Token(Eof),
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            ClassDecl {
                name: "Person",
                members: [
                    ClassMember {
                        name: "name",
                        kind: Field(
                            None,
                        ),
                    },
                    ClassMember {
                        name: "greet",
                        kind: Method {
                            params: [],
                            body: Block {
                                statements: [
                                    Expression(
                                        FunctionCall(
                                            Identifier(
                                                "print",
                                            ),
                                            [
                                                Literal(
                                                    String(
                                                        "Hello",
                                                    ),
                                                ),
                                            ],
                                        ),
                                    ),
                                ],
                                final_expr: None,
                            },
                        },
                    },
                ],
                is_pub: false,
            },
        ],
    ),
)
```

# Eval
```rust
Err(
    "Statement not yet implemented: ClassDecl { name: \"Person\", members: [ClassMember { name: \"name\", kind: Field(None) }, ClassMember { name: \"greet\", kind: Method { params: [], body: Block { statements: [Expression(FunctionCall(Identifier(\"print\"), [Literal(String(\"Hello\"))]))], final_expr: None } } }], is_pub: false }",
)
```