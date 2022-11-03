use crate::{Error, Position, Token};

pub(crate) struct Lexer<I: Iterator<Item = u8>> {
    iterator: I,
    next: Option<u8>,
    current_position: Position,
    unget: Option<(Token, Position)>,
}

impl<I: Iterator<Item = u8>> Lexer<I> {
    pub(crate) fn new(mut iterator: I) -> Self {
        Lexer {
            next: iterator.next(),
            iterator,
            current_position: Position::new(1, 1),
            unget: None,
        }
    }

    pub(crate) fn current_position(&self) -> &Position {
        &self.current_position
    }

    pub(crate) fn unget(&mut self, token: Token, position: Position) {
        self.unget = Some((token, position))
    }

    fn peek_char(&self) -> Option<u8> {
        self.next
    }

    fn next_char(&mut self) -> Option<u8> {
        let ret = self.next;
        self.next = self.iterator.next();
        self.current_position.increment(ret);
        ret
    }

    fn match_string(&mut self, string: &[u8]) -> Result<(), Error> {
        for byte in string {
            let position = self.current_position.clone();
            match self.next_char() {
                Some(c) => match *byte == c {
                    true => continue,
                    false => return Err(Error::UnexpectedCharacter(c, position)),
                },
                None => return Err(Error::UnexpectedEndOfStream(position)),
            }
        }

        Ok(())
    }

    fn parse_number(
        &mut self,
        minus: bool,
        first_char: u8,
        position: Position,
    ) -> Result<f64, Error> {
        let mut string = String::new();
        if minus {
            string.push('-');
        }
        string.push(first_char as char);

        if first_char == 0 {
            match self.peek_char() {
                Some(c) => match c {
                    b'.' => {
                        string.push('.');
                        self.next_char();
                        return self.parse_frac(string);
                    }
                    b'e' => {
                        string.push('e');
                        self.next_char();
                        return self.parse_exp(string);
                    }
                    _ => return Ok(0.),
                },
                None => return Err(Error::UnexpectedEndOfStream(position)),
            }
        }

        while let Some(c) = self.peek_char() {
            if c.is_ascii_digit() {
                string.push(c as char);
                self.next_char();
                continue;
            }

            match c {
                b'.' => {
                    string.push('.');
                    self.next_char();
                    return self.parse_frac(string);
                }
                b'e' => {
                    string.push('e');
                    self.next_char();
                    return self.parse_exp(string);
                }
                _ => break,
            }
        }

        Ok(string.parse().unwrap())
    }

    fn parse_frac(&mut self, mut string: String) -> Result<f64, Error> {
        let position = self.current_position.clone();
        match self.next_char() {
            Some(c) => match c.is_ascii_digit() {
                true => string.push(c as char),
                false => return Err(Error::UnexpectedCharacter(c, position)),
            },
            None => return Err(Error::UnexpectedEndOfStream(position)),
        }

        while let Some(c) = self.peek_char() {
            if c.is_ascii_digit() {
                string.push(c as char);
                self.next_char();
                continue;
            }

            if c == b'e' {
                string.push('e');
                self.next_char();
                return self.parse_exp(string);
            }

            break;
        }

        Ok(string.parse().unwrap())
    }

    fn parse_exp(&mut self, mut string: String) -> Result<f64, Error> {
        let position = self.current_position.clone();
        match self.next_char() {
            Some(c) => match c.is_ascii_digit() {
                true => string.push(c as char),
                false => match c {
                    b'+' | b'-' => {
                        if c == b'-' {
                            string.push(c as char);
                        }

                        let position = self.current_position.clone();
                        match self.next_char() {
                            Some(c) => match c.is_ascii_digit() {
                                true => string.push(c as char),
                                false => return Err(Error::UnexpectedCharacter(c, position)),
                            },
                            None => return Err(Error::UnexpectedEndOfStream(position)),
                        }
                    }
                    _ => return Err(Error::UnexpectedCharacter(c, position)),
                },
            },
            None => return Err(Error::UnexpectedEndOfStream(position)),
        }

        while let Some(c) = self.peek_char() {
            if c.is_ascii_digit() {
                string.push(c as char);
                self.next_char();
                continue;
            }

            break;
        }

        Ok(string.parse().unwrap())
    }

    fn parse_string(&mut self) -> Result<String, Error> {
        let mut string = Vec::new();

        let mut position = self.current_position.clone();
        while let Some(c) = self.next_char() {
            if c <= 0x1F {
                return Err(Error::UnexpectedCharacter(c, position));
            }

            match c {
                b'"' => match String::from_utf8(string) {
                    Ok(string) => return Ok(string),
                    Err(error) => return Err(Error::InvalidUTF8(error, position)),
                },
                b'\\' => {
                    position = self.current_position.clone();
                    match self.next_char() {
                        Some(c) => match c {
                            b'"' => string.push(b'"'),
                            b'\\' => string.push(b'\\'),
                            b'/' => string.push(b'/'),
                            b'b' => string.push(0x08),
                            b'f' => string.push(0x0C),
                            b'n' => string.push(b'\n'),
                            b'r' => string.push(b'\r'),
                            b't' => string.push(b'\t'),
                            b'u' => {
                                let mut value = 0;
                                for _ in 0..4 {
                                    value <<= 4;

                                    let position = self.current_position.clone();
                                    match self.next_char() {
                                        Some(c) => match c.is_ascii_hexdigit() {
                                            true => {
                                                value += (c as char).to_digit(16).unwrap() as u16
                                            }
                                            false => {
                                                return Err(Error::UnexpectedCharacter(c, position))
                                            }
                                        },
                                        None => return Err(Error::UnexpectedEndOfStream(position)),
                                    }
                                }

                                if value > u8::MAX as u16 {
                                    string.push(value.wrapping_shr(8) as u8);
                                    string.push((value & 0xFF) as u8);
                                } else {
                                    string.push(value as u8);
                                }
                            }
                            _ => return Err(Error::UnexpectedCharacter(c, position)),
                        },
                        None => return Err(Error::UnexpectedEndOfStream(position)),
                    }
                }
                _ => string.push(c),
            }

            position = self.current_position.clone();
        }

        Err(Error::UnexpectedEndOfStream(self.current_position.clone()))
    }
}

impl<I: Iterator<Item = u8>> Iterator for Lexer<I> {
    type Item = Result<(Token, Position), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.unget.take() {
            Some(token) => return Some(Ok(token)),
            None => {}
        }

        // Skip whitespace
        while let Some(c) = self.peek_char() {
            match c {
                b' ' | b'\t' | b'\n' | b'\r' => {
                    self.next_char();
                }
                _ => break,
            }
        }

        let position = self.current_position.clone();
        let c = match self.next_char() {
            Some(c) => c,
            None => return None,
        };

        if c.is_ascii_digit() {
            return Some(match self.parse_number(false, c, position.clone()) {
                Ok(number) => Ok((Token::Number(number), position)),
                Err(error) => Err(error),
            });
        }

        Some(Ok((
            match c {
                b'[' => Token::BeginArray,
                b'{' => Token::BeginObject,
                b']' => Token::EndArray,
                b'}' => Token::EndObject,
                b':' => Token::NameSeperator,
                b',' => Token::ValueSeperator,
                b'f' => match self.match_string(b"alse") {
                    Ok(_) => Token::False,
                    Err(error) => return Some(Err(error)),
                },
                b'n' => match self.match_string(b"ull") {
                    Ok(_) => Token::Null,
                    Err(error) => return Some(Err(error)),
                },
                b't' => match self.match_string(b"rue") {
                    Ok(_) => Token::True,
                    Err(error) => return Some(Err(error)),
                },
                b'-' => {
                    let position = self.current_position.clone();
                    match self.next_char() {
                        Some(c) => match c.is_ascii_digit() {
                            true => match self.parse_number(true, c, position) {
                                Ok(number) => Token::Number(number),
                                Err(error) => return Some(Err(error)),
                            },
                            false => return Some(Err(Error::UnexpectedCharacter(c, position))),
                        },
                        None => return Some(Err(Error::UnexpectedEndOfStream(position))),
                    }
                }
                b'"' => match self.parse_string() {
                    Ok(string) => Token::String(string),
                    Err(error) => return Some(Err(error)),
                },
                _ => return Some(Err(Error::UnexpectedCharacter(c, position))),
            },
            position,
        )))
    }
}
