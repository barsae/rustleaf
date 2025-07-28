use crate::lexer::{Token, TokenType};
use anyhow::Result;
use regex::Regex;

enum TokenAction {
    Simple(TokenType),
    WithText(TokenType),
    WithTextSlice(TokenType, usize, usize), // token_type, start, end_offset
    Custom(fn(&str) -> Token),
    Ignored,
}

struct LexerRule {
    pattern: Regex,
    action: TokenAction,
}

impl LexerRule {
    fn simple_ignored(pattern: &str) -> Self {
        LexerRule {
            pattern: Regex::new(pattern).unwrap(),
            action: TokenAction::Ignored,
        }
    }

    fn simple(pattern: &str, token_type: TokenType) -> Self {
        LexerRule {
            pattern: Regex::new(pattern).unwrap(),
            action: TokenAction::Simple(token_type),
        }
    }

    fn with_text(pattern: &str, token_type: TokenType) -> Self {
        LexerRule {
            pattern: Regex::new(pattern).unwrap(),
            action: TokenAction::WithText(token_type),
        }
    }

    fn with_text_slice(
        pattern: &str,
        token_type: TokenType,
        start: usize,
        end_offset: usize,
    ) -> Self {
        LexerRule {
            pattern: Regex::new(pattern).unwrap(),
            action: TokenAction::WithTextSlice(token_type, start, end_offset),
        }
    }

    fn custom(pattern: &str, token_fn: fn(&str) -> Token) -> Self {
        LexerRule {
            pattern: Regex::new(pattern).unwrap(),
            action: TokenAction::Custom(token_fn),
        }
    }

    fn apply(&self, text: &str) -> Option<Token> {
        match &self.action {
            TokenAction::Simple(token_type) => Some(Token::simple(*token_type)),
            TokenAction::WithText(token_type) => Some(Token::with_text(*token_type, text)),
            TokenAction::WithTextSlice(token_type, start, end_offset) => Some(Token::with_text(
                *token_type,
                &text[*start..text.len() - *end_offset],
            )),
            TokenAction::Custom(token_fn) => Some(token_fn(text)),
            TokenAction::Ignored => None,
        }
    }
}

pub struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn tokenize(source: &str) -> Result<Vec<Token>> {
        // UTF-8 validation - str guarantees valid UTF-8, but check for BOM
        let source = source.strip_prefix('\u{FEFF}').unwrap_or(source);

        let mut lexer = Self::new(source);
        lexer.tokenize_internal()
    }

    fn new(source: &str) -> Self {
        Lexer {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
        }
    }

    fn create_rules() -> Vec<LexerRule> {
        vec![
            // === WHITESPACE AND COMMENTS ===
            // Whitespace (ignored)
            LexerRule::simple_ignored(r"^[ \t]+"),
            // Line terminators (ignored)
            LexerRule::simple_ignored(r"^(\r\n|\r|\n)"),
            // Documentation comments
            LexerRule::with_text_slice(r"^///(.*)", TokenType::DocComment, 3, 0),
            LexerRule::with_text_slice(r"^//!(.*)", TokenType::InnerDocComment, 3, 0),
            LexerRule::with_text_slice(
                r"^/\*\*([^*]|\*[^/])*\*/",
                TokenType::DocCommentBlock,
                3,
                2,
            ),
            LexerRule::with_text_slice(
                r"^/\*!([^*]|\*[^/])*\*/",
                TokenType::InnerDocCommentBlock,
                3,
                2,
            ),
            // Regular comments (ignored)
            LexerRule::simple_ignored(r"^//.*"),
            LexerRule::simple_ignored(r"^/\*([^*]|\*[^/])*\*/"),
            // === STRING LITERALS ===
            LexerRule::custom(r#"^"([^"\\]|\\.|[\r\n])*""#, |s| {
                let content = &s[1..s.len() - 1];
                let unescaped = Lexer::process_escape_sequences(content);
                Token::with_text(TokenType::String, &unescaped)
            }),
            LexerRule::with_text_slice(r#"^r"([^"])*""#, TokenType::RawString, 2, 1),
            // === NUMERIC LITERALS ===
            // Floating-point literals
            LexerRule::with_text(r"^(\d+\.\d+)([eE][+-]?\d+)?", TokenType::Float),
            LexerRule::with_text(r"^(\d+)([eE][+-]?\d+)", TokenType::Float),
            // Integer literals
            LexerRule::with_text(r"^0x[0-9a-fA-F]([0-9a-fA-F_]*[0-9a-fA-F])?", TokenType::Int),
            LexerRule::with_text(r"^0o[0-7]([0-7_]*[0-7])?", TokenType::Int),
            LexerRule::with_text(r"^0b[01]([01_]*[01])?", TokenType::Int),
            LexerRule::with_text(r"^[0-9]([0-9_]*[0-9])?", TokenType::Int),
            // === OPERATORS ===
            // Multi-character operators
            LexerRule::simple(r"^\*\*", TokenType::StarStar),
            LexerRule::simple(r"^==", TokenType::EqualEqual),
            LexerRule::simple(r"^!=", TokenType::BangEqual),
            LexerRule::simple(r"^!", TokenType::Bang),
            LexerRule::simple(r"^<=", TokenType::LessEqual),
            LexerRule::simple(r"^>=", TokenType::GreaterEqual),
            LexerRule::simple(r"^\+=", TokenType::PlusEqual),
            LexerRule::simple(r"^-=", TokenType::MinusEqual),
            LexerRule::simple(r"^\*=", TokenType::StarEqual),
            LexerRule::simple(r"^/=", TokenType::SlashEqual),
            LexerRule::simple(r"^%=", TokenType::PercentEqual),
            LexerRule::simple(r"^<<", TokenType::LessLess),
            LexerRule::simple(r"^>>", TokenType::GreaterGreater),
            LexerRule::simple(r"^\.\.=", TokenType::DotDotEqual),
            LexerRule::simple(r"^\.\.", TokenType::DotDot),
            LexerRule::simple(r"^::", TokenType::DoubleColon),
            // Single-character operators
            LexerRule::simple(r"^\+", TokenType::Plus),
            LexerRule::simple(r"^-", TokenType::Minus),
            LexerRule::simple(r"^\*", TokenType::Star),
            LexerRule::simple(r"^/", TokenType::Slash),
            LexerRule::simple(r"^%", TokenType::Percent),
            LexerRule::simple(r"^=", TokenType::Equal),
            LexerRule::simple(r"^<", TokenType::Less),
            LexerRule::simple(r"^>", TokenType::Greater),
            LexerRule::simple(r"^&", TokenType::Ampersand),
            LexerRule::simple(r"^\|", TokenType::Pipe),
            LexerRule::simple(r"^\^", TokenType::Caret),
            LexerRule::simple(r"^~", TokenType::Tilde),
            // === PUNCTUATION ===
            LexerRule::simple(r"^\(", TokenType::LeftParen),
            LexerRule::simple(r"^\)", TokenType::RightParen),
            LexerRule::simple(r"^\{", TokenType::LeftBrace),
            LexerRule::simple(r"^\}", TokenType::RightBrace),
            LexerRule::simple(r"^\[", TokenType::LeftBracket),
            LexerRule::simple(r"^\]", TokenType::RightBracket),
            LexerRule::simple(r"^,", TokenType::Comma),
            LexerRule::simple(r"^;", TokenType::Semicolon),
            LexerRule::simple(r"^\.", TokenType::Dot),
            LexerRule::simple(r"^:", TokenType::Colon),
            LexerRule::simple(r"^#", TokenType::Hash),
            // === KEYWORDS ===
            // Control flow
            LexerRule::simple(r"^if\b", TokenType::If),
            LexerRule::simple(r"^else\b", TokenType::Else),
            LexerRule::simple(r"^while\b", TokenType::While),
            LexerRule::simple(r"^for\b", TokenType::For),
            LexerRule::simple(r"^loop\b", TokenType::Loop),
            LexerRule::simple(r"^return\b", TokenType::Return),
            LexerRule::simple(r"^break\b", TokenType::Break),
            LexerRule::simple(r"^continue\b", TokenType::Continue),
            LexerRule::simple(r"^match\b", TokenType::Match),
            LexerRule::simple(r"^case\b", TokenType::Case),
            // Declarations
            LexerRule::simple(r"^var\b", TokenType::Var),
            LexerRule::simple(r"^fn\b", TokenType::Fn),
            LexerRule::simple(r"^class\b", TokenType::Class),
            LexerRule::simple(r"^static\b", TokenType::Static),
            LexerRule::simple(r"^pub\b", TokenType::Pub),
            LexerRule::simple(r"^macro\b", TokenType::Macro),
            // Module system
            LexerRule::simple(r"^import\b", TokenType::Import),
            LexerRule::simple(r"^use\b", TokenType::Use),
            LexerRule::simple(r"^as\b", TokenType::As),
            LexerRule::simple(r"^from\b", TokenType::From),
            // Exception handling
            LexerRule::simple(r"^try\b", TokenType::Try),
            LexerRule::simple(r"^catch\b", TokenType::Catch),
            LexerRule::simple(r"^with\b", TokenType::With),
            // Logical operators
            LexerRule::simple(r"^and\b", TokenType::And),
            LexerRule::simple(r"^or\b", TokenType::Or),
            LexerRule::simple(r"^xor\b", TokenType::Xor),
            LexerRule::simple(r"^not\b", TokenType::Not),
            LexerRule::simple(r"^in\b", TokenType::In),
            LexerRule::simple(r"^is\b", TokenType::Is),
            // Literals
            LexerRule::simple(r"^true\b", TokenType::True),
            LexerRule::simple(r"^false\b", TokenType::False),
            LexerRule::simple(r"^null\b", TokenType::Null),
            // === IDENTIFIERS ===
            // Must come last to avoid matching keywords
            LexerRule::with_text(r"^[a-zA-Z_][a-zA-Z0-9_]*", TokenType::Ident),
        ]
    }

    fn tokenize_internal(&mut self) -> Result<Vec<Token>> {
        let rules = Self::create_rules();

        while !self.is_at_end() {
            self.start = self.current;
            self.next_token(&rules)?;
        }

        self.tokens.push(Token::simple(TokenType::Eof));

        // Phase 2: Expand interpolated strings
        let expanded_tokens = self.expand_string_interpolation(self.tokens.clone())?;

        // Phase 3: Rewrite token pairs (not in, is not)
        let rewritten_tokens = self.rewrite_token_pairs(expanded_tokens)?;
        Ok(rewritten_tokens)
    }

    fn next_token(&mut self, rules: &[LexerRule]) -> Result<()> {
        let remaining_text = &self.source[self.current..].iter().collect::<String>();

        let mut best_match: Option<(usize, &LexerRule)> = None;

        // Try all rules and find the longest match
        for rule in rules {
            if let Some(captures) = rule.pattern.find(remaining_text) {
                let match_len = captures.end();
                if match_len > 0 {
                    if let Some((current_best_len, _)) = best_match {
                        if match_len > current_best_len {
                            best_match = Some((match_len, rule));
                        }
                    } else {
                        best_match = Some((match_len, rule));
                    }
                }
            }
        }

        if let Some((match_len, rule)) = best_match {
            let matched_text = remaining_text[..match_len].to_string();

            // Handle line/column tracking for line terminators
            if matched_text.contains('\n') {
                let lines = matched_text.matches('\n').count();
                self.line += lines;
                if let Some(last_newline) = matched_text.rfind('\n') {
                    self.column = matched_text.len() - last_newline;
                } else {
                    self.column += matched_text.len();
                }
            } else {
                self.column += matched_text.len();
            }

            // Advance position
            self.current += match_len;

            // Create token if not ignored
            if let Some(token) = rule.apply(&matched_text) {
                self.tokens.push(token);
            }

            Ok(())
        } else {
            // No pattern matched
            let ch = self.source[self.current];
            Err(anyhow::anyhow!(
                "Unexpected character '{}' at line {}, column {}",
                ch,
                self.line,
                self.column
            ))
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// Process escape sequences in string literals
    pub fn process_escape_sequences(content: &str) -> String {
        let mut result = String::new();
        let mut chars = content.chars();

        while let Some(ch) = chars.next() {
            if ch == '\\' {
                if let Some(escaped) = chars.next() {
                    match escaped {
                        'n' => result.push('\n'),
                        'r' => result.push('\r'),
                        't' => result.push('\t'),
                        '\\' => result.push('\\'),
                        '"' => result.push('"'),
                        '\'' => result.push('\''),
                        '$' => result.push('$'),
                        '{' => result.push('{'),
                        '}' => result.push('}'),
                        'u' => {
                            // Handle \u{XXXXXX} Unicode escape sequences
                            if chars.next() == Some('{') {
                                let mut hex_digits = String::new();
                                for hex_char in chars.by_ref() {
                                    if hex_char == '}' {
                                        break;
                                    }
                                    hex_digits.push(hex_char);
                                }

                                if let Ok(code_point) = u32::from_str_radix(&hex_digits, 16) {
                                    if let Some(unicode_char) = char::from_u32(code_point) {
                                        result.push(unicode_char);
                                    } else {
                                        // Invalid Unicode code point, keep literal
                                        result.push_str(&format!("\\u{{{}}}", hex_digits));
                                    }
                                } else {
                                    // Invalid hex, keep literal
                                    result.push_str(&format!("\\u{{{}}}", hex_digits));
                                }
                            } else {
                                // Missing {, treat as literal
                                result.push('\\');
                                result.push('u');
                            }
                        }
                        _ => {
                            // Unknown escape sequence, keep literal
                            result.push('\\');
                            result.push(escaped);
                        }
                    }
                } else {
                    // Backslash at end of string
                    result.push('\\');
                }
            } else {
                result.push(ch);
            }
        }

        result
    }
}
