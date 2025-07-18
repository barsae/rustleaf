use crate::lexer::token::TokenType;
use std::collections::HashMap;

pub fn create_keywords_map() -> HashMap<String, TokenType> {
    let mut keywords = HashMap::new();

    // Insert all keywords
    keywords.insert("and".to_string(), TokenType::And);
    keywords.insert("break".to_string(), TokenType::Break);
    keywords.insert("case".to_string(), TokenType::Case);
    keywords.insert("catch".to_string(), TokenType::Catch);
    keywords.insert("class".to_string(), TokenType::Class);
    keywords.insert("continue".to_string(), TokenType::Continue);
    keywords.insert("else".to_string(), TokenType::Else);
    keywords.insert("false".to_string(), TokenType::False);
    keywords.insert("finally".to_string(), TokenType::Finally);
    keywords.insert("fn".to_string(), TokenType::Fn);
    keywords.insert("for".to_string(), TokenType::For);
    keywords.insert("from".to_string(), TokenType::From);
    keywords.insert("if".to_string(), TokenType::If);
    keywords.insert("in".to_string(), TokenType::In);
    keywords.insert("is".to_string(), TokenType::Is);
    keywords.insert("match".to_string(), TokenType::Match);
    keywords.insert("not".to_string(), TokenType::Not);
    keywords.insert("null".to_string(), TokenType::Null);
    keywords.insert("of".to_string(), TokenType::Of);
    keywords.insert("or".to_string(), TokenType::Or);
    keywords.insert("pub".to_string(), TokenType::Pub);
    // keywords.insert("raise".to_string(), TokenType::Raise); // raise is a function, not a keyword
    keywords.insert("require".to_string(), TokenType::Require);
    keywords.insert("return".to_string(), TokenType::Return);
    keywords.insert("root".to_string(), TokenType::Root);
    keywords.insert("self".to_string(), TokenType::Self_);
    keywords.insert("static".to_string(), TokenType::Static);
    keywords.insert("super".to_string(), TokenType::Super);
    keywords.insert("true".to_string(), TokenType::True);
    keywords.insert("try".to_string(), TokenType::Try);
    keywords.insert("use".to_string(), TokenType::Use);
    keywords.insert("var".to_string(), TokenType::Var);
    keywords.insert("while".to_string(), TokenType::While);
    keywords.insert("with".to_string(), TokenType::With);
    keywords.insert("xor".to_string(), TokenType::Xor);

    keywords
}
