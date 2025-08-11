# Program
Status: 🟢
Assertions: 1

```rustleaf
class Greeter {
    var name = "Eric";

    fn greet() {
        "Hello, ${self.name}"
    }
}

var greeter = Greeter();
var msg = greeter.greet();
assert(msg == "Hello, Eric");
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Class)
parse_statement: starting at position 0 (Class)
consume_token: position 0 consumed Class
consume_token: position 1 consumed Ident
consume_token: position 2 consumed LeftBrace
consume_token: position 3 consumed Var
consume_token: position 4 consumed Ident
consume_token: position 5 consumed Equal
parse_expression: starting at position 6 (String(Eric))
consume_token: position 6 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 7 consumed Semicolon
consume_token: position 8 consumed Fn
consume_token: position 9 consumed Ident
consume_token: position 10 consumed LeftParen
consume_token: position 11 consumed RightParen
consume_token: position 12 consumed LeftBrace
parse_statement: starting at position 13 (StringPart(Hello, ))
parse_statement: falling back to expression statement
parse_expression: starting at position 13 (StringPart(Hello, ))
parse_primary: success - parsing interpolated string
consume_token: position 13 consumed StringPart
consume_token: position 14 consumed InterpolationStart
parse_expression: starting at position 15 (Ident(self))
consume_token: position 15 consumed Ident
parse_primary: success - parsed identifier (self)
consume_token: position 16 consumed Dot
consume_token: position 17 consumed Ident
parse_expression: success - parsed precedence expression
consume_token: position 18 consumed InterpolationEnd
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 19
parse_expression: starting at position 13 (StringPart(Hello, ))
parse_primary: success - parsing interpolated string
consume_token: position 13 consumed StringPart
consume_token: position 14 consumed InterpolationStart
parse_expression: starting at position 15 (Ident(self))
consume_token: position 15 consumed Ident
parse_primary: success - parsed identifier (self)
consume_token: position 16 consumed Dot
consume_token: position 17 consumed Ident
parse_expression: success - parsed precedence expression
consume_token: position 18 consumed InterpolationEnd
parse_expression: success - parsed precedence expression
consume_token: position 19 consumed RightBrace
consume_token: position 20 consumed RightBrace
parse_statement: success - parsed class declaration
parse_program: parsing statement at position 21 (Var)
parse_statement: starting at position 21 (Var)
consume_token: position 21 consumed Var
consume_token: position 22 consumed Ident
consume_token: position 23 consumed Equal
parse_expression: starting at position 24 (Ident(Greeter))
consume_token: position 24 consumed Ident
parse_primary: success - parsed identifier (Greeter)
consume_token: position 25 consumed LeftParen
consume_token: position 26 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 27 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 28 (Var)
parse_statement: starting at position 28 (Var)
consume_token: position 28 consumed Var
consume_token: position 29 consumed Ident
consume_token: position 30 consumed Equal
parse_expression: starting at position 31 (Ident(greeter))
consume_token: position 31 consumed Ident
parse_primary: success - parsed identifier (greeter)
consume_token: position 32 consumed Dot
consume_token: position 33 consumed Ident
consume_token: position 34 consumed LeftParen
consume_token: position 35 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 36 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 37 (Ident(assert))
parse_statement: starting at position 37 (Ident(assert))
consume_token: position 37 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 37 (Ident(assert))
consume_token: position 37 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 38 consumed LeftParen
parse_expression: starting at position 39 (Ident(msg))
consume_token: position 39 consumed Ident
parse_primary: success - parsed identifier (msg)
consume_token: position 40 consumed EqualEqual
consume_token: position 41 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 42 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 43 consumed Semicolon
parse_program: parsed 4 statements
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
        0: Token(Class),
        1: Token(Ident, "Greeter"),
        2: Token(LeftBrace),
        3: Token(Var),
        4: Token(Ident, "name"),
        5: Token(Equal),
        6: Token(String, "Eric"),
        7: Token(Semicolon),
        8: Token(Fn),
        9: Token(Ident, "greet"),
        10: Token(LeftParen),
        11: Token(RightParen),
        12: Token(LeftBrace),
        13: Token(StringPart, "Hello, "),
        14: Token(InterpolationStart),
        15: Token(Ident, "self"),
        16: Token(Dot),
        17: Token(Ident, "name"),
        18: Token(InterpolationEnd),
        19: Token(RightBrace),
        20: Token(RightBrace),
        21: Token(Var),
        22: Token(Ident, "greeter"),
        23: Token(Equal),
        24: Token(Ident, "Greeter"),
        25: Token(LeftParen),
        26: Token(RightParen),
        27: Token(Semicolon),
        28: Token(Var),
        29: Token(Ident, "msg"),
        30: Token(Equal),
        31: Token(Ident, "greeter"),
        32: Token(Dot),
        33: Token(Ident, "greet"),
        34: Token(LeftParen),
        35: Token(RightParen),
        36: Token(Semicolon),
        37: Token(Ident, "assert"),
        38: Token(LeftParen),
        39: Token(Ident, "msg"),
        40: Token(EqualEqual),
        41: Token(String, "Hello, Eric"),
        42: Token(RightParen),
        43: Token(Semicolon),
        44: Token(Eof)
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            ClassDecl {
                name: "Greeter",
                members: [
                    ClassMember {
                        name: "name",
                        kind: Field(
                            Some(
                                Literal(
                                    String(
                                        "Eric",
                                    ),
                                ),
                            ),
                        ),
                    },
                    ClassMember {
                        name: "greet",
                        kind: Method {
                            params: [],
                            body: Block {
                                statements: [],
                                final_expr: Some(
                                    InterpolatedString(
                                        [
                                            Text(
                                                "Hello, ",
                                            ),
                                            Expression(
                                                GetAttr(
                                                    Identifier(
                                                        "self",
                                                    ),
                                                    "name",
                                                ),
                                            ),
                                        ],
                                    ),
                                ),
                            },
                        },
                    },
                ],
                is_pub: false,
            },
            VarDecl {
                pattern: Variable(
                    "greeter",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "Greeter",
                        ),
                        [],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "msg",
                ),
                value: Some(
                    MethodCall(
                        Identifier(
                            "greeter",
                        ),
                        "greet",
                        [],
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
                                "msg",
                            ),
                            Literal(
                                String(
                                    "Hello, Eric",
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
    RustValue(<unknown>),
)
```