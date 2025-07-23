# Program

```rustleaf
class Person {
    var name;
    
    fn greet() {
        print("Hello");
    }
}
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Class,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "Person",
            ),
        },
        Token {
            token_type: LeftBrace,
            text: None,
        },
        Token {
            token_type: Var,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "name",
            ),
        },
        Token {
            token_type: Semicolon,
            text: None,
        },
        Token {
            token_type: Fn,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "greet",
            ),
        },
        Token {
            token_type: LeftParen,
            text: None,
        },
        Token {
            token_type: RightParen,
            text: None,
        },
        Token {
            token_type: LeftBrace,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "print",
            ),
        },
        Token {
            token_type: LeftParen,
            text: None,
        },
        Token {
            token_type: String,
            text: Some(
                "Hello",
            ),
        },
        Token {
            token_type: RightParen,
            text: None,
        },
        Token {
            token_type: Semicolon,
            text: None,
        },
        Token {
            token_type: RightBrace,
            text: None,
        },
        Token {
            token_type: RightBrace,
            text: None,
        },
        Token {
            token_type: Eof,
            text: None,
        },
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

# Output

```

```

# Result

```rust
Err(
    "Statement not yet implemented: ClassDecl { name: \"Person\", members: [ClassMember { name: \"name\", kind: Field(None) }, ClassMember { name: \"greet\", kind: Method { params: [], body: Block { statements: [Expression(FunctionCall(Identifier(\"print\"), [Literal(String(\"Hello\"))]))], final_expr: None } } }], is_pub: false }",
)
```
