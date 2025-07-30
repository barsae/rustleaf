# Parser Refactoring

## Proposed API

```rust
pub struct TokenStream {
    tokens: Vec<Token>,
    current: usize,
}

// No direct token access
impl TokenStream {
    pub fn is_at_end(&self) -> bool;

    // Backtracking parse - returns None if parse fails
    pub fn try_parse<T, F>(&mut self, f: F) -> Result<Option<T>>
        where F: FnOnce(&mut ParseStream) -> Result<T>;
    
    // Non-backtracking parse - for when you're committed to this parse path
    pub fn parse<T, F>(&mut self, f: F) -> Result<T>
        where F: FnOnce(&mut ParseStream) -> Result<T>;
}

// Can consume tokens, but only exists inside try_parse
pub struct ParseStream<'a> {
    stream: &'a mut TokenStream,
    checkpoint: usize,
}

impl<'a> ParseStream<'a> {
    // accept_* methods: Ok(None) = no match, Ok(Some(T)) = matched, Err = error
    pub fn accept_lexeme(&mut self, lexeme: &str) -> Result<Option<Token>>;
    pub fn accept_type(&mut self, token_type: TokenType) -> Result<Option<Token>>;
    
    // expect_* methods: parse-or-fail
    pub fn expect_lexeme(&mut self, lexeme: &str) -> Result<Token>;
    pub fn expect_type(&mut self, token_type: TokenType) -> Result<Token>;
    
    pub fn is_at_end(&self) -> bool;
    
    // Nested speculation
    pub fn try_parse<T, F>(&mut self, f: F) -> Result<Option<T>>
        where F: FnOnce(&mut ParseStream) -> Result<T>;
}
```

## When to Use Each Method

- **`try_parse`**: Use for alternatives, optional elements, or any speculative parsing
- **`parse`**: Use when you're already committed to a parse path (e.g., after matching a keyword)

Example:
```rust
// Use try_parse for alternatives
fn parse_instruction(s: &mut ParseStream) -> Result<Option<Instruction>> {
    if let Some(push) = s.try_parse(parse_push)? {
        Ok(Some(push))
    } else if s.accept_lexeme("while")?.is_some() {
        // We found "while" - we're committed, use parse for better errors
        stream.parse(|s| {
            let cond = parse_expression(s)?;
            Err(anyhow!("While loops are not supported in this language"))
        })?
    } else {
        Ok(None)
    }
}
```

## Error Handling Philosophy

- **`accept_*`** methods return `Result<Option<T>>`:
  - `Ok(None)` - didn't match (not an error, try next alternative)
  - `Ok(Some(T))` - successfully matched
  - `Err(e)` - syntax error (e.g., started to match but found invalid syntax)

- **`expect_*`** methods return `Result<T>`:
  - `Ok(T)` - successfully parsed
  - `Err(e)` - error (expected token not found)

This allows proper error propagation:
```rust
// instruction = push | pop | operation | label | jump
fn parse_instruction(s: &mut ParseStream) -> Result<Instruction> {
    if let Some(push) = s.try_parse(parse_push)? {
        Ok(push)
    } else if let Some(pop) = s.try_parse(parse_pop)? {
        Ok(pop)
    } else if s.accept_lexeme("while")?.is_some() {
        // Started with 'while' but this language doesn't have while loops
        Err(anyhow!("Unexpected 'while' - this language has no while loops"))
    } else {
        Err(anyhow!("Expected instruction"))
    }
}
```

## Grammar Patterns

### Example Stack Language Grammar (NOT RustLeaf)

```
program = instruction*

instruction = push | pop | operation | label | jump | block

push = "push" value                                 // Sequence
pop = "pop" register?                               // Optional register

operation = binop | unop                            // Choice
binop = "add" | "sub" | "mul" | "div"
unop = "neg" | "abs"

label = "@" ident ":" instruction+                  // One or more instructions

jump = "jmp" ident ("if" condition)?                // Optional conditional

block = "[" (value ("," value)*)? "]"              // Separated list with optional elements

value = number | string | register
register = "$" ident
condition = "zero" | "pos" | "neg"
```

Example program:
```
push 10
push 20
add
pop $result

@factorial:
  push $n
  push 1
  sub
  pop $n
  push $n
  push $result
  mul
  pop $result
  push $n
  jmp factorial if pos

push 5
pop $n
push 1
pop $result
jmp factorial

push [1, 2, 3]
push "done"
```

This grammar demonstrates:
- **Sequence**: `push = "push" value`
- **Choice**: `operation = binop | unop`
- **Optional**: `register?` in pop
- **Zero or more**: `instruction*`
- **One or more**: `instruction+` in label
- **Separated list**: `value ("," value)*`

### Sequence (A B C)
```rust
// "push" value
fn parse_push(s: &mut ParseStream) -> Result<Instruction> {
    s.expect_lexeme("push")?;
    let value = parse_value(s)?;
    Ok(Instruction::Push(value))
}
```

### Choice (A | B | C)
```rust
// operation = binop | unop
fn parse_operation(s: &mut ParseStream) -> Result<Option<Operation>> {
    if let Some(op) = parse_binop(s)? {
        Ok(Some(op))
    } else if let Some(op) = parse_unop(s)? {
        Ok(Some(op))
    } else {
        Ok(None)
    }
}

fn parse_binop(s: &mut ParseStream) -> Result<Option<Operation>> {
    if s.accept_lexeme("add")?.is_some() { Ok(Some(Operation::Add)) }
    else if s.accept_lexeme("sub")?.is_some() { Ok(Some(Operation::Sub)) }
    else if s.accept_lexeme("mul")?.is_some() { Ok(Some(Operation::Mul)) }
    else if s.accept_lexeme("div")?.is_some() { Ok(Some(Operation::Div)) }
    else { Ok(None) }
}
```

### Optional (A?)
```rust
// "pop" register?
fn parse_pop(s: &mut ParseStream) -> Result<Instruction> {
    s.expect_lexeme("pop")?;
    let register = s.try_parse(parse_register)?;
    Ok(Instruction::Pop(register))
}
```

### Zero or more (A*)
```rust
// program = instruction*
fn parse_program(s: &mut ParseStream) -> Result<Vec<Instruction>> {
    let mut instructions = Vec::new();
    while !s.is_at_end() {
        match parse_instruction(s)? {
            Some(instr) => instructions.push(instr),
            None => return Err(anyhow!("Expected instruction")),
        }
    }
    Ok(instructions)
}
```

### One or more (A+)
```rust
// @label: instruction+
fn parse_label(s: &mut ParseStream) -> Result<Option<Label>> {
    // Check if this is a label
    if s.accept_lexeme("@")?.is_none() {
        return Ok(None);
    }
    
    // Now we're committed - use expect_* for required elements
    let name = s.expect_type(TokenType::Ident)?.text;
    s.expect_lexeme(":")?;
    
    let first = parse_instruction(s)?
        .ok_or_else(|| anyhow!("Expected at least one instruction in label"))?;
    let mut body = vec![first];
    
    // Keep parsing instructions until we can't
    while let Some(instr) = parse_instruction(s)? {
        body.push(instr);
    }
    
    Ok(Some(Label { name, body }))
}
```

### Separated list
```rust
// "[" (value ("," value)*)? "]"
fn parse_block(s: &mut ParseStream) -> Result<Option<Block>> {
    // Check if this is a block
    if s.accept_lexeme("[")?.is_none() {
        return Ok(None);
    }
    
    let mut values = vec![];
    
    // Try to parse first value
    if let Some(first) = parse_value(s)? {
        values.push(first);
        
        // Parse remaining comma-separated values
        while s.accept_lexeme(",")?.is_some() {
            match parse_value(s)? {
                Some(v) => values.push(v),
                None => return Err(anyhow!("Expected value after comma")),
            }
        }
    }
    
    s.expect_lexeme("]")?;
    Ok(Some(Block(values)))
}
```