use editor::Lexer;

fn main() {
    let input = r#""""test""""#;
    println!("Input: {:?}", input);
    println!("Input chars: {:?}", input.chars().collect::<Vec<_>>());
    
    let mut lexer = Lexer::new(input);
    let (tokens, errors) = lexer.tokenize();
    
    println!("Errors: {:?}", errors);
    println!("Tokens: {:?}", tokens);
}