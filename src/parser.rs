use crate::{Error, Lexer, Token, Value};
use rustc_hash::FxHashMap;
use std::borrow::Cow;

pub(crate) fn parse_value<I: Iterator<Item = u8>>(lexer: &mut Lexer<I>) -> Result<Value, Error> {
    match lexer.next() {
        Some(token) => {
            let (class, position) = token?;
            match class {
                Token::String(string) => Ok(Value::String(Cow::Owned(string))),
                Token::Number(number) => Ok(Value::Number(number)),
                Token::False => Ok(Value::Boolean(false)),
                Token::True => Ok(Value::Boolean(true)),
                Token::Null => Ok(Value::Null),
                Token::BeginArray => Ok(Value::Array(parse_array(lexer)?.into_boxed_slice())),
                Token::BeginObject => Ok(Value::Object(parse_object(lexer)?)),
                _ => Err(Error::UnexpectedToken(class, position)),
            }
        }
        None => Err(Error::UnexpectedEndOfStream(
            lexer.current_position().clone(),
        )),
    }
}

fn parse_array<I: Iterator<Item = u8>>(lexer: &mut Lexer<I>) -> Result<Vec<Value>, Error> {
    let mut values = match lexer.next() {
        Some(token) => {
            let (class, position) = token?;
            match class {
                Token::EndArray => return Ok(Vec::new()),
                _ => {
                    lexer.unget(class, position);
                    vec![parse_value(lexer)?]
                }
            }
        }
        None => {
            return Err(Error::UnexpectedEndOfStream(
                lexer.current_position().clone(),
            ))
        }
    };

    loop {
        match lexer.next() {
            Some(token) => {
                let (class, position) = token?;
                match class {
                    Token::ValueSeperator => {}
                    Token::EndArray => return Ok(values),
                    _ => return Err(Error::UnexpectedToken(class, position)),
                }
            }
            None => {
                return Err(Error::UnexpectedEndOfStream(
                    lexer.current_position().clone(),
                ))
            }
        }

        values.push(parse_value(lexer)?)
    }
}

fn parse_object<I: Iterator<Item = u8>>(
    lexer: &mut Lexer<I>,
) -> Result<FxHashMap<String, Value>, Error> {
    let mut values = FxHashMap::default();
    match lexer.next() {
        Some(token) => {
            let (class, position) = token?;
            match class {
                Token::EndObject => return Ok(values),
                _ => {
                    lexer.unget(class, position);
                    let (key, value) = parse_member(lexer)?;
                    values.insert(key, value);
                }
            }
        }
        None => {
            return Err(Error::UnexpectedEndOfStream(
                lexer.current_position().clone(),
            ))
        }
    };

    loop {
        match lexer.next() {
            Some(token) => {
                let (class, position) = token?;
                match class {
                    Token::ValueSeperator => {}
                    Token::EndObject => return Ok(values),
                    _ => return Err(Error::UnexpectedToken(class, position)),
                }
            }
            None => {
                return Err(Error::UnexpectedEndOfStream(
                    lexer.current_position().clone(),
                ))
            }
        }

        let (key, value) = parse_member(lexer)?;
        values.insert(key, value);
    }
}

fn parse_member<I: Iterator<Item = u8>>(lexer: &mut Lexer<I>) -> Result<(String, Value), Error> {
    let key = match lexer.next() {
        Some(token) => {
            let (class, position) = token?;
            match class {
                Token::String(string) => string,
                _ => return Err(Error::UnexpectedToken(class, position)),
            }
        }
        None => {
            return Err(Error::UnexpectedEndOfStream(
                lexer.current_position().clone(),
            ))
        }
    };

    match lexer.next() {
        Some(token) => {
            let (class, position) = token?;
            match class {
                Token::NameSeperator => {}
                _ => return Err(Error::UnexpectedToken(class, position)),
            }
        }
        None => {
            return Err(Error::UnexpectedEndOfStream(
                lexer.current_position().clone(),
            ))
        }
    };

    let value = parse_value(lexer)?;

    Ok((key, value))
}
