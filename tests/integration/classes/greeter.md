# Program
Status: 🟢
Assertions: 1

```rustleaf
class Greeter {
    var name;

    fn greet() {
        "Hello, ${self.name}"
    }
}

var greeter = Greeter();
greeter.name = "Eric";
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
consume_token: position 5 consumed Semicolon
consume_token: position 6 consumed Fn
consume_token: position 7 consumed Ident
consume_token: position 8 consumed LeftParen
consume_token: position 9 consumed RightParen
consume_token: position 10 consumed LeftBrace
parse_statement: starting at position 11 (StringPart(Hello, ))
parse_statement: falling back to expression statement
parse_expression: starting at position 11 (StringPart(Hello, ))
parse_primary: success - parsing interpolated string
consume_token: position 11 consumed StringPart
consume_token: position 12 consumed InterpolationStart
parse_expression: starting at position 13 (Ident(self))
consume_token: position 13 consumed Ident
parse_primary: success - parsed identifier (self)
consume_token: position 14 consumed Dot
consume_token: position 15 consumed Ident
parse_expression: success - parsed precedence expression
consume_token: position 16 consumed InterpolationEnd
parse_expression: success - parsed precedence expression
parse_statement: failed - Expected Semicolon, found RightBrace at position 17
parse_expression: starting at position 11 (StringPart(Hello, ))
parse_primary: success - parsing interpolated string
consume_token: position 11 consumed StringPart
consume_token: position 12 consumed InterpolationStart
parse_expression: starting at position 13 (Ident(self))
consume_token: position 13 consumed Ident
parse_primary: success - parsed identifier (self)
consume_token: position 14 consumed Dot
consume_token: position 15 consumed Ident
parse_expression: success - parsed precedence expression
consume_token: position 16 consumed InterpolationEnd
parse_expression: success - parsed precedence expression
consume_token: position 17 consumed RightBrace
consume_token: position 18 consumed RightBrace
parse_statement: success - parsed class declaration
parse_program: parsing statement at position 19 (Var)
parse_statement: starting at position 19 (Var)
consume_token: position 19 consumed Var
consume_token: position 20 consumed Ident
consume_token: position 21 consumed Equal
parse_expression: starting at position 22 (Ident(Greeter))
consume_token: position 22 consumed Ident
parse_primary: success - parsed identifier (Greeter)
consume_token: position 23 consumed LeftParen
consume_token: position 24 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 25 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 26 (Ident(greeter))
parse_statement: starting at position 26 (Ident(greeter))
consume_token: position 26 consumed Ident
consume_token: position 27 consumed Dot
consume_token: position 28 consumed Ident
consume_token: position 29 consumed Equal
parse_expression: starting at position 30 (String(Eric))
consume_token: position 30 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 31 consumed Semicolon
parse_statement: success - parsed assignment
parse_program: parsing statement at position 32 (Var)
parse_statement: starting at position 32 (Var)
consume_token: position 32 consumed Var
consume_token: position 33 consumed Ident
consume_token: position 34 consumed Equal
parse_expression: starting at position 35 (Ident(greeter))
consume_token: position 35 consumed Ident
parse_primary: success - parsed identifier (greeter)
consume_token: position 36 consumed Dot
consume_token: position 37 consumed Ident
consume_token: position 38 consumed LeftParen
consume_token: position 39 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 40 consumed Semicolon
parse_statement: success - parsed var declaration
parse_program: parsing statement at position 41 (Ident(assert))
parse_statement: starting at position 41 (Ident(assert))
consume_token: position 41 consumed Ident
parse_statement: falling back to expression statement
parse_expression: starting at position 41 (Ident(assert))
consume_token: position 41 consumed Ident
parse_primary: success - parsed identifier (assert)
consume_token: position 42 consumed LeftParen
parse_expression: starting at position 43 (Ident(msg))
consume_token: position 43 consumed Ident
parse_primary: success - parsed identifier (msg)
consume_token: position 44 consumed EqualEqual
consume_token: position 45 consumed String
parse_primary: success - parsed numeric/string literal
parse_expression: success - parsed precedence expression
consume_token: position 46 consumed RightParen
parse_expression: success - parsed precedence expression
consume_token: position 47 consumed Semicolon
parse_program: parsed 5 statements
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
        5: Token(Semicolon),
        6: Token(Fn),
        7: Token(Ident, "greet"),
        8: Token(LeftParen),
        9: Token(RightParen),
        10: Token(LeftBrace),
        11: Token(StringPart, "Hello, "),
        12: Token(InterpolationStart),
        13: Token(Ident, "self"),
        14: Token(Dot),
        15: Token(Ident, "name"),
        16: Token(InterpolationEnd),
        17: Token(RightBrace),
        18: Token(RightBrace),
        19: Token(Var),
        20: Token(Ident, "greeter"),
        21: Token(Equal),
        22: Token(Ident, "Greeter"),
        23: Token(LeftParen),
        24: Token(RightParen),
        25: Token(Semicolon),
        26: Token(Ident, "greeter"),
        27: Token(Dot),
        28: Token(Ident, "name"),
        29: Token(Equal),
        30: Token(String, "Eric"),
        31: Token(Semicolon),
        32: Token(Var),
        33: Token(Ident, "msg"),
        34: Token(Equal),
        35: Token(Ident, "greeter"),
        36: Token(Dot),
        37: Token(Ident, "greet"),
        38: Token(LeftParen),
        39: Token(RightParen),
        40: Token(Semicolon),
        41: Token(Ident, "assert"),
        42: Token(LeftParen),
        43: Token(Ident, "msg"),
        44: Token(EqualEqual),
        45: Token(String, "Hello, Eric"),
        46: Token(RightParen),
        47: Token(Semicolon),
        48: Token(Eof)
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
                            None,
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
            Assignment {
                target: GetAttr(
                    Identifier(
                        "greeter",
                    ),
                    "name",
                ),
                op: Assign,
                value: Literal(
                    String(
                        "Eric",
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