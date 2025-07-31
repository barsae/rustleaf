# Program
Status: 🔴
Assertions: 0

```rustleaf
// Comprehensive test of is and is not operators

// Basic type checking
var int_val = 42;
var float_val = 3.14;
var string_val = "hello";
var bool_val = true;
var list_val = [1, 2, 3];
var dict_val = {"key": "value"};
var range_val = 1..10;
var lambda_val = |x| x + 1;

// Test 'is' operator with various types
assert(int_val is Int, "int_val should be Int");
assert(float_val is Float, "float_val should be Float");
assert(string_val is String, "string_val should be String");
assert(bool_val is Bool, "bool_val should be Bool");
assert(list_val is List, "list_val should be List");
assert(dict_val is Dict, "dict_val should be Dict");
assert(range_val is Range, "range_val should be Range");
assert(lambda_val is Function, "lambda_val should be Function");

// Test 'is not' operator
assert(int_val is not String, "int_val should not be String");
assert(float_val is not Int, "float_val should not be Int");
assert(string_val is not Bool, "string_val should not be Bool");
assert(bool_val is not List, "bool_val should not be List");
assert(list_val is not Dict, "list_val should not be Dict");
assert(dict_val is not Range, "dict_val should not be Range");
assert(range_val is not Function, "range_val should not be Function");
assert(lambda_val is not Int, "lambda_val should not be Int");

// Test with special values
fn f() {}
var unit = f();
assert(null is Null, "null should be Null");
assert(unit is Unit, "unit should be Unit");
assert(null is not Unit, "null should not be Unit");
assert(unit is not Null, "unit should not be Null");

// Test identity comparison fallback for non-type values
var list1 = [1, 2, 3];
var list2 = [1, 2, 3];
var list3 = list1;
assert(list1 is not list2, "list1 should not be identical to list2 (different objects)");
assert(list1 is list3, "list1 should be identical to list3 (same object reference)");
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
        Token(Ident, "int_val"),
        Token(Equal),
        Token(Int, "42"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "float_val"),
        Token(Equal),
        Token(Float, "3.14"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "string_val"),
        Token(Equal),
        Token(String, "hello"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "bool_val"),
        Token(Equal),
        Token(True),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "list_val"),
        Token(Equal),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightBracket),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "dict_val"),
        Token(Equal),
        Token(LeftBrace),
        Token(String, "key"),
        Token(Colon),
        Token(String, "value"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "range_val"),
        Token(Equal),
        Token(Int, "1"),
        Token(DotDot),
        Token(Int, "10"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "lambda_val"),
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
        Token(Ident, "int_val"),
        Token(Is),
        Token(Ident, "Int"),
        Token(Comma),
        Token(String, "int_val should be Int"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "float_val"),
        Token(Is),
        Token(Ident, "Float"),
        Token(Comma),
        Token(String, "float_val should be Float"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "string_val"),
        Token(Is),
        Token(Ident, "String"),
        Token(Comma),
        Token(String, "string_val should be String"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "bool_val"),
        Token(Is),
        Token(Ident, "Bool"),
        Token(Comma),
        Token(String, "bool_val should be Bool"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "list_val"),
        Token(Is),
        Token(Ident, "List"),
        Token(Comma),
        Token(String, "list_val should be List"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "dict_val"),
        Token(Is),
        Token(Ident, "Dict"),
        Token(Comma),
        Token(String, "dict_val should be Dict"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "range_val"),
        Token(Is),
        Token(Ident, "Range"),
        Token(Comma),
        Token(String, "range_val should be Range"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "lambda_val"),
        Token(Is),
        Token(Ident, "Function"),
        Token(Comma),
        Token(String, "lambda_val should be Function"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "int_val"),
        Token(IsNot),
        Token(Ident, "String"),
        Token(Comma),
        Token(String, "int_val should not be String"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "float_val"),
        Token(IsNot),
        Token(Ident, "Int"),
        Token(Comma),
        Token(String, "float_val should not be Int"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "string_val"),
        Token(IsNot),
        Token(Ident, "Bool"),
        Token(Comma),
        Token(String, "string_val should not be Bool"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "bool_val"),
        Token(IsNot),
        Token(Ident, "List"),
        Token(Comma),
        Token(String, "bool_val should not be List"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "list_val"),
        Token(IsNot),
        Token(Ident, "Dict"),
        Token(Comma),
        Token(String, "list_val should not be Dict"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "dict_val"),
        Token(IsNot),
        Token(Ident, "Range"),
        Token(Comma),
        Token(String, "dict_val should not be Range"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "range_val"),
        Token(IsNot),
        Token(Ident, "Function"),
        Token(Comma),
        Token(String, "range_val should not be Function"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "lambda_val"),
        Token(IsNot),
        Token(Ident, "Int"),
        Token(Comma),
        Token(String, "lambda_val should not be Int"),
        Token(RightParen),
        Token(Semicolon),
        Token(Fn),
        Token(Ident, "f"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(RightBrace),
        Token(Var),
        Token(Ident, "unit"),
        Token(Equal),
        Token(Ident, "f"),
        Token(LeftParen),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Null),
        Token(Is),
        Token(Ident, "Null"),
        Token(Comma),
        Token(String, "null should be Null"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "unit"),
        Token(Is),
        Token(Ident, "Unit"),
        Token(Comma),
        Token(String, "unit should be Unit"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Null),
        Token(IsNot),
        Token(Ident, "Unit"),
        Token(Comma),
        Token(String, "null should not be Unit"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "unit"),
        Token(IsNot),
        Token(Ident, "Null"),
        Token(Comma),
        Token(String, "unit should not be Null"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "list1"),
        Token(Equal),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightBracket),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "list2"),
        Token(Equal),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightBracket),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "list3"),
        Token(Equal),
        Token(Ident, "list1"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "list1"),
        Token(IsNot),
        Token(Ident, "list2"),
        Token(Comma),
        Token(String, "list1 should not be identical to list2 (different objects)"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "list1"),
        Token(Is),
        Token(Ident, "list3"),
        Token(Comma),
        Token(String, "list1 should be identical to list3 (same object reference)"),
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