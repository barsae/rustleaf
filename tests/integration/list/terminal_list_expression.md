# Program
Status: 🔴
Assertions: 0

```rustleaf
// Copy of the original failing code from project-euler/004.rustleaf
// This should reproduce the "Unexpected token: Comma" parse error

fn is_palindrome(n) {
    var s = str(n);
    var chars = s.to_list();
    var len = chars.len();
    
    for i in range(len / 2) {
        if chars[i] != chars[len - 1 - i] {
            return false;
        }
    }
    true
}

fn solve_euler_004(min_digits, max_digits) {
    var largest_palindrome = 0;
    var found_a = 0;
    var found_b = 0;
    
    // Start from the largest numbers and work down
    for a in range(max_digits, min_digits - 1, -1) {
        for b in range(a, min_digits - 1, -1) {
            var product = a * b;
            
            // Early termination: if product is smaller than current best, skip
            if product <= largest_palindrome {
                break;
            }
            
            if is_palindrome(product) {
                largest_palindrome = product;
                found_a = a;
                found_b = b;
            }
        }
    }
    
    [largest_palindrome, found_a, found_b]
}

// Test with 2-digit numbers  
var test_result = solve_euler_004(10, 99);
assert(test_result[0] == 9009);
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
        Token(Fn),
        Token(Ident, "is_palindrome"),
        Token(LeftParen),
        Token(Ident, "n"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "s"),
        Token(Equal),
        Token(Ident, "str"),
        Token(LeftParen),
        Token(Ident, "n"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "chars"),
        Token(Equal),
        Token(Ident, "s"),
        Token(Dot),
        Token(Ident, "to_list"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "len"),
        Token(Equal),
        Token(Ident, "chars"),
        Token(Dot),
        Token(Ident, "len"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(For),
        Token(Ident, "i"),
        Token(In),
        Token(Ident, "range"),
        Token(LeftParen),
        Token(Ident, "len"),
        Token(Slash),
        Token(Int, "2"),
        Token(RightParen),
        Token(LeftBrace),
        Token(If),
        Token(Ident, "chars"),
        Token(LeftBracket),
        Token(Ident, "i"),
        Token(RightBracket),
        Token(BangEqual),
        Token(Ident, "chars"),
        Token(LeftBracket),
        Token(Ident, "len"),
        Token(Minus),
        Token(Int, "1"),
        Token(Minus),
        Token(Ident, "i"),
        Token(RightBracket),
        Token(LeftBrace),
        Token(Return),
        Token(False),
        Token(Semicolon),
        Token(RightBrace),
        Token(RightBrace),
        Token(True),
        Token(RightBrace),
        Token(Fn),
        Token(Ident, "solve_euler_004"),
        Token(LeftParen),
        Token(Ident, "min_digits"),
        Token(Comma),
        Token(Ident, "max_digits"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "largest_palindrome"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "found_a"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "found_b"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(For),
        Token(Ident, "a"),
        Token(In),
        Token(Ident, "range"),
        Token(LeftParen),
        Token(Ident, "max_digits"),
        Token(Comma),
        Token(Ident, "min_digits"),
        Token(Minus),
        Token(Int, "1"),
        Token(Comma),
        Token(Minus),
        Token(Int, "1"),
        Token(RightParen),
        Token(LeftBrace),
        Token(For),
        Token(Ident, "b"),
        Token(In),
        Token(Ident, "range"),
        Token(LeftParen),
        Token(Ident, "a"),
        Token(Comma),
        Token(Ident, "min_digits"),
        Token(Minus),
        Token(Int, "1"),
        Token(Comma),
        Token(Minus),
        Token(Int, "1"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "product"),
        Token(Equal),
        Token(Ident, "a"),
        Token(Star),
        Token(Ident, "b"),
        Token(Semicolon),
        Token(If),
        Token(Ident, "product"),
        Token(LessEqual),
        Token(Ident, "largest_palindrome"),
        Token(LeftBrace),
        Token(Break),
        Token(Semicolon),
        Token(RightBrace),
        Token(If),
        Token(Ident, "is_palindrome"),
        Token(LeftParen),
        Token(Ident, "product"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "largest_palindrome"),
        Token(Equal),
        Token(Ident, "product"),
        Token(Semicolon),
        Token(Ident, "found_a"),
        Token(Equal),
        Token(Ident, "a"),
        Token(Semicolon),
        Token(Ident, "found_b"),
        Token(Equal),
        Token(Ident, "b"),
        Token(Semicolon),
        Token(RightBrace),
        Token(RightBrace),
        Token(RightBrace),
        Token(LeftBracket),
        Token(Ident, "largest_palindrome"),
        Token(Comma),
        Token(Ident, "found_a"),
        Token(Comma),
        Token(Ident, "found_b"),
        Token(RightBracket),
        Token(RightBrace),
        Token(Var),
        Token(Ident, "test_result"),
        Token(Equal),
        Token(Ident, "solve_euler_004"),
        Token(LeftParen),
        Token(Int, "10"),
        Token(Comma),
        Token(Int, "99"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "test_result"),
        Token(LeftBracket),
        Token(Int, "0"),
        Token(RightBracket),
        Token(EqualEqual),
        Token(Int, "9009"),
        Token(RightParen),
        Token(Semicolon),
        Token(Eof),
    ],
)
```

# Parse
```rust
Err(
    "Unexpected token: Comma",
)
```

# Eval
```rust
Skipped due to parse error
```