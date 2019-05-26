#[derive(Debug)]
pub enum Token {
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    Colon,
    Comma,
    TrueKeyword,
    FalseKeyword,
    NullKeyword,
    StringLiteral(String),
    NumberLiteral(f64),
}
