# Program
Status: 🔴
Assertions: 0

```rustleaf
// Test 'in' operator (op_contains)

// String contains
assert("hello" in "hello world");
assert(not ("xyz" in "hello world"));

// List contains
var my_list = [1, 2, 3, "hello"];
assert(2 in my_list);
assert("hello" in my_list);
assert(not (99 in my_list));

// Dict contains (check keys)
var my_dict = {"a": 1, "b": 2, 3: "three"};
assert("a" in my_dict);
assert("b" in my_dict);
assert(3 in my_dict);
assert(not ("z" in my_dict));
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
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(String, "hello"),
        Token(In),
        Token(String, "hello world"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(LeftParen),
        Token(String, "xyz"),
        Token(In),
        Token(String, "hello world"),
        Token(RightParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "my_list"),
        Token(Equal),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(Comma),
        Token(String, "hello"),
        Token(RightBracket),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "2"),
        Token(In),
        Token(Ident, "my_list"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(String, "hello"),
        Token(In),
        Token(Ident, "my_list"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(LeftParen),
        Token(Int, "99"),
        Token(In),
        Token(Ident, "my_list"),
        Token(RightParen),
        Token(RightParen),
        Token(Semicolon),
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
        Token(Comma),
        Token(Int, "3"),
        Token(Colon),
        Token(String, "three"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(String, "a"),
        Token(In),
        Token(Ident, "my_dict"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(String, "b"),
        Token(In),
        Token(Ident, "my_dict"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "3"),
        Token(In),
        Token(Ident, "my_dict"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Not),
        Token(LeftParen),
        Token(String, "z"),
        Token(In),
        Token(Ident, "my_dict"),
        Token(RightParen),
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