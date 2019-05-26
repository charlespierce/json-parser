use crate::input::Input;
use crate::token::Token;

pub struct Lexer {
    input: Input,
}

impl Lexer {
    pub fn new(input: Input) -> Self {
        Lexer { input }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.input.cur() {
                Some('\t') | Some('\n') | Some('\r') | Some(' ') => self.input.bump(),
                _ => break,
            }
        }
    }

    fn read_keyword(&mut self, expected: &str) {
        for chr in expected.chars() {
            match self.input.cur() {
                Some(c) if c == chr => self.input.bump(),
                Some(c) => panic!("Unexpected character '{}'", c),
                None => panic!("Unexpected end of file"),
            }
        }
    }

    fn read_string(&mut self) -> String {
        self.input.bump(); // Consume opening '"' character
        let mut result = String::new();

        while let Some(chr) = self.input.cur() {
            match chr {
                '"' => {
                    self.input.bump(); // Consume closing '"' character
                    return result;
                }
                '\n' => panic!("Unclosed string literal"),
                '\\' => result.push(self.read_escape()),
                c if c >= ' ' && c <= '\u{10ffff}' => {
                    self.input.bump(); // Consume current character
                    result.push(c)
                }
                c => panic!("Unexpected character '{}'", c),
            }
        }

        panic!("Unclosed string literal");
    }

    fn read_escape(&mut self) -> char {
        self.input.bump(); // Consume opening '\' character

        let chr = match self.input.cur() {
            Some(c) => c,
            None => panic!("Unexpected end of file in escape sequence"),
        };

        match chr {
            '"' | '\\' | '/' => {
                self.input.bump();
                chr
            }
            'b' => {
                self.input.bump();
                '\x08'
            }
            'f' => {
                self.input.bump();
                '\x0C'
            }
            'n' => {
                self.input.bump();
                '\n'
            }
            'r' => {
                self.input.bump();
                '\r'
            }
            't' => {
                self.input.bump();
                '\t'
            }
            'u' => {
                self.input.bump();
                std::char::from_u32(self.read_hex_digits())
                    .expect("Hex digits are a valid character escape")
            }
            c => panic!("Unexpected character '{}'", c),
        }
    }

    fn read_hex_digits(&mut self) -> u32 {
        let mut result = 0;
        for _ in 0..4 {
            match self.input.cur() {
                Some(c) if c.is_digit(16) => {
                    self.input.bump();
                    result *= 16;
                    result += c.to_digit(16).unwrap();
                }
                Some(c) => panic!("Unexpected character '{}'", c),
                None => panic!("Unexpected end of file in escape sequence"),
            }
        }
        result
    }

    fn read_number(&mut self) -> f64 {
        format!("{}{}{}", self.read_int(), self.read_frac(), self.read_exp())
            .parse()
            .expect("Could not parse numeric literal")
    }

    fn read_int(&mut self) -> String {
        let mut result = String::new();

        if self.input.cur() == Some('-') {
            result.push('-');
            self.input.bump();
        }

        match self.input.cur() {
            Some('0') => {
                result.push('0');
                self.input.bump();

                if let Some(c) = self.input.cur() {
                    if c.is_digit(10) {
                        panic!("Numeric literals cannot have leading zeroes");
                    }
                }
            }
            _ => self.read_digits(&mut result),
        }

        result
    }

    fn read_frac(&mut self) -> String {
        let mut result = String::new();

        if self.input.cur() == Some('.') {
            result.push('.');
            self.input.bump();
            self.read_digits(&mut result);
        }

        result
    }

    fn read_exp(&mut self) -> String {
        let mut result = String::new();

        if self.input.cur() == Some('e') || self.input.cur() == Some('E') {
            result.push('e');
            self.input.bump();

            match self.input.cur() {
                Some('+') => self.input.bump(),
                Some('-') => {
                    result.push('-');
                    self.input.bump();
                }
                _ => {}
            }
            self.read_digits(&mut result);
        }

        result
    }

    fn read_digits(&mut self, buffer: &mut String) {
        let mut found_digits = false;

        loop {
            match self.input.cur() {
                Some(c) if c.is_digit(10) => {
                    buffer.push(c);
                    self.input.bump();
                    found_digits = true;
                }
                _ => break,
            }
        }

        if !found_digits {
            panic!("Unexpected end of numeric literal");
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        let token = match self.input.cur()? {
            '[' => {
                self.input.bump();
                Token::OpenBracket
            }
            ']' => {
                self.input.bump();
                Token::CloseBracket
            }
            '{' => {
                self.input.bump();
                Token::OpenBrace
            }
            '}' => {
                self.input.bump();
                Token::CloseBrace
            }
            ':' => {
                self.input.bump();
                Token::Colon
            }
            ',' => {
                self.input.bump();
                Token::Comma
            }
            't' => {
                self.read_keyword("true");
                Token::TrueKeyword
            }
            'f' => {
                self.read_keyword("false");
                Token::FalseKeyword
            }
            'n' => {
                self.read_keyword("null");
                Token::NullKeyword
            }
            '"' => {
                let value = self.read_string();
                Token::StringLiteral(value)
            }
            c if is_number_start(c) => {
                let value = self.read_number();
                Token::NumberLiteral(value)
            }
            c => panic!("Unexpected character '{}'", c),
        };

        Some(token)
    }
}

fn is_number_start(c: char) -> bool {
    c == '-' || c.is_digit(10)
}
