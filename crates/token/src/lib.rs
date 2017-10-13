use std::num::ParseIntError;
use std::str::CharIndices;
use std::str::FromStr;
use std::fmt;

pub type Spanned<T> = Result<(usize, T, usize), Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Print,
    Equals,
    LeftParens,
    RightParens,
    Minus,
    Plus,
    Input,
    Lt,
    Gt,
    Comma,
    Terminator(char),
    DecimalI32(i32),
    Name(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidIntegerLiteral(ParseIntError),
    UnexpectedEof,
    UnexpectedChar,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidIntegerLiteral(ref e) => write!(f, "{}", e),
            Error::UnexpectedEof => write!(f, "unexpected end of file"),
            Error::UnexpectedChar => write!(f, "unexpected character"),
        }
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidIntegerLiteral(_) => "invalid integer literal",
            Error::UnexpectedEof => "unexpected end of file",
            Error::UnexpectedChar => "unexpected character",
        }
    }
}

/// Very closely mirrors lalrpop implementation
/// https://github.com/nikomatsakis/lalrpop/blob/master/lalrpop/src/tok/mod.rs
pub struct Stream<'input> {
    text: &'input str,
    chars: CharIndices<'input>,
    next: Option<(usize, char)>,
}

impl<'input> Stream<'input> {
    pub fn new(text: &'input str) -> Self {
        ////println!("text: {:?}", text);
        let mut lexer = Stream {
            text,
            chars: text.char_indices(),
            next: None,
        };
        // self.next will now contain correct initial-peek value
        lexer.next = lexer.chars.next();

        lexer
    }

    // TODO Better naming here
    pub fn peek(&mut self) -> Option<(usize, char)> {
        self.next
    }

    // TODO Better naming here
    pub fn consume1(&mut self) {
        self.next = self.chars.next();
    }

    /// Returns Some(index) once should_stop(c) == true or EOF,
    /// returns None if EOF immediately. peek() will return the
    /// c where the stop condition occurred.
    pub fn consume_until<F>(&mut self, mut should_stop: F) -> Option<usize>
    where
        F: FnMut(usize, char) -> bool,
    {
        let mut i0 = match self.peek() {
            Some((i, _)) => i,
            None => return None,
        };
        loop {
            let (i, c) = match self.peek() {
                Some((i, c)) => (i, c),
                None => return Some(i0 + 1),
            };
            i0 = i;
            if should_stop(i0, c) {
                return Some(i0);
            }
            self.consume1();
        }
    }

    pub fn decimal_i32(&mut self) -> Spanned<()> {
        // first check to see if first digit is 0, if so, immediately
        // return, because 01 should not be parsed as 1, it should be [zero, one]
        // at least, that's how I'm writing it, then at a higher level
        // there will be a grammar rule error
        let start = match self.peek() {
            Some((i, '0')) => {
                self.consume1();
                return Ok((i, (), i + 1));
            }
            Some((i, _)) => i,
            None => return Err(Error::UnexpectedEof),
        };
        self.consume_until(|_, c| match c {
            '0'...'9' => false,
            _ => true,
        }).map(|end| (start, (), end))
            .ok_or(Error::UnexpectedEof)
    }

    pub fn name_or_keyword(&mut self, start: usize) -> Spanned<Token> {
        // I don't require that first character is non-decimal-digit, since that is
        // already checked in Iterator::next()
        self.consume_until(|_, c| match c {
            'a'...'z' | 'A'...'Z' | '0'...'9' | '_' => false,
            _ => true,
        }).map(|end| {
                let s = &self.text[start..end];
                let tok = match s {
                    "print" => Token::Print,
                    // This is just a special case for p0, it will be removed and replaced
                    // by generic call_func(name, args) later
                    "input" => Token::Input,
                    _ => Token::Name(s.into()),
                };
                (start, tok, end)
            })
            .ok_or(Error::UnexpectedEof)
    }
}

impl<'input> Iterator for Stream<'input> {
    type Item = Spanned<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (i, c) = match self.peek() {
                Some(item) => item,
                None => {
                    //println!("eof");
                    return None; // EOF
                }
            };
            //println!("i={}, c={:?}: {:?}", i, c, &self.text[i..]);
            let single_char_tok = match c {
                '\n' => Some(Token::Terminator('\n')),
                '=' => Some(Token::Equals),
                '+' => Some(Token::Plus),
                '-' => {
                    // could be parsed as decimal_i32 or unary_neg
                    // first mark index and consume "-"
                    let minus_index = i;
                    //println!("minus_index: {}", minus_index);
                    self.consume1();
                    if let Some((_, '0'...'9')) = self.peek() {
                        let result = self.decimal_i32().and_then(|(_, _, end)| {
                            let s = &self.text[minus_index..end];
                            i32::from_str(s)
                                .map(|i| (minus_index, Token::DecimalI32(i), end))
                                .map_err(Error::InvalidIntegerLiteral)
                        });
                        return Some(result);
                    }
                    // if the next character is eof or not 0-9, unary_neg
                    return Some(Ok((minus_index, Token::Minus, minus_index + 1)));
                }
                '(' => Some(Token::LeftParens),
                ')' => Some(Token::RightParens),
                '<' => Some(Token::Lt),
                '>' => Some(Token::Gt),
                ',' => Some(Token::Comma),
                ';' => Some(Token::Terminator(';')),
                '#' => {
                    // eat comment
                    loop {
                        let c = match self.peek() {
                            Some((_, c)) => c,
                            None => return None,
                        };
                        if c == '\n' {
                            break;
                        }
                        self.consume1();
                    }
                    continue;
                }
                _ => None,
            };

            if let Some(tok) = single_char_tok {
                //println!("parsed single_char_tok: {:?}\n", tok);
                // consume token before returning
                self.consume1();
                return Some(Ok((i, tok, i + 1)));
            }

            // if c is numeric, can't be a name, must be decimal_i32
            // else, must be name
            let parsed = match c {
                '0'...'9' => {
                    self.decimal_i32().and_then(|(start, (), end)| {
                        (&self.text[start..end]).parse()
                            .map(|i| (start, Token::DecimalI32(i), end))
                            .map_err(|e: ParseIntError| Error::InvalidIntegerLiteral(e))
                    })
                }
                'a'...'z' | 'A'...'Z' | '_' => self.name_or_keyword(i),
                _ => {
                    if !c.is_whitespace() {
                        return Some(Err(Error::UnexpectedChar));
                    }
                    //println!("skipping over {:?}\n", c);
                    self.consume1();
                    continue;
                }
            };
            //println!("parsed: {:?}\n", parsed);
            return Some(parsed);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Stream, Spanned, Token, Error};
    use std::iter::{Iterator, IntoIterator};

    trait CollectTokens {
        fn collect_tokens(self) -> Result<Vec<Token>, Error>;
    }

    impl<T, I> CollectTokens for T
    where
        T: IntoIterator<Item = Spanned<Token>, IntoIter = I>,
        I: Iterator<Item = Spanned<Token>>,
    {
        fn collect_tokens(self) -> Result<Vec<Token>, Error> {
            self.into_iter().map(|r| r.map(|(_, t, _)| t)).collect()
        }
    }

    #[test]
    fn all_tokens() {
        let tokens = "\n = - + ( ) < > , ; # abcdefk akdkdkdl;lskdfjda \n identifier 123 0";
        let tokens = Stream::new(tokens).collect_tokens().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Terminator('\n'),
                Token::Equals,
                Token::Minus,
                Token::Plus,
                Token::LeftParens,
                Token::RightParens,
                Token::Lt,
                Token::Gt,
                Token::Comma,
                Token::Terminator(';'),
                Token::Terminator('\n'),
                Token::Name("identifier".into()),
                Token::DecimalI32(123),
                Token::DecimalI32(0),
            ]
        );
    }

    #[test]
    fn octal_number() {
        let octal = "099";
        let tokens = Stream::new(octal).collect_tokens().unwrap();
        assert_eq!(tokens, vec![Token::DecimalI32(0), Token::DecimalI32(99)]);
    }
}
