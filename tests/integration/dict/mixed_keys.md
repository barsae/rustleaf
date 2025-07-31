# Program
Status: 🔴
Assertions: 0

```rustleaf
// Dict access with different key types
var my_dict = {"a": 1, "b": 2};
assert(my_dict["a"] == 1);

// Mixed key types
var mixed = {1: "one", "two": 2, true: "yes"};
assert(mixed[1] == "one");
assert(mixed["two"] == 2);
```

# Output
None

# Result
```rust
Skipped due to parse error
```

# Lex
```rust
Ok(
    [
        Token(Var),
        Token(Ident, "my_dict"),
        Token(Equal),
        Token(LeftBrace),
        Token(String, "a"),
        Token(Colon),
        Token(Int, "1"),
        Token(Comma),
        Token(String, "b"),
        Token(Colon),
        Token(Int, "2"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "my_dict"),
        Token(LeftBracket),
        Token(String, "a"),
        Token(RightBracket),
        Token(EqualEqual),
        Token(Int, "1"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "mixed"),
        Token(Equal),
        Token(LeftBrace),
        Token(Int, "1"),
        Token(Colon),
        Token(String, "one"),
        Token(Comma),
        Token(String, "two"),
        Token(Colon),
        Token(Int, "2"),
        Token(Comma),
        Token(True),
        Token(Colon),
        Token(String, "yes"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "mixed"),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(RightBracket),
        Token(EqualEqual),
        Token(String, "one"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "mixed"),
        Token(LeftBracket),
        Token(String, "two"),
        Token(RightBracket),
        Token(EqualEqual),
        Token(Int, "2"),
        Token(RightParen),
        Token(Semicolon),
        Token(Eof),
    ],
)
```

# Parse
```rust
Err(
    "Expected expression, found Var",
)
```

# Eval
```rust
Skipped due to parse error
```