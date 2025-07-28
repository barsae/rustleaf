# Extract token stream abstraction

Create a higher-level API for token consumption in the parser.

Read all relevant code before working.

## Tasks

1. Create `src/parser/token_stream.rs`:
   ```rust
   pub struct TokenStream {
       tokens: Vec<Token>,
       current: usize,
   }

   impl TokenStream {
       pub fn peek(&self) -> &Token
       pub fn peek_type(&self) -> TokenType
       pub fn consume(&mut self) -> Token
       pub fn consume_if(&mut self, token_type: TokenType) -> Option<Token>
       pub fn expect(&mut self, token_type: TokenType) -> Result<Token>
       pub fn checkpoint(&self) -> usize
       pub fn restore(&mut self, checkpoint: usize)
       pub fn is_at_end(&self) -> bool
   }
   ```

2. Replace direct token array access in parser with TokenStream methods

3. Implement lookahead methods:
   ```rust
   pub fn peek_ahead(&self, n: usize) -> Option<&Token>
   pub fn match_sequence(&self, types: &[TokenType]) -> bool
   ```

4. Update `Parser` struct to use `TokenStream` instead of `Vec<Token>` and `current`

5. Update all parser methods to use new API

6. Run `just test` after migrating each parser method

## Success Criteria
- Cleaner parser code
- No direct index manipulation
- Better encapsulation
- All tests pass