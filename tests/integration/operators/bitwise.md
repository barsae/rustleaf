# Program
Status: 🔴
Assertions: 0

```rustleaf
// Bitwise operators test
assert((5 & 3) == 1);  // 101 & 011 = 001
assert((5 | 3) == 7);  // 101 | 011 = 111
assert((5 ^ 3) == 6);  // 101 ^ 011 = 110

// Bit shifts
assert((8 << 1) == 16);
assert((8 >> 1) == 4);
assert((8 >> 2) == 2);

// Bitwise NOT
assert((~5) == -6);  // Two's complement
```

# Output
None

# Result
```rust
Skipped due to parse error
```

# Lex
```rust
Err(
    "Unexpected character '&' at line 2, column 11",
)
```

# Parse
```rust
Skipped due to lex error
```

# Eval
```rust
Skipped due to parse error
```