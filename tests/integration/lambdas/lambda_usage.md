# Program
Status: 🔴
Assertions: 0

```rustleaf
// Test lambda expressions in practical usage scenarios

// Simple lambda assignment and calling
var add_one = |x| x + 1;
assert(add_one(5) == 6);

// Lambda with multiple parameters (when supported)
// var add = |x, y| x + y;
// assert(add(3, 4) == 7);

// Lambda as argument to other functions (higher-order functions)
fn apply(func, value) {
    func(value)
}
var double = |x| x * 2;
assert(apply(double, 21) == 42);

// Lambda with block body
var complex_func = |n| {
    var result = n * n;
    result + 1
};
assert(complex_func(3) == 10); // 3*3 + 1 = 10

// Lambda closures - capturing variables from outer scope
var multiplier = 3;
var multiply_by_three = |x| x * multiplier;
assert(multiply_by_three(4) == 12);
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
        Token(Ident, "add_one"),
        Token(Equal),
        Token(Pipe),
        Token(Ident, "x"),
        Token(Pipe),
        Token(Ident, "x"),
        Token(Plus),
        Token(Int, "1"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "add_one"),
        Token(LeftParen),
        Token(Int, "5"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "6"),
        Token(RightParen),
        Token(Semicolon),
        Token(Fn),
        Token(Ident, "apply"),
        Token(LeftParen),
        Token(Ident, "func"),
        Token(Comma),
        Token(Ident, "value"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "func"),
        Token(LeftParen),
        Token(Ident, "value"),
        Token(RightParen),
        Token(RightBrace),
        Token(Var),
        Token(Ident, "double"),
        Token(Equal),
        Token(Pipe),
        Token(Ident, "x"),
        Token(Pipe),
        Token(Ident, "x"),
        Token(Star),
        Token(Int, "2"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "apply"),
        Token(LeftParen),
        Token(Ident, "double"),
        Token(Comma),
        Token(Int, "21"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "42"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "complex_func"),
        Token(Equal),
        Token(Pipe),
        Token(Ident, "n"),
        Token(Pipe),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "result"),
        Token(Equal),
        Token(Ident, "n"),
        Token(Star),
        Token(Ident, "n"),
        Token(Semicolon),
        Token(Ident, "result"),
        Token(Plus),
        Token(Int, "1"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "complex_func"),
        Token(LeftParen),
        Token(Int, "3"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "10"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "multiplier"),
        Token(Equal),
        Token(Int, "3"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "multiply_by_three"),
        Token(Equal),
        Token(Pipe),
        Token(Ident, "x"),
        Token(Pipe),
        Token(Ident, "x"),
        Token(Star),
        Token(Ident, "multiplier"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "multiply_by_three"),
        Token(LeftParen),
        Token(Int, "4"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "12"),
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