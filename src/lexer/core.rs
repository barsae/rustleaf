use crate::lexer::{Token, TokenType};
use anyhow::Result;
use regex::Regex;

struct LexerRule {
    pattern: Regex,
    token_type: fn(&str) -> Token,
    ignore: bool,
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
            LexerRule {
                pattern: Regex::new(r"^[ \t]+").unwrap(),
                token_type: |_| Token::simple(TokenType::Eof),
                ignore: true,
            },
            // Line terminators (ignored)
            LexerRule {
                pattern: Regex::new(r"^(\r\n|\r|\n)").unwrap(),
                token_type: |_| Token::simple(TokenType::Eof),
                ignore: true,
            },
            // Documentation comments
            LexerRule {
                pattern: Regex::new(r"^///(.*)").unwrap(),
                token_type: |s| Token::with_text(TokenType::DocComment, &s[3..]),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^//!(.*)").unwrap(),
                token_type: |s| Token::with_text(TokenType::InnerDocComment, &s[3..]),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^/\*\*([^*]|\*[^/])*\*/").unwrap(),
                token_type: |s| Token::with_text(TokenType::DocCommentBlock, &s[3..s.len() - 2]),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^/\*!([^*]|\*[^/])*\*/").unwrap(),
                token_type: |s| {
                    Token::with_text(TokenType::InnerDocCommentBlock, &s[3..s.len() - 2])
                },
                ignore: false,
            },
            // Regular comments (ignored)
            LexerRule {
                pattern: Regex::new(r"^//.*").unwrap(),
                token_type: |_| Token::simple(TokenType::Eof),
                ignore: true,
            },
            LexerRule {
                pattern: Regex::new(r"^/\*([^*]|\*[^/])*\*/").unwrap(),
                token_type: |_| Token::simple(TokenType::Eof),
                ignore: true,
            },
            // === STRING LITERALS ===
            LexerRule {
                pattern: Regex::new(r#"^"([^"\\]|\\.|[\r\n])*""#).unwrap(),
                token_type: |s| {
                    let content = &s[1..s.len() - 1];
                    let unescaped = Self::process_escape_sequences(content);
                    Token::with_text(TokenType::String, &unescaped)
                },
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r#"^r"([^"])*""#).unwrap(),
                token_type: |s| Token::with_text(TokenType::RawString, &s[2..s.len() - 1]),
                ignore: false,
            },
            // === NUMERIC LITERALS ===
            // Floating-point literals
            LexerRule {
                pattern: Regex::new(r"^(\d+\.\d+)([eE][+-]?\d+)?").unwrap(),
                token_type: |s| Token::with_text(TokenType::Float, s),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^(\d+)([eE][+-]?\d+)").unwrap(),
                token_type: |s| Token::with_text(TokenType::Float, s),
                ignore: false,
            },
            // Integer literals
            LexerRule {
                pattern: Regex::new(r"^0x[0-9a-fA-F]([0-9a-fA-F_]*[0-9a-fA-F])?").unwrap(),
                token_type: |s| Token::with_text(TokenType::Int, s),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^0o[0-7]([0-7_]*[0-7])?").unwrap(),
                token_type: |s| Token::with_text(TokenType::Int, s),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^0b[01]([01_]*[01])?").unwrap(),
                token_type: |s| Token::with_text(TokenType::Int, s),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^[0-9]([0-9_]*[0-9])?").unwrap(),
                token_type: |s| Token::with_text(TokenType::Int, s),
                ignore: false,
            },
            // === OPERATORS ===
            // Multi-character operators
            LexerRule {
                pattern: Regex::new(r"^\*\*").unwrap(),
                token_type: |_| Token::simple(TokenType::StarStar),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^==").unwrap(),
                token_type: |_| Token::simple(TokenType::EqualEqual),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^!=").unwrap(),
                token_type: |_| Token::simple(TokenType::BangEqual),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^!").unwrap(),
                token_type: |_| Token::simple(TokenType::Bang),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^<=").unwrap(),
                token_type: |_| Token::simple(TokenType::LessEqual),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^>=").unwrap(),
                token_type: |_| Token::simple(TokenType::GreaterEqual),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^\+=").unwrap(),
                token_type: |_| Token::simple(TokenType::PlusEqual),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^-=").unwrap(),
                token_type: |_| Token::simple(TokenType::MinusEqual),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^\*=").unwrap(),
                token_type: |_| Token::simple(TokenType::StarEqual),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^/=").unwrap(),
                token_type: |_| Token::simple(TokenType::SlashEqual),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^%=").unwrap(),
                token_type: |_| Token::simple(TokenType::PercentEqual),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^<<").unwrap(),
                token_type: |_| Token::simple(TokenType::LessLess),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^>>").unwrap(),
                token_type: |_| Token::simple(TokenType::GreaterGreater),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^\.\.=").unwrap(),
                token_type: |_| Token::simple(TokenType::DotDotEqual),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^\.\.").unwrap(),
                token_type: |_| Token::simple(TokenType::DotDot),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^::").unwrap(),
                token_type: |_| Token::simple(TokenType::DoubleColon),
                ignore: false,
            },
            // Single-character operators
            LexerRule {
                pattern: Regex::new(r"^\+").unwrap(),
                token_type: |_| Token::simple(TokenType::Plus),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^-").unwrap(),
                token_type: |_| Token::simple(TokenType::Minus),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^\*").unwrap(),
                token_type: |_| Token::simple(TokenType::Star),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^/").unwrap(),
                token_type: |_| Token::simple(TokenType::Slash),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^%").unwrap(),
                token_type: |_| Token::simple(TokenType::Percent),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^=").unwrap(),
                token_type: |_| Token::simple(TokenType::Equal),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^<").unwrap(),
                token_type: |_| Token::simple(TokenType::Less),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^>").unwrap(),
                token_type: |_| Token::simple(TokenType::Greater),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^&").unwrap(),
                token_type: |_| Token::simple(TokenType::Ampersand),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^\|").unwrap(),
                token_type: |_| Token::simple(TokenType::Pipe),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^\^").unwrap(),
                token_type: |_| Token::simple(TokenType::Caret),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^~").unwrap(),
                token_type: |_| Token::simple(TokenType::Tilde),
                ignore: false,
            },
            // === PUNCTUATION ===
            LexerRule {
                pattern: Regex::new(r"^\(").unwrap(),
                token_type: |_| Token::simple(TokenType::LeftParen),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^\)").unwrap(),
                token_type: |_| Token::simple(TokenType::RightParen),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^\{").unwrap(),
                token_type: |_| Token::simple(TokenType::LeftBrace),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^\}").unwrap(),
                token_type: |_| Token::simple(TokenType::RightBrace),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^\[").unwrap(),
                token_type: |_| Token::simple(TokenType::LeftBracket),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^\]").unwrap(),
                token_type: |_| Token::simple(TokenType::RightBracket),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^,").unwrap(),
                token_type: |_| Token::simple(TokenType::Comma),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^;").unwrap(),
                token_type: |_| Token::simple(TokenType::Semicolon),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^\.").unwrap(),
                token_type: |_| Token::simple(TokenType::Dot),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^:").unwrap(),
                token_type: |_| Token::simple(TokenType::Colon),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^#").unwrap(),
                token_type: |_| Token::simple(TokenType::Hash),
                ignore: false,
            },
            // === KEYWORDS ===
            // Control flow
            LexerRule {
                pattern: Regex::new(r"^if\b").unwrap(),
                token_type: |_| Token::simple(TokenType::If),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^else\b").unwrap(),
                token_type: |_| Token::simple(TokenType::Else),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^while\b").unwrap(),
                token_type: |_| Token::simple(TokenType::While),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^for\b").unwrap(),
                token_type: |_| Token::simple(TokenType::For),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^loop\b").unwrap(),
                token_type: |_| Token::simple(TokenType::Loop),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^return\b").unwrap(),
                token_type: |_| Token::simple(TokenType::Return),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^break\b").unwrap(),
                token_type: |_| Token::simple(TokenType::Break),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^continue\b").unwrap(),
                token_type: |_| Token::simple(TokenType::Continue),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^match\b").unwrap(),
                token_type: |_| Token::simple(TokenType::Match),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^case\b").unwrap(),
                token_type: |_| Token::simple(TokenType::Case),
                ignore: false,
            },
            // Declarations
            LexerRule {
                pattern: Regex::new(r"^var\b").unwrap(),
                token_type: |_| Token::simple(TokenType::Var),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^fn\b").unwrap(),
                token_type: |_| Token::simple(TokenType::Fn),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^class\b").unwrap(),
                token_type: |_| Token::simple(TokenType::Class),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^static\b").unwrap(),
                token_type: |_| Token::simple(TokenType::Static),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^pub\b").unwrap(),
                token_type: |_| Token::simple(TokenType::Pub),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^macro\b").unwrap(),
                token_type: |_| Token::simple(TokenType::Macro),
                ignore: false,
            },
            // Module system
            LexerRule {
                pattern: Regex::new(r"^import\b").unwrap(),
                token_type: |_| Token::simple(TokenType::Import),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^use\b").unwrap(),
                token_type: |_| Token::simple(TokenType::Use),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^as\b").unwrap(),
                token_type: |_| Token::simple(TokenType::As),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^from\b").unwrap(),
                token_type: |_| Token::simple(TokenType::From),
                ignore: false,
            },
            // Exception handling
            LexerRule {
                pattern: Regex::new(r"^try\b").unwrap(),
                token_type: |_| Token::simple(TokenType::Try),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^catch\b").unwrap(),
                token_type: |_| Token::simple(TokenType::Catch),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^with\b").unwrap(),
                token_type: |_| Token::simple(TokenType::With),
                ignore: false,
            },
            // Logical operators
            LexerRule {
                pattern: Regex::new(r"^and\b").unwrap(),
                token_type: |_| Token::simple(TokenType::And),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^or\b").unwrap(),
                token_type: |_| Token::simple(TokenType::Or),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^xor\b").unwrap(),
                token_type: |_| Token::simple(TokenType::Xor),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^not\b").unwrap(),
                token_type: |_| Token::simple(TokenType::Not),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^in\b").unwrap(),
                token_type: |_| Token::simple(TokenType::In),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^is\b").unwrap(),
                token_type: |_| Token::simple(TokenType::Is),
                ignore: false,
            },
            // Literals
            LexerRule {
                pattern: Regex::new(r"^true\b").unwrap(),
                token_type: |_| Token::simple(TokenType::True),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^false\b").unwrap(),
                token_type: |_| Token::simple(TokenType::False),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^null\b").unwrap(),
                token_type: |_| Token::simple(TokenType::Null),
                ignore: false,
            },
            // === IDENTIFIERS ===
            // Must come last to avoid matching keywords
            LexerRule {
                pattern: Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap(),
                token_type: |s| Token::with_text(TokenType::Ident, s),
                ignore: false,
            },
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
            if !rule.ignore {
                let token = (rule.token_type)(&matched_text);
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

    fn expand_string_interpolation(&self, tokens: Vec<Token>) -> Result<Vec<Token>> {
        let mut result = Vec::new();

        for token in tokens {
            if token.token_type == TokenType::String && self.contains_interpolation(&token) {
                result.extend(self.expand_interpolated_string(token)?);
            } else {
                result.push(token);
            }
        }

        Ok(result)
    }

    fn contains_interpolation(&self, token: &Token) -> bool {
        if let Some(text) = &token.text {
            text.contains("${")
        } else {
            false
        }
    }

    fn expand_interpolated_string(&self, token: Token) -> Result<Vec<Token>> {
        let text = token.text.as_ref().unwrap();
        let mut result = Vec::new();
        let mut current_pos = 0;

        while let Some(start_pos) = text[current_pos..].find("${") {
            let actual_start = current_pos + start_pos;

            // Add text part before interpolation (if any)
            if actual_start > current_pos {
                let text_part = &text[current_pos..actual_start];
                result.push(Token::with_text(TokenType::StringPart, text_part));
            }

            // Find the matching closing brace
            let expr_start = actual_start + 2; // Skip ${
            if let Some(end_pos) = self.find_matching_brace(text, expr_start) {
                // Add interpolation start marker
                result.push(Token::simple(TokenType::InterpolationStart));

                // Extract and tokenize the expression
                let expr_text = &text[expr_start..end_pos];
                let expr_tokens = Self::tokenize(expr_text)?;

                // Add expression tokens (excluding EOF)
                for expr_token in expr_tokens {
                    if expr_token.token_type != TokenType::Eof {
                        result.push(expr_token);
                    }
                }

                // Add interpolation end marker
                result.push(Token::simple(TokenType::InterpolationEnd));

                current_pos = end_pos + 1; // Skip closing brace
            } else {
                return Err(anyhow::anyhow!(
                    "Unclosed interpolation in string: missing '}}' after '${{' at position {}",
                    actual_start
                ));
            }
        }

        // Add remaining text (if any)
        if current_pos < text.len() {
            let remaining = &text[current_pos..];
            result.push(Token::with_text(TokenType::StringPart, remaining));
        }

        Ok(result)
    }

    fn find_matching_brace(&self, text: &str, start: usize) -> Option<usize> {
        let chars: Vec<char> = text.chars().collect();
        let mut brace_count = 1;
        let mut in_string = false;
        let mut escape_next = false;

        for (i, &ch) in chars.iter().enumerate().skip(start) {
            if escape_next {
                escape_next = false;
                continue;
            }

            match ch {
                '\\' if in_string => escape_next = true,
                '"' => in_string = !in_string,
                '{' if !in_string => brace_count += 1,
                '}' if !in_string => {
                    brace_count -= 1;
                    if brace_count == 0 {
                        return Some(i);
                    }
                }
                _ => {}
            }
        }

        None // No matching brace found
    }

    /// Process escape sequences in string literals
    fn process_escape_sequences(content: &str) -> String {
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
                                while let Some(hex_char) = chars.next() {
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

    /// Phase 3: Rewrite token pairs like "not in" -> NotIn and "is not" -> IsNot
    fn rewrite_token_pairs(&self, tokens: Vec<Token>) -> Result<Vec<Token>> {
        let mut result = Vec::new();
        let mut i = 0;

        while i < tokens.len() {
            // Check for "not in" pattern
            if i + 1 < tokens.len()
                && tokens[i].token_type == TokenType::Not
                && tokens[i + 1].token_type == TokenType::In
            {
                // Replace "not in" with "NotIn"
                result.push(Token::simple(TokenType::NotIn));
                i += 2; // Skip both tokens
            }
            // Check for "is not" pattern
            else if i + 1 < tokens.len()
                && tokens[i].token_type == TokenType::Is
                && tokens[i + 1].token_type == TokenType::Not
            {
                // Replace "is not" with "IsNot"
                result.push(Token::simple(TokenType::IsNot));
                i += 2; // Skip both tokens
            } else {
                // Copy token as-is
                result.push(tokens[i].clone());
                i += 1;
            }
        }

        Ok(result)
    }
}
