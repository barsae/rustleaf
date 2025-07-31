# Program
Status: 🔴
Assertions: 0

```rustleaf
use math::adder;
var x = adder(1, 2);
assert(x == 3);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Use)
parse_statement: starting at position 0 (Use)
```

# Result
```rust
Skipped due to parse error
```

# Lex
```rust
Ok(
    [
        Token(Use),
        Token(Ident, "math"),
        Token(DoubleColon),
        Token(Ident, "adder"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "x"),
        Token(Equal),
        Token(Ident, "adder"),
        Token(LeftParen),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(EqualEqual),
        Token(Int, "3"),
        Token(RightParen),
        Token(Semicolon),
        Token(Eof),
    ],
)
```

# Parse
```rust
Err(
    "Expected Hash, found Use",
)
```

# Eval
```rust
Skipped due to parse error
```