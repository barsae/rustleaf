# Program
Status: 🔴
Assertions: 0

```rustleaf
fn add(x, y) { x + y }
fn multiply(a, b) { a * b }

var sum = add(5, 3);
var product = multiply(4, 6);

assert(sum == 8);
assert(product == 24);
assert(add(10, -2) == 8);
assert(multiply(0, 100) == 0);
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
        Token(Ident, "add"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(Comma),
        Token(Ident, "y"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "x"),
        Token(Plus),
        Token(Ident, "y"),
        Token(RightBrace),
        Token(Fn),
        Token(Ident, "multiply"),
        Token(LeftParen),
        Token(Ident, "a"),
        Token(Comma),
        Token(Ident, "b"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "a"),
        Token(Star),
        Token(Ident, "b"),
        Token(RightBrace),
        Token(Var),
        Token(Ident, "sum"),
        Token(Equal),
        Token(Ident, "add"),
        Token(LeftParen),
        Token(Int, "5"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "product"),
        Token(Equal),
        Token(Ident, "multiply"),
        Token(LeftParen),
        Token(Int, "4"),
        Token(Comma),
        Token(Int, "6"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "sum"),
        Token(EqualEqual),
        Token(Int, "8"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "product"),
        Token(EqualEqual),
        Token(Int, "24"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "add"),
        Token(LeftParen),
        Token(Int, "10"),
        Token(Comma),
        Token(Minus),
        Token(Int, "2"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "8"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "multiply"),
        Token(LeftParen),
        Token(Int, "0"),
        Token(Comma),
        Token(Int, "100"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "0"),
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