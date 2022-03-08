use std::cmp::PartialEq;
use utf8_read::Char as Utf8Char;

#[derive(Debug)]
pub struct Char {
    value: Utf8Char,
}

impl Char {
    pub fn new(value: Utf8Char) -> Self {
        Self { value }
    }

    pub fn is_alphabetic(&self) -> bool {
        match self.value {
            Utf8Char::Char(ch) => ch.is_alphabetic(),
            _ => false,
        }
    }

    pub fn is_alphanumeric(&self) -> bool {
        match self.value {
            Utf8Char::Char(ch) => ch.is_alphanumeric(),
            _ => false,
        }
    }

    pub fn is_eof(&self) -> bool {
        match self.value {
            Utf8Char::Char(_) => false,
            _ => true,
        }
    }

    pub fn is_digit(&self) -> bool {
        match self.value {
            Utf8Char::Char(ch) => ch.is_digit(10),
            _ => false,
        }
    }

    pub fn is_whitespace(&self) -> bool {
        match self.value {
            Utf8Char::Char(ch) => ch.is_whitespace(),
            _ => false,
        }
    }

    pub fn as_char(&self) -> char {
        match self.value {
            Utf8Char::Char(ch) => ch,
            Utf8Char::Eof => panic!("unable to convert utf8_read::Char::Eof"),
            Utf8Char::NoData => panic!("unable to convert utf8_read::Char::NoData"),
        }
    }

    pub fn is_newline(&self) -> bool {
        *self == '\r' || *self == '\n'
    }
}

impl PartialEq<char> for Char {
    fn eq(&self, other: &char) -> bool {
        match self.value {
            Utf8Char::Char(ch) => ch == *other,
            _ => false,
        }
    }
}

impl PartialEq<Char> for char {
    fn eq(&self, other: &Char) -> bool {
        match other.value {
            Utf8Char::Char(ch) => ch == *self,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_check_alphabetic() {
        let ch = Char {
            value: Utf8Char::Char('a'),
        };

        assert!(ch.is_alphabetic());
    }

    #[test]
    fn it_check_numericity() {
        let ch = Char {
            value: Utf8Char::Char('1'),
        };

        assert!(ch.is_digit());
    }

    #[test]
    fn it_check_alphanumericity() {
        let ch1 = Char {
            value: Utf8Char::Char('1'),
        };

        let ch2 = Char {
            value: Utf8Char::Char('a'),
        };

        assert!(ch1.is_alphanumeric());
        assert!(ch2.is_alphanumeric());
    }

    #[test]
    fn it_check_whitespce() {
        let space = Char {
            value: Utf8Char::Char(' '),
        };
        let new_line = Char {
            value: Utf8Char::Char('\n'),
        };
        let cr = Char {
            value: Utf8Char::Char('\r'),
        };
        let tab = Char {
            value: Utf8Char::Char('\t'),
        };

        assert!(space.is_whitespace());
        assert!(new_line.is_whitespace());
        assert!(cr.is_whitespace());
        assert!(tab.is_whitespace());
    }

    #[test]
    fn it_converts_into_char() {
        let ch = Char {
            value: Utf8Char::Char('a'),
        };

        assert_eq!('a', ch.as_char());
    }

    #[test]
    fn it_compares_with_char() {
        let ch = Char {
            value: Utf8Char::Char('a'),
        };

        assert_eq!(ch, 'a');
        assert_eq!('a', ch);

        assert_ne!(ch, 'b');
        assert_ne!('b', ch);
    }
}
