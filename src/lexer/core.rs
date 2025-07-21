use anyhow::Result;
use regex::Regex;
use crate::lexer::Token;

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
                token_type: |_| Token::Eof,
                ignore: true,
            },
            // Line terminators (ignored)
            LexerRule {
                pattern: Regex::new(r"^(\r\n|\r|\n)").unwrap(),
                token_type: |_| Token::Eof,
                ignore: true,
            },
            // Documentation comments
            LexerRule {
                pattern: Regex::new(r"^///(.*)").unwrap(),
                token_type: |s| Token::DocComment(s[3..].to_string()),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^//!(.*)").unwrap(),
                token_type: |s| Token::InnerDocComment(s[3..].to_string()),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^/\*\*([^*]|\*[^/])*\*/").unwrap(),
                token_type: |s| Token::DocCommentBlock(s[3..s.len()-2].to_string()),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^/\*!([^*]|\*[^/])*\*/").unwrap(),
                token_type: |s| Token::InnerDocCommentBlock(s[3..s.len()-2].to_string()),
                ignore: false,
            },
            // Regular comments (ignored)
            LexerRule {
                pattern: Regex::new(r"^//.*").unwrap(),
                token_type: |_| Token::Eof,
                ignore: true,
            },
            LexerRule {
                pattern: Regex::new(r"^/\*([^*]|\*[^/])*\*/").unwrap(),
                token_type: |_| Token::Eof,
                ignore: true,
            },

            // === STRING LITERALS ===
            LexerRule {
                pattern: Regex::new(r#"^"([^"\\]|\\.)*""#).unwrap(),
                token_type: |s| Token::String(s[1..s.len()-1].to_string()),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r#"^"""([^"]*|"[^"]*|""[^"])*""""#).unwrap(),
                token_type: |s| Token::MultilineString(s[3..s.len()-3].to_string()),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r#"^r"([^"\r\n])*""#).unwrap(),
                token_type: |s| Token::RawString(s[2..s.len()-1].to_string()),
                ignore: false,
            },

            // === NUMERIC LITERALS ===
            // Floating-point literals
            LexerRule {
                pattern: Regex::new(r"^(\d+\.\d+)([eE][+-]?\d+)?").unwrap(),
                token_type: |s| Token::Float(s.replace('_', "").parse().unwrap_or(0.0)),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^(\d+)([eE][+-]?\d+)").unwrap(),
                token_type: |s| Token::Float(s.replace('_', "").parse().unwrap_or(0.0)),
                ignore: false,
            },
            // Integer literals
            LexerRule {
                pattern: Regex::new(r"^0x[0-9a-fA-F]([0-9a-fA-F_]*[0-9a-fA-F])?").unwrap(),
                token_type: |s| Token::Int(i64::from_str_radix(&s[2..].replace('_', ""), 16).unwrap_or(0)),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^0o[0-7]([0-7_]*[0-7])?").unwrap(),
                token_type: |s| Token::Int(i64::from_str_radix(&s[2..].replace('_', ""), 8).unwrap_or(0)),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^0b[01]([01_]*[01])?").unwrap(),
                token_type: |s| Token::Int(i64::from_str_radix(&s[2..].replace('_', ""), 2).unwrap_or(0)),
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^[0-9]([0-9_]*[0-9])?").unwrap(),
                token_type: |s| Token::Int(s.replace('_', "").parse().unwrap_or(0)),
                ignore: false,
            },

            // === OPERATORS ===
            // Multi-character operators
            LexerRule {
                pattern: Regex::new(r"^\*\*").unwrap(),
                token_type: |_| Token::StarStar,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^==").unwrap(),
                token_type: |_| Token::EqualEqual,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^!=").unwrap(),
                token_type: |_| Token::BangEqual,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^<=").unwrap(),
                token_type: |_| Token::LessEqual,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^>=").unwrap(),
                token_type: |_| Token::GreaterEqual,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^\+=").unwrap(),
                token_type: |_| Token::PlusEqual,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^-=").unwrap(),
                token_type: |_| Token::MinusEqual,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^\*=").unwrap(),
                token_type: |_| Token::StarEqual,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^/=").unwrap(),
                token_type: |_| Token::SlashEqual,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^%=").unwrap(),
                token_type: |_| Token::PercentEqual,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^<<").unwrap(),
                token_type: |_| Token::LessLess,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^>>").unwrap(),
                token_type: |_| Token::GreaterGreater,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^\.\.=").unwrap(),
                token_type: |_| Token::DotDotEqual,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^\.\.").unwrap(),
                token_type: |_| Token::DotDot,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^::").unwrap(),
                token_type: |_| Token::DoubleColon,
                ignore: false,
            },
            // Single-character operators
            LexerRule {
                pattern: Regex::new(r"^\+").unwrap(),
                token_type: |_| Token::Plus,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^-").unwrap(),
                token_type: |_| Token::Minus,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^\*").unwrap(),
                token_type: |_| Token::Star,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^/").unwrap(),
                token_type: |_| Token::Slash,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^%").unwrap(),
                token_type: |_| Token::Percent,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^=").unwrap(),
                token_type: |_| Token::Equal,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^<").unwrap(),
                token_type: |_| Token::Less,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^>").unwrap(),
                token_type: |_| Token::Greater,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^&").unwrap(),
                token_type: |_| Token::Ampersand,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^\|").unwrap(),
                token_type: |_| Token::Pipe,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^\^").unwrap(),
                token_type: |_| Token::Caret,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^~").unwrap(),
                token_type: |_| Token::Tilde,
                ignore: false,
            },

            // === PUNCTUATION ===
            LexerRule {
                pattern: Regex::new(r"^\(").unwrap(),
                token_type: |_| Token::LeftParen,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^\)").unwrap(),
                token_type: |_| Token::RightParen,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^\{").unwrap(),
                token_type: |_| Token::LeftBrace,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^\}").unwrap(),
                token_type: |_| Token::RightBrace,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^\[").unwrap(),
                token_type: |_| Token::LeftBracket,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^\]").unwrap(),
                token_type: |_| Token::RightBracket,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^,").unwrap(),
                token_type: |_| Token::Comma,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^;").unwrap(),
                token_type: |_| Token::Semicolon,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^\.").unwrap(),
                token_type: |_| Token::Dot,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^:").unwrap(),
                token_type: |_| Token::Colon,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^#").unwrap(),
                token_type: |_| Token::Hash,
                ignore: false,
            },

            // === KEYWORDS ===
            // Control flow
            LexerRule {
                pattern: Regex::new(r"^if\b").unwrap(),
                token_type: |_| Token::If,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^else\b").unwrap(),
                token_type: |_| Token::Else,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^while\b").unwrap(),
                token_type: |_| Token::While,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^for\b").unwrap(),
                token_type: |_| Token::For,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^loop\b").unwrap(),
                token_type: |_| Token::Loop,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^return\b").unwrap(),
                token_type: |_| Token::Return,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^break\b").unwrap(),
                token_type: |_| Token::Break,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^continue\b").unwrap(),
                token_type: |_| Token::Continue,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^match\b").unwrap(),
                token_type: |_| Token::Match,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^case\b").unwrap(),
                token_type: |_| Token::Case,
                ignore: false,
            },
            // Declarations
            LexerRule {
                pattern: Regex::new(r"^var\b").unwrap(),
                token_type: |_| Token::Var,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^fn\b").unwrap(),
                token_type: |_| Token::Fn,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^class\b").unwrap(),
                token_type: |_| Token::Class,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^static\b").unwrap(),
                token_type: |_| Token::Static,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^pub\b").unwrap(),
                token_type: |_| Token::Pub,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^macro\b").unwrap(),
                token_type: |_| Token::Macro,
                ignore: false,
            },
            // Module system
            LexerRule {
                pattern: Regex::new(r"^import\b").unwrap(),
                token_type: |_| Token::Import,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^export\b").unwrap(),
                token_type: |_| Token::Export,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^use\b").unwrap(),
                token_type: |_| Token::Use,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^as\b").unwrap(),
                token_type: |_| Token::As,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^from\b").unwrap(),
                token_type: |_| Token::From,
                ignore: false,
            },
            // Exception handling
            LexerRule {
                pattern: Regex::new(r"^try\b").unwrap(),
                token_type: |_| Token::Try,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^catch\b").unwrap(),
                token_type: |_| Token::Catch,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^finally\b").unwrap(),
                token_type: |_| Token::Finally,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^raise\b").unwrap(),
                token_type: |_| Token::Raise,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^with\b").unwrap(),
                token_type: |_| Token::With,
                ignore: false,
            },
            // OOP
            LexerRule {
                pattern: Regex::new(r"^self\b").unwrap(),
                token_type: |_| Token::Self_,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^super\b").unwrap(),
                token_type: |_| Token::Super,
                ignore: false,
            },
            // Logical operators
            LexerRule {
                pattern: Regex::new(r"^and\b").unwrap(),
                token_type: |_| Token::And,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^or\b").unwrap(),
                token_type: |_| Token::Or,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^xor\b").unwrap(),
                token_type: |_| Token::Xor,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^not\b").unwrap(),
                token_type: |_| Token::Not,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^in\b").unwrap(),
                token_type: |_| Token::In,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^is\b").unwrap(),
                token_type: |_| Token::Is,
                ignore: false,
            },
            // Literals
            LexerRule {
                pattern: Regex::new(r"^true\b").unwrap(),
                token_type: |_| Token::True,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^false\b").unwrap(),
                token_type: |_| Token::False,
                ignore: false,
            },
            LexerRule {
                pattern: Regex::new(r"^null\b").unwrap(),
                token_type: |_| Token::Null,
                ignore: false,
            },

            // === IDENTIFIERS ===
            // Must come last to avoid matching keywords
            LexerRule {
                pattern: Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap(),
                token_type: |s| Token::Ident(s.to_string()),
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

        self.tokens.push(Token::Eof);
        Ok(self.tokens.clone())
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
                ch, self.line, self.column
            ))
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}