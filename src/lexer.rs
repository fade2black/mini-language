use crate::char::Char;
use crate::token::Token;
use utf8_read::Char as Utf8Char;
use utf8_read::Reader;

pub struct Lexer<T>
where
    T: std::io::Read,
{
    reader: Reader<T>,
    pub lexeme: String,
    last_char: Char,
    line: usize,
}

impl<T> Lexer<T>
where
    T: std::io::Read,
{
    pub fn new(src: T) -> Self {
        let reader = Reader::new(src);
        let lexeme = String::new();
        let last_char = Char::new(Utf8Char::Char(' '));
        let line = 1;

        Self {
            reader,
            lexeme,
            last_char,
            line,
        }
    }

    pub fn get_line_numer(&self) -> usize {
        self.line
    }

    pub fn get_token(&mut self) -> Token {
        self.lexeme.clear();
        self.skip_whitespace();

        // identifier: [a-zA-Z][a-zA-Z0-9]*
        if self.last_char.is_alphabetic() {
            self.get_identifier();

            return match self.lexeme.as_str() {
                "def" => Token::Define,
                "if" => Token::If,
                "else" => Token::Else,
                "then" => Token::Then,
                _ => Token::Identifier,
            };
        }

        // Number: [0-9]?(.?[0-9])
        if self.last_char.is_digit() {
            self.get_number();
            return Token::Number;
        }

        if self.last_char == '#' {
            self.skip_comment();
            if !self.last_char.is_eof() {
                return self.get_token();
            }
        }

        if self.last_char.is_eof() {
            return Token::Eof;
        }

        self.other()
    }

    pub fn get_char(&mut self) {
        match self.reader.next_char() {
            Ok(utf8ch) => self.last_char = Char::new(utf8ch),
            Err(e) => panic!("{}", e),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.last_char.is_whitespace() {
            if self.last_char.is_newline() {
                self.line += 1;
            }

            self.get_char();
        }
    }

    fn get_identifier(&mut self) {
        loop {
            self.lexeme.push(self.last_char.as_char());
            self.get_char();
            if !self.last_char.is_alphanumeric() {
                break;
            }
        }
    }

    fn get_number(&mut self) {
        self.get_digits();

        if self.last_char == '.' {
            self.get_digits();
        }
    }

    fn get_digits(&mut self) {
        loop {
            self.lexeme.push(self.last_char.as_char());
            self.get_char();
            if !self.last_char.is_digit() {
                break;
            }
        }
    }

    fn skip_comment(&mut self) {
        loop {
            self.get_char();
            if self.last_char.is_eof() || self.last_char == '\r' || self.last_char == '\n' {
                break;
            }
        }
    }

    fn other(&mut self) -> Token {
        let mut need_next_char = true;

        let token = match self.last_char.as_char() {
            '(' => Token::Lpar,
            ')' => Token::Rpar,
            '>' => Token::Greater,
            '<' => {
                self.get_char();
                if self.last_char == '>' {
                    Token::NotEq
                } else {
                    need_next_char = false;
                    Token::Less
                }
            }
            '*' => Token::Star,
            '-' => Token::Minus,
            '/' => Token::Slash,
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '|' => Token::Or,
            '&' => Token::And,
            '=' => {
                self.get_char();
                if self.last_char == '=' {
                    Token::Equal
                } else {
                    need_next_char = false;
                    Token::None
                }
            }
            _ => Token::InvalidChar,
        };

        if need_next_char {
            self.get_char();
        }
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn lexer_with_source(src: &str) -> Lexer<Cursor<&str>> {
        Lexer::new(Cursor::new(src))
    }

    #[test]
    fn it_returns_a_single_char() {
        let mut lexer = lexer_with_source("foobar");

        lexer.get_char();
        assert_eq!(lexer.last_char, 'f');
    }

    #[test]
    fn it_returns_eof() {
        let mut lexer = lexer_with_source("foo");

        for _ in 0..3 {
            lexer.get_char();
        }

        lexer.get_char();
        assert!(lexer.last_char.is_eof());
    }

    #[test]
    fn it_returns_multibyte_chars() {
        let mut lexer = lexer_with_source("🙂👍");

        lexer.get_char();
        assert_eq!(lexer.last_char, '🙂');

        lexer.get_char();
        assert_eq!(lexer.last_char, '👍');
    }

    #[test]
    fn it_parses_identifires() {
        let src = "  ident1  \n\t ident2 ident3";
        let mut lexer = lexer_with_source(src);
        let identifiers = vec!["ident1", "ident2", "ident3"];

        for ident in identifiers {
            assert_eq!(lexer.get_token(), Token::Identifier);
            assert_eq!(lexer.lexeme, ident);
        }
    }

    #[test]
    fn it_parses_define() {
        let mut lexer = lexer_with_source("def foobar");

        assert_eq!(lexer.get_token(), Token::Define);
        assert_eq!(lexer.lexeme, "def");
    }

    #[test]
    fn it_parses_int_number() {
        let mut lexer = lexer_with_source("  1234 ");

        assert_eq!(lexer.get_token(), Token::Number);
        assert_eq!(lexer.lexeme, "1234");
    }

    #[test]
    fn it_parses_fractional_number() {
        let mut lexer = lexer_with_source("  1234.12 ");

        assert_eq!(lexer.get_token(), Token::Number);
        assert_eq!(lexer.lexeme, "1234.12");
    }

    #[test]
    fn it_parses_fractional_number_with_missin_fraction() {
        let mut lexer = lexer_with_source("  1234. ");

        assert_eq!(lexer.get_token(), Token::Number);
        assert_eq!(lexer.lexeme, "1234.");
    }

    #[test]
    fn it_skips_comment() {
        let mut lexer =
            lexer_with_source("# comment 1 \n # comment 2 \n\r foobar # comment 3\n123");

        assert_eq!(lexer.get_token(), Token::Identifier);
        assert_eq!(lexer.lexeme, "foobar");
        assert_eq!(lexer.get_token(), Token::Number);
        assert_eq!(lexer.lexeme, "123");
    }

    #[test]
    fn it_returns_invalid_char() {
        let mut lexer = lexer_with_source("$123");

        assert_eq!(lexer.get_token(), Token::InvalidChar);
    }
}
