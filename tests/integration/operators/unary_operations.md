# Program
Status: 🔴
Assertions: 0

```rustleaf
assert(-42 == -42);
assert(not true == false);
assert(not false == true);
assert(~5 == -6);
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
    "Unexpected character '~' at line 4, column 8",
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