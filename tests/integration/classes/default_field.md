# Program
Status: 🔴
Assertions: 0

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
```

# Result
```rust
Skipped due to parse error
```

# Lex
```rust
Ok(
    [
        Token(Class),
        Token(Ident, "Greeter"),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "name"),
        Token(Equal),
        Token(String, "Eric"),
        Token(Semicolon),
        Token(Fn),
        Token(Ident, "greet"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(StringPart, "Hello, "),
        Token(InterpolationStart),
        Token(Ident, "self"),
        Token(Dot),
        Token(Ident, "name"),
        Token(InterpolationEnd),
        Token(RightBrace),
        Token(RightBrace),
        Token(Var),
        Token(Ident, "greeter"),
        Token(Equal),
        Token(Ident, "Greeter"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "msg"),
        Token(Equal),
        Token(Ident, "greeter"),
        Token(Dot),
        Token(Ident, "greet"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "msg"),
        Token(EqualEqual),
        Token(String, "Hello, Eric"),
        Token(RightParen),
        Token(Semicolon),
        Token(Eof),
    ],
)
```

# Parse
```rust
Err(
    "Expected Hash, found Class",
)
```

# Eval
```rust
Skipped due to parse error
```