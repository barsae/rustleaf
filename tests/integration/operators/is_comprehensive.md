# Program
Status: 🟢
Assertions: 22

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
None

# Result
```rust
Ok(
    Unit,
)
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
Ok(
    Program(
        [
            VarDecl {
                pattern: Variable(
                    "int_val",
                ),
                value: Some(
                    Literal(
                        Int(
                            42,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "float_val",
                ),
                value: Some(
                    Literal(
                        Float(
                            3.14,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "string_val",
                ),
                value: Some(
                    Literal(
                        String(
                            "hello",
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "bool_val",
                ),
                value: Some(
                    Literal(
                        Bool(
                            true,
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "list_val",
                ),
                value: Some(
                    List(
                        [
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                            Literal(
                                Int(
                                    2,
                                ),
                            ),
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "dict_val",
                ),
                value: Some(
                    Dict(
                        [
                            (
                                Literal(
                                    String(
                                        "key",
                                    ),
                                ),
                                Literal(
                                    String(
                                        "value",
                                    ),
                                ),
                            ),
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "range_val",
                ),
                value: Some(
                    RangeExclusive(
                        Literal(
                            Int(
                                1,
                            ),
                        ),
                        Literal(
                            Int(
                                10,
                            ),
                        ),
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "lambda_val",
                ),
                value: Some(
                    Lambda {
                        params: [
                            "x",
                        ],
                        body: Expression(
                            Add(
                                Identifier(
                                    "x",
                                ),
                                Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Is(
                            Identifier(
                                "int_val",
                            ),
                            Identifier(
                                "Int",
                            ),
                        ),
                        Literal(
                            String(
                                "int_val should be Int",
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Is(
                            Identifier(
                                "float_val",
                            ),
                            Identifier(
                                "Float",
                            ),
                        ),
                        Literal(
                            String(
                                "float_val should be Float",
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Is(
                            Identifier(
                                "string_val",
                            ),
                            Identifier(
                                "String",
                            ),
                        ),
                        Literal(
                            String(
                                "string_val should be String",
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Is(
                            Identifier(
                                "bool_val",
                            ),
                            Identifier(
                                "Bool",
                            ),
                        ),
                        Literal(
                            String(
                                "bool_val should be Bool",
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Is(
                            Identifier(
                                "list_val",
                            ),
                            Identifier(
                                "List",
                            ),
                        ),
                        Literal(
                            String(
                                "list_val should be List",
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Is(
                            Identifier(
                                "dict_val",
                            ),
                            Identifier(
                                "Dict",
                            ),
                        ),
                        Literal(
                            String(
                                "dict_val should be Dict",
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Is(
                            Identifier(
                                "range_val",
                            ),
                            Identifier(
                                "Range",
                            ),
                        ),
                        Literal(
                            String(
                                "range_val should be Range",
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Is(
                            Identifier(
                                "lambda_val",
                            ),
                            Identifier(
                                "Function",
                            ),
                        ),
                        Literal(
                            String(
                                "lambda_val should be Function",
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        IsNot(
                            Identifier(
                                "int_val",
                            ),
                            Identifier(
                                "String",
                            ),
                        ),
                        Literal(
                            String(
                                "int_val should not be String",
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        IsNot(
                            Identifier(
                                "float_val",
                            ),
                            Identifier(
                                "Int",
                            ),
                        ),
                        Literal(
                            String(
                                "float_val should not be Int",
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        IsNot(
                            Identifier(
                                "string_val",
                            ),
                            Identifier(
                                "Bool",
                            ),
                        ),
                        Literal(
                            String(
                                "string_val should not be Bool",
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        IsNot(
                            Identifier(
                                "bool_val",
                            ),
                            Identifier(
                                "List",
                            ),
                        ),
                        Literal(
                            String(
                                "bool_val should not be List",
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        IsNot(
                            Identifier(
                                "list_val",
                            ),
                            Identifier(
                                "Dict",
                            ),
                        ),
                        Literal(
                            String(
                                "list_val should not be Dict",
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        IsNot(
                            Identifier(
                                "dict_val",
                            ),
                            Identifier(
                                "Range",
                            ),
                        ),
                        Literal(
                            String(
                                "dict_val should not be Range",
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        IsNot(
                            Identifier(
                                "range_val",
                            ),
                            Identifier(
                                "Function",
                            ),
                        ),
                        Literal(
                            String(
                                "range_val should not be Function",
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        IsNot(
                            Identifier(
                                "lambda_val",
                            ),
                            Identifier(
                                "Int",
                            ),
                        ),
                        Literal(
                            String(
                                "lambda_val should not be Int",
                            ),
                        ),
                    ],
                ),
            ),
            FnDecl {
                name: "f",
                params: [],
                body: Block {
                    statements: [],
                    final_expr: None,
                },
                is_pub: false,
            },
            VarDecl {
                pattern: Variable(
                    "unit",
                ),
                value: Some(
                    FunctionCall(
                        Identifier(
                            "f",
                        ),
                        [],
                    ),
                ),
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Is(
                            Literal(
                                Null,
                            ),
                            Identifier(
                                "Null",
                            ),
                        ),
                        Literal(
                            String(
                                "null should be Null",
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Is(
                            Identifier(
                                "unit",
                            ),
                            Identifier(
                                "Unit",
                            ),
                        ),
                        Literal(
                            String(
                                "unit should be Unit",
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        IsNot(
                            Literal(
                                Null,
                            ),
                            Identifier(
                                "Unit",
                            ),
                        ),
                        Literal(
                            String(
                                "null should not be Unit",
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        IsNot(
                            Identifier(
                                "unit",
                            ),
                            Identifier(
                                "Null",
                            ),
                        ),
                        Literal(
                            String(
                                "unit should not be Null",
                            ),
                        ),
                    ],
                ),
            ),
            VarDecl {
                pattern: Variable(
                    "list1",
                ),
                value: Some(
                    List(
                        [
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                            Literal(
                                Int(
                                    2,
                                ),
                            ),
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "list2",
                ),
                value: Some(
                    List(
                        [
                            Literal(
                                Int(
                                    1,
                                ),
                            ),
                            Literal(
                                Int(
                                    2,
                                ),
                            ),
                            Literal(
                                Int(
                                    3,
                                ),
                            ),
                        ],
                    ),
                ),
            },
            VarDecl {
                pattern: Variable(
                    "list3",
                ),
                value: Some(
                    Identifier(
                        "list1",
                    ),
                ),
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        IsNot(
                            Identifier(
                                "list1",
                            ),
                            Identifier(
                                "list2",
                            ),
                        ),
                        Literal(
                            String(
                                "list1 should not be identical to list2 (different objects)",
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Is(
                            Identifier(
                                "list1",
                            ),
                            Identifier(
                                "list3",
                            ),
                        ),
                        Literal(
                            String(
                                "list1 should be identical to list3 (same object reference)",
                            ),
                        ),
                    ],
                ),
            ),
        ],
    ),
)
```

# Eval
```rust
Ok(
    Eval(
        EvalRef(
            RefCell {
                value: EvalProgram {
                    statements: [
                        EvalRef(
                            RefCell {
                                value: EvalDeclare {
                                    name: "int_val",
                                    init_expr: Some(
                                        EvalRef(
                                            RefCell {
                                                value: EvalLiteral {
                                                    value: Int(
                                                        42,
                                                    ),
                                                },
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalDeclare {
                                    name: "float_val",
                                    init_expr: Some(
                                        EvalRef(
                                            RefCell {
                                                value: EvalLiteral {
                                                    value: Float(
                                                        3.14,
                                                    ),
                                                },
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalDeclare {
                                    name: "string_val",
                                    init_expr: Some(
                                        EvalRef(
                                            RefCell {
                                                value: EvalLiteral {
                                                    value: String(
                                                        "hello",
                                                    ),
                                                },
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalDeclare {
                                    name: "bool_val",
                                    init_expr: Some(
                                        EvalRef(
                                            RefCell {
                                                value: EvalLiteral {
                                                    value: Bool(
                                                        true,
                                                    ),
                                                },
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalDeclare {
                                    name: "list_val",
                                    init_expr: Some(
                                        EvalRef(
                                            RefCell {
                                                value: EvalList {
                                                    elements: [
                                                        EvalRef(
                                                            RefCell {
                                                                value: EvalLiteral {
                                                                    value: Int(
                                                                        1,
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                        EvalRef(
                                                            RefCell {
                                                                value: EvalLiteral {
                                                                    value: Int(
                                                                        2,
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                        EvalRef(
                                                            RefCell {
                                                                value: EvalLiteral {
                                                                    value: Int(
                                                                        3,
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                },
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalDeclare {
                                    name: "dict_val",
                                    init_expr: Some(
                                        EvalRef(
                                            RefCell {
                                                value: EvalDict {
                                                    pairs: [
                                                        (
                                                            EvalRef(
                                                                RefCell {
                                                                    value: EvalLiteral {
                                                                        value: String(
                                                                            "key",
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                            EvalRef(
                                                                RefCell {
                                                                    value: EvalLiteral {
                                                                        value: String(
                                                                            "value",
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                },
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalDeclare {
                                    name: "range_val",
                                    init_expr: Some(
                                        EvalRef(
                                            RefCell {
                                                value: EvalLiteral {
                                                    value: Range(
                                                        Range {
                                                            start: 1,
                                                            end: 10,
                                                            inclusive: false,
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalDeclare {
                                    name: "lambda_val",
                                    init_expr: Some(
                                        EvalRef(
                                            RefCell {
                                                value: EvalLambda {
                                                    data: LambdaData {
                                                        params: [
                                                            "x",
                                                        ],
                                                        body: Eval(
                                                            EvalRef(
                                                                RefCell {
                                                                    value: EvalCall {
                                                                        func_expr: EvalRef(
                                                                            RefCell {
                                                                                value: EvalGetAttr {
                                                                                    obj_expr: EvalRef(
                                                                                        RefCell {
                                                                                            value: EvalVariable {
                                                                                                name: "x",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    attr_name: "op_add",
                                                                                },
                                                                            },
                                                                        ),
                                                                        args: [
                                                                            EvalRef(
                                                                                RefCell {
                                                                                    value: EvalLiteral {
                                                                                        value: Int(
                                                                                            1,
                                                                                        ),
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                },
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalCall {
                                    func_expr: EvalRef(
                                        RefCell {
                                            value: EvalVariable {
                                                name: "assert",
                                            },
                                        },
                                    ),
                                    args: [
                                        EvalRef(
                                            RefCell {
                                                value: EvalIs {
                                                    left: EvalRef(
                                                        RefCell {
                                                            value: EvalVariable {
                                                                name: "int_val",
                                                            },
                                                        },
                                                    ),
                                                    right: EvalRef(
                                                        RefCell {
                                                            value: EvalVariable {
                                                                name: "Int",
                                                            },
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                        EvalRef(
                                            RefCell {
                                                value: EvalLiteral {
                                                    value: String(
                                                        "int_val should be Int",
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalCall {
                                    func_expr: EvalRef(
                                        RefCell {
                                            value: EvalVariable {
                                                name: "assert",
                                            },
                                        },
                                    ),
                                    args: [
                                        EvalRef(
                                            RefCell {
                                                value: EvalIs {
                                                    left: EvalRef(
                                                        RefCell {
                                                            value: EvalVariable {
                                                                name: "float_val",
                                                            },
                                                        },
                                                    ),
                                                    right: EvalRef(
                                                        RefCell {
                                                            value: EvalVariable {
                                                                name: "Float",
                                                            },
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                        EvalRef(
                                            RefCell {
                                                value: EvalLiteral {
                                                    value: String(
                                                        "float_val should be Float",
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalCall {
                                    func_expr: EvalRef(
                                        RefCell {
                                            value: EvalVariable {
                                                name: "assert",
                                            },
                                        },
                                    ),
                                    args: [
                                        EvalRef(
                                            RefCell {
                                                value: EvalIs {
                                                    left: EvalRef(
                                                        RefCell {
                                                            value: EvalVariable {
                                                                name: "string_val",
                                                            },
                                                        },
                                                    ),
                                                    right: EvalRef(
                                                        RefCell {
                                                            value: EvalVariable {
                                                                name: "String",
                                                            },
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                        EvalRef(
                                            RefCell {
                                                value: EvalLiteral {
                                                    value: String(
                                                        "string_val should be String",
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalCall {
                                    func_expr: EvalRef(
                                        RefCell {
                                            value: EvalVariable {
                                                name: "assert",
                                            },
                                        },
                                    ),
                                    args: [
                                        EvalRef(
                                            RefCell {
                                                value: EvalIs {
                                                    left: EvalRef(
                                                        RefCell {
                                                            value: EvalVariable {
                                                                name: "bool_val",
                                                            },
                                                        },
                                                    ),
                                                    right: EvalRef(
                                                        RefCell {
                                                            value: EvalVariable {
                                                                name: "Bool",
                                                            },
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                        EvalRef(
                                            RefCell {
                                                value: EvalLiteral {
                                                    value: String(
                                                        "bool_val should be Bool",
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalCall {
                                    func_expr: EvalRef(
                                        RefCell {
                                            value: EvalVariable {
                                                name: "assert",
                                            },
                                        },
                                    ),
                                    args: [
                                        EvalRef(
                                            RefCell {
                                                value: EvalIs {
                                                    left: EvalRef(
                                                        RefCell {
                                                            value: EvalVariable {
                                                                name: "list_val",
                                                            },
                                                        },
                                                    ),
                                                    right: EvalRef(
                                                        RefCell {
                                                            value: EvalVariable {
                                                                name: "List",
                                                            },
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                        EvalRef(
                                            RefCell {
                                                value: EvalLiteral {
                                                    value: String(
                                                        "list_val should be List",
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalCall {
                                    func_expr: EvalRef(
                                        RefCell {
                                            value: EvalVariable {
                                                name: "assert",
                                            },
                                        },
                                    ),
                                    args: [
                                        EvalRef(
                                            RefCell {
                                                value: EvalIs {
                                                    left: EvalRef(
                                                        RefCell {
                                                            value: EvalVariable {
                                                                name: "dict_val",
                                                            },
                                                        },
                                                    ),
                                                    right: EvalRef(
                                                        RefCell {
                                                            value: EvalVariable {
                                                                name: "Dict",
                                                            },
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                        EvalRef(
                                            RefCell {
                                                value: EvalLiteral {
                                                    value: String(
                                                        "dict_val should be Dict",
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalCall {
                                    func_expr: EvalRef(
                                        RefCell {
                                            value: EvalVariable {
                                                name: "assert",
                                            },
                                        },
                                    ),
                                    args: [
                                        EvalRef(
                                            RefCell {
                                                value: EvalIs {
                                                    left: EvalRef(
                                                        RefCell {
                                                            value: EvalVariable {
                                                                name: "range_val",
                                                            },
                                                        },
                                                    ),
                                                    right: EvalRef(
                                                        RefCell {
                                                            value: EvalVariable {
                                                                name: "Range",
                                                            },
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                        EvalRef(
                                            RefCell {
                                                value: EvalLiteral {
                                                    value: String(
                                                        "range_val should be Range",
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalCall {
                                    func_expr: EvalRef(
                                        RefCell {
                                            value: EvalVariable {
                                                name: "assert",
                                            },
                                        },
                                    ),
                                    args: [
                                        EvalRef(
                                            RefCell {
                                                value: EvalIs {
                                                    left: EvalRef(
                                                        RefCell {
                                                            value: EvalVariable {
                                                                name: "lambda_val",
                                                            },
                                                        },
                                                    ),
                                                    right: EvalRef(
                                                        RefCell {
                                                            value: EvalVariable {
                                                                name: "Function",
                                                            },
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                        EvalRef(
                                            RefCell {
                                                value: EvalLiteral {
                                                    value: String(
                                                        "lambda_val should be Function",
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalCall {
                                    func_expr: EvalRef(
                                        RefCell {
                                            value: EvalVariable {
                                                name: "assert",
                                            },
                                        },
                                    ),
                                    args: [
                                        EvalRef(
                                            RefCell {
                                                value: EvalLogicalNot {
                                                    expr: EvalRef(
                                                        RefCell {
                                                            value: EvalIs {
                                                                left: EvalRef(
                                                                    RefCell {
                                                                        value: EvalVariable {
                                                                            name: "int_val",
                                                                        },
                                                                    },
                                                                ),
                                                                right: EvalRef(
                                                                    RefCell {
                                                                        value: EvalVariable {
                                                                            name: "String",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                        EvalRef(
                                            RefCell {
                                                value: EvalLiteral {
                                                    value: String(
                                                        "int_val should not be String",
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalCall {
                                    func_expr: EvalRef(
                                        RefCell {
                                            value: EvalVariable {
                                                name: "assert",
                                            },
                                        },
                                    ),
                                    args: [
                                        EvalRef(
                                            RefCell {
                                                value: EvalLogicalNot {
                                                    expr: EvalRef(
                                                        RefCell {
                                                            value: EvalIs {
                                                                left: EvalRef(
                                                                    RefCell {
                                                                        value: EvalVariable {
                                                                            name: "float_val",
                                                                        },
                                                                    },
                                                                ),
                                                                right: EvalRef(
                                                                    RefCell {
                                                                        value: EvalVariable {
                                                                            name: "Int",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                        EvalRef(
                                            RefCell {
                                                value: EvalLiteral {
                                                    value: String(
                                                        "float_val should not be Int",
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalCall {
                                    func_expr: EvalRef(
                                        RefCell {
                                            value: EvalVariable {
                                                name: "assert",
                                            },
                                        },
                                    ),
                                    args: [
                                        EvalRef(
                                            RefCell {
                                                value: EvalLogicalNot {
                                                    expr: EvalRef(
                                                        RefCell {
                                                            value: EvalIs {
                                                                left: EvalRef(
                                                                    RefCell {
                                                                        value: EvalVariable {
                                                                            name: "string_val",
                                                                        },
                                                                    },
                                                                ),
                                                                right: EvalRef(
                                                                    RefCell {
                                                                        value: EvalVariable {
                                                                            name: "Bool",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                        EvalRef(
                                            RefCell {
                                                value: EvalLiteral {
                                                    value: String(
                                                        "string_val should not be Bool",
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalCall {
                                    func_expr: EvalRef(
                                        RefCell {
                                            value: EvalVariable {
                                                name: "assert",
                                            },
                                        },
                                    ),
                                    args: [
                                        EvalRef(
                                            RefCell {
                                                value: EvalLogicalNot {
                                                    expr: EvalRef(
                                                        RefCell {
                                                            value: EvalIs {
                                                                left: EvalRef(
                                                                    RefCell {
                                                                        value: EvalVariable {
                                                                            name: "bool_val",
                                                                        },
                                                                    },
                                                                ),
                                                                right: EvalRef(
                                                                    RefCell {
                                                                        value: EvalVariable {
                                                                            name: "List",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                        EvalRef(
                                            RefCell {
                                                value: EvalLiteral {
                                                    value: String(
                                                        "bool_val should not be List",
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalCall {
                                    func_expr: EvalRef(
                                        RefCell {
                                            value: EvalVariable {
                                                name: "assert",
                                            },
                                        },
                                    ),
                                    args: [
                                        EvalRef(
                                            RefCell {
                                                value: EvalLogicalNot {
                                                    expr: EvalRef(
                                                        RefCell {
                                                            value: EvalIs {
                                                                left: EvalRef(
                                                                    RefCell {
                                                                        value: EvalVariable {
                                                                            name: "list_val",
                                                                        },
                                                                    },
                                                                ),
                                                                right: EvalRef(
                                                                    RefCell {
                                                                        value: EvalVariable {
                                                                            name: "Dict",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                        EvalRef(
                                            RefCell {
                                                value: EvalLiteral {
                                                    value: String(
                                                        "list_val should not be Dict",
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalCall {
                                    func_expr: EvalRef(
                                        RefCell {
                                            value: EvalVariable {
                                                name: "assert",
                                            },
                                        },
                                    ),
                                    args: [
                                        EvalRef(
                                            RefCell {
                                                value: EvalLogicalNot {
                                                    expr: EvalRef(
                                                        RefCell {
                                                            value: EvalIs {
                                                                left: EvalRef(
                                                                    RefCell {
                                                                        value: EvalVariable {
                                                                            name: "dict_val",
                                                                        },
                                                                    },
                                                                ),
                                                                right: EvalRef(
                                                                    RefCell {
                                                                        value: EvalVariable {
                                                                            name: "Range",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                        EvalRef(
                                            RefCell {
                                                value: EvalLiteral {
                                                    value: String(
                                                        "dict_val should not be Range",
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalCall {
                                    func_expr: EvalRef(
                                        RefCell {
                                            value: EvalVariable {
                                                name: "assert",
                                            },
                                        },
                                    ),
                                    args: [
                                        EvalRef(
                                            RefCell {
                                                value: EvalLogicalNot {
                                                    expr: EvalRef(
                                                        RefCell {
                                                            value: EvalIs {
                                                                left: EvalRef(
                                                                    RefCell {
                                                                        value: EvalVariable {
                                                                            name: "range_val",
                                                                        },
                                                                    },
                                                                ),
                                                                right: EvalRef(
                                                                    RefCell {
                                                                        value: EvalVariable {
                                                                            name: "Function",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                        EvalRef(
                                            RefCell {
                                                value: EvalLiteral {
                                                    value: String(
                                                        "range_val should not be Function",
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalCall {
                                    func_expr: EvalRef(
                                        RefCell {
                                            value: EvalVariable {
                                                name: "assert",
                                            },
                                        },
                                    ),
                                    args: [
                                        EvalRef(
                                            RefCell {
                                                value: EvalLogicalNot {
                                                    expr: EvalRef(
                                                        RefCell {
                                                            value: EvalIs {
                                                                left: EvalRef(
                                                                    RefCell {
                                                                        value: EvalVariable {
                                                                            name: "lambda_val",
                                                                        },
                                                                    },
                                                                ),
                                                                right: EvalRef(
                                                                    RefCell {
                                                                        value: EvalVariable {
                                                                            name: "Int",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                        EvalRef(
                                            RefCell {
                                                value: EvalLiteral {
                                                    value: String(
                                                        "lambda_val should not be Int",
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalFunction {
                                    data: FunctionData {
                                        name: "f",
                                        params: [],
                                        body: Eval(
                                            EvalRef(
                                                RefCell {
                                                    value: EvalBlock {
                                                        statements: [],
                                                        final_expr: None,
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalDeclare {
                                    name: "unit",
                                    init_expr: Some(
                                        EvalRef(
                                            RefCell {
                                                value: EvalCall {
                                                    func_expr: EvalRef(
                                                        RefCell {
                                                            value: EvalVariable {
                                                                name: "f",
                                                            },
                                                        },
                                                    ),
                                                    args: [],
                                                },
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalCall {
                                    func_expr: EvalRef(
                                        RefCell {
                                            value: EvalVariable {
                                                name: "assert",
                                            },
                                        },
                                    ),
                                    args: [
                                        EvalRef(
                                            RefCell {
                                                value: EvalIs {
                                                    left: EvalRef(
                                                        RefCell {
                                                            value: EvalLiteral {
                                                                value: Null,
                                                            },
                                                        },
                                                    ),
                                                    right: EvalRef(
                                                        RefCell {
                                                            value: EvalVariable {
                                                                name: "Null",
                                                            },
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                        EvalRef(
                                            RefCell {
                                                value: EvalLiteral {
                                                    value: String(
                                                        "null should be Null",
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalCall {
                                    func_expr: EvalRef(
                                        RefCell {
                                            value: EvalVariable {
                                                name: "assert",
                                            },
                                        },
                                    ),
                                    args: [
                                        EvalRef(
                                            RefCell {
                                                value: EvalIs {
                                                    left: EvalRef(
                                                        RefCell {
                                                            value: EvalVariable {
                                                                name: "unit",
                                                            },
                                                        },
                                                    ),
                                                    right: EvalRef(
                                                        RefCell {
                                                            value: EvalVariable {
                                                                name: "Unit",
                                                            },
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                        EvalRef(
                                            RefCell {
                                                value: EvalLiteral {
                                                    value: String(
                                                        "unit should be Unit",
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalCall {
                                    func_expr: EvalRef(
                                        RefCell {
                                            value: EvalVariable {
                                                name: "assert",
                                            },
                                        },
                                    ),
                                    args: [
                                        EvalRef(
                                            RefCell {
                                                value: EvalLogicalNot {
                                                    expr: EvalRef(
                                                        RefCell {
                                                            value: EvalIs {
                                                                left: EvalRef(
                                                                    RefCell {
                                                                        value: EvalLiteral {
                                                                            value: Null,
                                                                        },
                                                                    },
                                                                ),
                                                                right: EvalRef(
                                                                    RefCell {
                                                                        value: EvalVariable {
                                                                            name: "Unit",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                        EvalRef(
                                            RefCell {
                                                value: EvalLiteral {
                                                    value: String(
                                                        "null should not be Unit",
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalCall {
                                    func_expr: EvalRef(
                                        RefCell {
                                            value: EvalVariable {
                                                name: "assert",
                                            },
                                        },
                                    ),
                                    args: [
                                        EvalRef(
                                            RefCell {
                                                value: EvalLogicalNot {
                                                    expr: EvalRef(
                                                        RefCell {
                                                            value: EvalIs {
                                                                left: EvalRef(
                                                                    RefCell {
                                                                        value: EvalVariable {
                                                                            name: "unit",
                                                                        },
                                                                    },
                                                                ),
                                                                right: EvalRef(
                                                                    RefCell {
                                                                        value: EvalVariable {
                                                                            name: "Null",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                        EvalRef(
                                            RefCell {
                                                value: EvalLiteral {
                                                    value: String(
                                                        "unit should not be Null",
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalDeclare {
                                    name: "list1",
                                    init_expr: Some(
                                        EvalRef(
                                            RefCell {
                                                value: EvalList {
                                                    elements: [
                                                        EvalRef(
                                                            RefCell {
                                                                value: EvalLiteral {
                                                                    value: Int(
                                                                        1,
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                        EvalRef(
                                                            RefCell {
                                                                value: EvalLiteral {
                                                                    value: Int(
                                                                        2,
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                        EvalRef(
                                                            RefCell {
                                                                value: EvalLiteral {
                                                                    value: Int(
                                                                        3,
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                },
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalDeclare {
                                    name: "list2",
                                    init_expr: Some(
                                        EvalRef(
                                            RefCell {
                                                value: EvalList {
                                                    elements: [
                                                        EvalRef(
                                                            RefCell {
                                                                value: EvalLiteral {
                                                                    value: Int(
                                                                        1,
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                        EvalRef(
                                                            RefCell {
                                                                value: EvalLiteral {
                                                                    value: Int(
                                                                        2,
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                        EvalRef(
                                                            RefCell {
                                                                value: EvalLiteral {
                                                                    value: Int(
                                                                        3,
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                },
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalDeclare {
                                    name: "list3",
                                    init_expr: Some(
                                        EvalRef(
                                            RefCell {
                                                value: EvalVariable {
                                                    name: "list1",
                                                },
                                            },
                                        ),
                                    ),
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalCall {
                                    func_expr: EvalRef(
                                        RefCell {
                                            value: EvalVariable {
                                                name: "assert",
                                            },
                                        },
                                    ),
                                    args: [
                                        EvalRef(
                                            RefCell {
                                                value: EvalLogicalNot {
                                                    expr: EvalRef(
                                                        RefCell {
                                                            value: EvalIs {
                                                                left: EvalRef(
                                                                    RefCell {
                                                                        value: EvalVariable {
                                                                            name: "list1",
                                                                        },
                                                                    },
                                                                ),
                                                                right: EvalRef(
                                                                    RefCell {
                                                                        value: EvalVariable {
                                                                            name: "list2",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                        EvalRef(
                                            RefCell {
                                                value: EvalLiteral {
                                                    value: String(
                                                        "list1 should not be identical to list2 (different objects)",
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                },
                            },
                        ),
                        EvalRef(
                            RefCell {
                                value: EvalCall {
                                    func_expr: EvalRef(
                                        RefCell {
                                            value: EvalVariable {
                                                name: "assert",
                                            },
                                        },
                                    ),
                                    args: [
                                        EvalRef(
                                            RefCell {
                                                value: EvalIs {
                                                    left: EvalRef(
                                                        RefCell {
                                                            value: EvalVariable {
                                                                name: "list1",
                                                            },
                                                        },
                                                    ),
                                                    right: EvalRef(
                                                        RefCell {
                                                            value: EvalVariable {
                                                                name: "list3",
                                                            },
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                        EvalRef(
                                            RefCell {
                                                value: EvalLiteral {
                                                    value: String(
                                                        "list1 should be identical to list3 (same object reference)",
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                },
                            },
                        ),
                    ],
                },
            },
        ),
    ),
)
```