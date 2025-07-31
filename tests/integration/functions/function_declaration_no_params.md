# Program
Status: 🔴
Assertions: 0

```rustleaf
fn hello() { 42 }
fn get_pi() { 3.14 }
fn get_greeting() { "Hello, World!" }

var result1 = hello();
var result2 = get_pi();
var result3 = get_greeting();

assert(result1 == 42);
assert(result2 == 3.14);
assert(result3 == "Hello, World!");
assert(hello() == 42);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Fn)
parse_statement: starting at position 0 (Fn)
```

# Result
```rust
Skipped due to parse error
```

# Lex
```rust
Ok(
    [
        Token(Fn),
        Token(Ident, "hello"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(Int, "42"),
        Token(RightBrace),
        Token(Fn),
        Token(Ident, "get_pi"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(Float, "3.14"),
        Token(RightBrace),
        Token(Fn),
        Token(Ident, "get_greeting"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(String, "Hello, World!"),
        Token(RightBrace),
        Token(Var),
        Token(Ident, "result1"),
        Token(Equal),
        Token(Ident, "hello"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result2"),
        Token(Equal),
        Token(Ident, "get_pi"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result3"),
        Token(Equal),
        Token(Ident, "get_greeting"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result1"),
        Token(EqualEqual),
        Token(Int, "42"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result2"),
        Token(EqualEqual),
        Token(Float, "3.14"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result3"),
        Token(EqualEqual),
        Token(String, "Hello, World!"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "hello"),
        Token(LeftParen),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "42"),
        Token(RightParen),
        Token(Semicolon),
        Token(Eof),
    ],
)
```

# Parse
```rust
Err(
    "Expected Hash, found Fn",
)
```

# Eval
```rust
Skipped due to parse error
```