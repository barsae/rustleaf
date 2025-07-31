# Program
Status: 🔴
Assertions: 0

```rustleaf
var increment = |x| x + 1;
var double = |y| y * 2;
var add_ten = |z| z + 10;

assert(increment(5) == 6);
assert(double(7) == 14);  
assert(add_ten(15) == 25);
assert(increment(0) == 1);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
```

# Result
```rust
Skipped due to parse error
```

# Lex
```rust
Ok(
    [
        Token(Var),
        Token(Ident, "increment"),
        Token(Equal),
        Token(Pipe),
        Token(Ident, "x"),
        Token(Pipe),
        Token(Ident, "x"),
        Token(Plus),
        Token(Int, "1"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "double"),
        Token(Equal),
        Token(Pipe),
        Token(Ident, "y"),
        Token(Pipe),
        Token(Ident, "y"),
        Token(Star),
        Token(Int, "2"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "add_ten"),
        Token(Equal),
        Token(Pipe),
        Token(Ident, "z"),
        Token(Pipe),
        Token(Ident, "z"),
        Token(Plus),
        Token(Int, "10"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "increment"),
        Token(LeftParen),
        Token(Int, "5"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "6"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "double"),
        Token(LeftParen),
        Token(Int, "7"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "14"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "add_ten"),
        Token(LeftParen),
        Token(Int, "15"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "25"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "increment"),
        Token(LeftParen),
        Token(Int, "0"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "1"),
        Token(RightParen),
        Token(Semicolon),
        Token(Eof),
    ],
)
```

# Parse
```rust
Err(
    "Expected Hash, found Var",
)
```

# Eval
```rust
Skipped due to parse error
```