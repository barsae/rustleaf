# Program
Status: 🔴
Assertions: 0

```rustleaf
var x = 1;
var result = match x {
    0: {
        "zero"
    }
    1: {
        "one"
    }
    _: {
        "other"
    }
};
assert(result == "one");

var y = 42;
var result2 = match y {
    0: {
        "zero"
    }
    1: {
        "one"
    }
    _: {
        "other"
    }
};
assert(result2 == "other");

var z = 0;
var result3 = match z {
    0: {
        "zero"
    }
    1: {
        "one"
    }
    _: {
        "other"
    }
};
assert(result3 == "zero");
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
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "1"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result"),
        Token(Equal),
        Token(Match),
        Token(Ident, "x"),
        Token(LeftBrace),
        Token(Int, "0"),
        Token(Colon),
        Token(LeftBrace),
        Token(String, "zero"),
        Token(RightBrace),
        Token(Int, "1"),
        Token(Colon),
        Token(LeftBrace),
        Token(String, "one"),
        Token(RightBrace),
        Token(Ident, "_"),
        Token(Colon),
        Token(LeftBrace),
        Token(String, "other"),
        Token(RightBrace),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result"),
        Token(EqualEqual),
        Token(String, "one"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "y"),
        Token(Equal),
        Token(Int, "42"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result2"),
        Token(Equal),
        Token(Match),
        Token(Ident, "y"),
        Token(LeftBrace),
        Token(Int, "0"),
        Token(Colon),
        Token(LeftBrace),
        Token(String, "zero"),
        Token(RightBrace),
        Token(Int, "1"),
        Token(Colon),
        Token(LeftBrace),
        Token(String, "one"),
        Token(RightBrace),
        Token(Ident, "_"),
        Token(Colon),
        Token(LeftBrace),
        Token(String, "other"),
        Token(RightBrace),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result2"),
        Token(EqualEqual),
        Token(String, "other"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "z"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result3"),
        Token(Equal),
        Token(Match),
        Token(Ident, "z"),
        Token(LeftBrace),
        Token(Int, "0"),
        Token(Colon),
        Token(LeftBrace),
        Token(String, "zero"),
        Token(RightBrace),
        Token(Int, "1"),
        Token(Colon),
        Token(LeftBrace),
        Token(String, "one"),
        Token(RightBrace),
        Token(Ident, "_"),
        Token(Colon),
        Token(LeftBrace),
        Token(String, "other"),
        Token(RightBrace),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result3"),
        Token(EqualEqual),
        Token(String, "zero"),
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