# Program 🟢
```rustleaf
// Test basic arithmetic
assert(2 + 3 == 5, "Addition should work");
assert(10 - 4 == 6, "Subtraction should work");  
assert(3 * 4 == 12, "Multiplication should work");

// Test unary operations
assert(-5 == 0 - 5, "Negation should work");
assert(!false == true, "Logical not should work");
assert(~0 == -1, "Bitwise not should work");

// Test variables and scoping
var x = 100;
assert(x == 100, "Variable assignment should work");

{
    var x = 200;
    assert(x == 200, "Block scoping should work");
}
assert(x == 100, "Outer scope should be restored");

// Test functions
fn add(a, b) a + b
assert(add(7, 8) == 15, "Function calls should work");

// Test block expressions  
var result = {
    var temp = 5;
    temp * 2
};
assert(result == 10, "Block expressions should return values");

print("All comprehensive tests passed!");
```

# Output
```
String("All comprehensive tests passed!")
```

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
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "2"),
        Token(Plus),
        Token(Int, "3"),
        Token(EqualEqual),
        Token(Int, "5"),
        Token(Comma),
        Token(String, "Addition should work"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "10"),
        Token(Minus),
        Token(Int, "4"),
        Token(EqualEqual),
        Token(Int, "6"),
        Token(Comma),
        Token(String, "Subtraction should work"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Int, "3"),
        Token(Star),
        Token(Int, "4"),
        Token(EqualEqual),
        Token(Int, "12"),
        Token(Comma),
        Token(String, "Multiplication should work"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Minus),
        Token(Int, "5"),
        Token(EqualEqual),
        Token(Int, "0"),
        Token(Minus),
        Token(Int, "5"),
        Token(Comma),
        Token(String, "Negation should work"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Bang),
        Token(False),
        Token(EqualEqual),
        Token(True),
        Token(Comma),
        Token(String, "Logical not should work"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Tilde),
        Token(Int, "0"),
        Token(EqualEqual),
        Token(Minus),
        Token(Int, "1"),
        Token(Comma),
        Token(String, "Bitwise not should work"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "100"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(EqualEqual),
        Token(Int, "100"),
        Token(Comma),
        Token(String, "Variable assignment should work"),
        Token(RightParen),
        Token(Semicolon),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "200"),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(EqualEqual),
        Token(Int, "200"),
        Token(Comma),
        Token(String, "Block scoping should work"),
        Token(RightParen),
        Token(Semicolon),
        Token(RightBrace),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(EqualEqual),
        Token(Int, "100"),
        Token(Comma),
        Token(String, "Outer scope should be restored"),
        Token(RightParen),
        Token(Semicolon),
        Token(Fn),
        Token(Ident, "add"),
        Token(LeftParen),
        Token(Ident, "a"),
        Token(Comma),
        Token(Ident, "b"),
        Token(RightParen),
        Token(Ident, "a"),
        Token(Plus),
        Token(Ident, "b"),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "add"),
        Token(LeftParen),
        Token(Int, "7"),
        Token(Comma),
        Token(Int, "8"),
        Token(RightParen),
        Token(EqualEqual),
        Token(Int, "15"),
        Token(Comma),
        Token(String, "Function calls should work"),
        Token(RightParen),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result"),
        Token(Equal),
        Token(LeftBrace),
        Token(Var),
        Token(Ident, "temp"),
        Token(Equal),
        Token(Int, "5"),
        Token(Semicolon),
        Token(Ident, "temp"),
        Token(Star),
        Token(Int, "2"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result"),
        Token(EqualEqual),
        Token(Int, "10"),
        Token(Comma),
        Token(String, "Block expressions should return values"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "print"),
        Token(LeftParen),
        Token(String, "All comprehensive tests passed!"),
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
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Add(
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
                            ),
                            Literal(
                                Int(
                                    5,
                                ),
                            ),
                        ),
                        Literal(
                            String(
                                "Addition should work",
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
                        Eq(
                            Sub(
                                Literal(
                                    Int(
                                        10,
                                    ),
                                ),
                                Literal(
                                    Int(
                                        4,
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    6,
                                ),
                            ),
                        ),
                        Literal(
                            String(
                                "Subtraction should work",
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
                        Eq(
                            Mul(
                                Literal(
                                    Int(
                                        3,
                                    ),
                                ),
                                Literal(
                                    Int(
                                        4,
                                    ),
                                ),
                            ),
                            Literal(
                                Int(
                                    12,
                                ),
                            ),
                        ),
                        Literal(
                            String(
                                "Multiplication should work",
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
                        Eq(
                            Neg(
                                Literal(
                                    Int(
                                        5,
                                    ),
                                ),
                            ),
                            Sub(
                                Literal(
                                    Int(
                                        0,
                                    ),
                                ),
                                Literal(
                                    Int(
                                        5,
                                    ),
                                ),
                            ),
                        ),
                        Literal(
                            String(
                                "Negation should work",
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
                        Eq(
                            Not(
                                Literal(
                                    Bool(
                                        false,
                                    ),
                                ),
                            ),
                            Literal(
                                Bool(
                                    true,
                                ),
                            ),
                        ),
                        Literal(
                            String(
                                "Logical not should work",
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
                        Eq(
                            BitNot(
                                Literal(
                                    Int(
                                        0,
                                    ),
                                ),
                            ),
                            Neg(
                                Literal(
                                    Int(
                                        1,
                                    ),
                                ),
                            ),
                        ),
                        Literal(
                            String(
                                "Bitwise not should work",
                            ),
                        ),
                    ],
                ),
            ),
            VarDecl {
                pattern: Variable(
                    "x",
                ),
                value: Some(
                    Literal(
                        Int(
                            100,
                        ),
                    ),
                ),
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Identifier(
                                "x",
                            ),
                            Literal(
                                Int(
                                    100,
                                ),
                            ),
                        ),
                        Literal(
                            String(
                                "Variable assignment should work",
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                Block(
                    Block {
                        statements: [
                            VarDecl {
                                pattern: Variable(
                                    "x",
                                ),
                                value: Some(
                                    Literal(
                                        Int(
                                            200,
                                        ),
                                    ),
                                ),
                            },
                            Expression(
                                FunctionCall(
                                    Identifier(
                                        "assert",
                                    ),
                                    [
                                        Eq(
                                            Identifier(
                                                "x",
                                            ),
                                            Literal(
                                                Int(
                                                    200,
                                                ),
                                            ),
                                        ),
                                        Literal(
                                            String(
                                                "Block scoping should work",
                                            ),
                                        ),
                                    ],
                                ),
                            ),
                        ],
                        final_expr: None,
                    },
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Identifier(
                                "x",
                            ),
                            Literal(
                                Int(
                                    100,
                                ),
                            ),
                        ),
                        Literal(
                            String(
                                "Outer scope should be restored",
                            ),
                        ),
                    ],
                ),
            ),
            FnDecl {
                name: "add",
                params: [
                    Parameter {
                        name: "a",
                        default: None,
                        kind: Regular,
                    },
                    Parameter {
                        name: "b",
                        default: None,
                        kind: Regular,
                    },
                ],
                body: Block {
                    statements: [],
                    final_expr: Some(
                        Add(
                            Identifier(
                                "a",
                            ),
                            Identifier(
                                "b",
                            ),
                        ),
                    ),
                },
                is_pub: false,
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            FunctionCall(
                                Identifier(
                                    "add",
                                ),
                                [
                                    Literal(
                                        Int(
                                            7,
                                        ),
                                    ),
                                    Literal(
                                        Int(
                                            8,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                Int(
                                    15,
                                ),
                            ),
                        ),
                        Literal(
                            String(
                                "Function calls should work",
                            ),
                        ),
                    ],
                ),
            ),
            VarDecl {
                pattern: Variable(
                    "result",
                ),
                value: Some(
                    Block(
                        Block {
                            statements: [
                                VarDecl {
                                    pattern: Variable(
                                        "temp",
                                    ),
                                    value: Some(
                                        Literal(
                                            Int(
                                                5,
                                            ),
                                        ),
                                    ),
                                },
                            ],
                            final_expr: Some(
                                Mul(
                                    Identifier(
                                        "temp",
                                    ),
                                    Literal(
                                        Int(
                                            2,
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
            },
            Expression(
                FunctionCall(
                    Identifier(
                        "assert",
                    ),
                    [
                        Eq(
                            Identifier(
                                "result",
                            ),
                            Literal(
                                Int(
                                    10,
                                ),
                            ),
                        ),
                        Literal(
                            String(
                                "Block expressions should return values",
                            ),
                        ),
                    ],
                ),
            ),
            Expression(
                FunctionCall(
                    Identifier(
                        "print",
                    ),
                    [
                        Literal(
                            String(
                                "All comprehensive tests passed!",
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
    Block(
        [
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Call(
                                GetAttr(
                                    Literal(
                                        Int(
                                            2,
                                        ),
                                    ),
                                    "op_add",
                                ),
                                [
                                    Literal(
                                        Int(
                                            3,
                                        ),
                                    ),
                                ],
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    5,
                                ),
                            ),
                        ],
                    ),
                    Literal(
                        String(
                            "Addition should work",
                        ),
                    ),
                ],
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Call(
                                GetAttr(
                                    Literal(
                                        Int(
                                            10,
                                        ),
                                    ),
                                    "op_sub",
                                ),
                                [
                                    Literal(
                                        Int(
                                            4,
                                        ),
                                    ),
                                ],
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    6,
                                ),
                            ),
                        ],
                    ),
                    Literal(
                        String(
                            "Subtraction should work",
                        ),
                    ),
                ],
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Call(
                                GetAttr(
                                    Literal(
                                        Int(
                                            3,
                                        ),
                                    ),
                                    "op_mul",
                                ),
                                [
                                    Literal(
                                        Int(
                                            4,
                                        ),
                                    ),
                                ],
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    12,
                                ),
                            ),
                        ],
                    ),
                    Literal(
                        String(
                            "Multiplication should work",
                        ),
                    ),
                ],
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Call(
                                GetAttr(
                                    Literal(
                                        Int(
                                            5,
                                        ),
                                    ),
                                    "op_neg",
                                ),
                                [],
                            ),
                            "op_eq",
                        ),
                        [
                            Call(
                                GetAttr(
                                    Literal(
                                        Int(
                                            0,
                                        ),
                                    ),
                                    "op_sub",
                                ),
                                [
                                    Literal(
                                        Int(
                                            5,
                                        ),
                                    ),
                                ],
                            ),
                        ],
                    ),
                    Literal(
                        String(
                            "Negation should work",
                        ),
                    ),
                ],
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            LogicalNot(
                                Literal(
                                    Bool(
                                        false,
                                    ),
                                ),
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Bool(
                                    true,
                                ),
                            ),
                        ],
                    ),
                    Literal(
                        String(
                            "Logical not should work",
                        ),
                    ),
                ],
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Call(
                                GetAttr(
                                    Literal(
                                        Int(
                                            0,
                                        ),
                                    ),
                                    "op_bitwise_not",
                                ),
                                [],
                            ),
                            "op_eq",
                        ),
                        [
                            Call(
                                GetAttr(
                                    Literal(
                                        Int(
                                            1,
                                        ),
                                    ),
                                    "op_neg",
                                ),
                                [],
                            ),
                        ],
                    ),
                    Literal(
                        String(
                            "Bitwise not should work",
                        ),
                    ),
                ],
            ),
            Declare(
                "x",
                Some(
                    Literal(
                        Int(
                            100,
                        ),
                    ),
                ),
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Variable(
                                "x",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    100,
                                ),
                            ),
                        ],
                    ),
                    Literal(
                        String(
                            "Variable assignment should work",
                        ),
                    ),
                ],
            ),
            Block(
                [
                    Declare(
                        "x",
                        Some(
                            Literal(
                                Int(
                                    200,
                                ),
                            ),
                        ),
                    ),
                    Call(
                        Variable(
                            "assert",
                        ),
                        [
                            Call(
                                GetAttr(
                                    Variable(
                                        "x",
                                    ),
                                    "op_eq",
                                ),
                                [
                                    Literal(
                                        Int(
                                            200,
                                        ),
                                    ),
                                ],
                            ),
                            Literal(
                                String(
                                    "Block scoping should work",
                                ),
                            ),
                        ],
                    ),
                ],
                None,
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Variable(
                                "x",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    100,
                                ),
                            ),
                        ],
                    ),
                    Literal(
                        String(
                            "Outer scope should be restored",
                        ),
                    ),
                ],
            ),
            Function(
                "add",
                [
                    "a",
                    "b",
                ],
                Block(
                    [],
                    Some(
                        Call(
                            GetAttr(
                                Variable(
                                    "a",
                                ),
                                "op_add",
                            ),
                            [
                                Variable(
                                    "b",
                                ),
                            ],
                        ),
                    ),
                ),
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Call(
                                Variable(
                                    "add",
                                ),
                                [
                                    Literal(
                                        Int(
                                            7,
                                        ),
                                    ),
                                    Literal(
                                        Int(
                                            8,
                                        ),
                                    ),
                                ],
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    15,
                                ),
                            ),
                        ],
                    ),
                    Literal(
                        String(
                            "Function calls should work",
                        ),
                    ),
                ],
            ),
            Declare(
                "result",
                Some(
                    Block(
                        [
                            Declare(
                                "temp",
                                Some(
                                    Literal(
                                        Int(
                                            5,
                                        ),
                                    ),
                                ),
                            ),
                        ],
                        Some(
                            Call(
                                GetAttr(
                                    Variable(
                                        "temp",
                                    ),
                                    "op_mul",
                                ),
                                [
                                    Literal(
                                        Int(
                                            2,
                                        ),
                                    ),
                                ],
                            ),
                        ),
                    ),
                ),
            ),
            Call(
                Variable(
                    "assert",
                ),
                [
                    Call(
                        GetAttr(
                            Variable(
                                "result",
                            ),
                            "op_eq",
                        ),
                        [
                            Literal(
                                Int(
                                    10,
                                ),
                            ),
                        ],
                    ),
                    Literal(
                        String(
                            "Block expressions should return values",
                        ),
                    ),
                ],
            ),
        ],
        Some(
            Call(
                Variable(
                    "print",
                ),
                [
                    Literal(
                        String(
                            "All comprehensive tests passed!",
                        ),
                    ),
                ],
            ),
        ),
    ),
)
```