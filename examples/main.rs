use json_parser::Input;
use json_parser::Lexer;
use json_parser::Token;

pub fn main() {
    let lexer = Lexer::new(Input::new(r#"0 0 1.0 null false"#.into()));

    for token in lexer {
        match token {
            Token::StringLiteral(s) => println!("StringLiteral(\"{}\")", s),
            _ => println!("{:?}", token),
        }
    }
}
