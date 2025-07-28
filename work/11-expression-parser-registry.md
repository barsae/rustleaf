# Create expression parser registry

Replace if-else chains in parser with a registry-based approach.

Read all relevant code before working.

## Tasks

1. Create parser registry in `src/parser/registry.rs`:
   ```rust
   type ExpressionParser = fn(&mut Parser) -> Result<Option<Expression>>;

   pub struct ParserRegistry {
       prefix_parsers: HashMap<TokenType, ExpressionParser>,
       infix_parsers: HashMap<TokenType, InfixParser>,
   }

   pub struct InfixParser {
       parser: fn(&mut Parser, left: Expression) -> Result<Expression>,
       precedence: Precedence,
   }
   ```

2. Register parsers for each token type:
   ```rust
   impl ParserRegistry {
       pub fn new() -> Self {
           let mut registry = Self::default();

           // Prefix parsers
           registry.register_prefix(TokenType::Int, Parser::parse_int_literal);
           registry.register_prefix(TokenType::String, Parser::parse_string_literal);
           registry.register_prefix(TokenType::LeftParen, Parser::parse_grouped_expr);
           // ... etc

           // Infix parsers
           registry.register_infix(TokenType::Plus, Parser::parse_binary_op, Precedence::Addition);
           registry.register_infix(TokenType::Dot, Parser::parse_member_access, Precedence::Call);
           // ... etc
       }
   }
   ```

3. Update `parse_expression()` to use registry

4. Remove if-else chains from expression parsing

5. Enable dynamic parser registration for extensibility

6. Run `just test` after registry implementation

## Success Criteria
- No if-else chains in expression parsing
- Easy to add new expression types
- Plugin architecture ready
- All tests pass