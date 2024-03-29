mod deserialize;
mod error;
mod lexer;
mod parser;
mod position;
mod serialize;
mod token;
mod r#type;
mod value;

pub(crate) use lexer::Lexer;
pub(crate) use token::Token;

pub use deserialize::{deserialize, Deserialize};
pub use error::Error;
pub use position::Position;
pub use r#type::Type;
pub use serialize::Serialize;
pub use value::Value;

pub use json_macro::*;

pub use rustc_hash::FxHashMap;

pub fn parse<I: Iterator<Item = u8>>(stream: I) -> Result<Value, Error> {
    let mut lexer = Lexer::new(stream);

    let value = parser::parse_value(&mut lexer)?;

    match lexer.next() {
        Some(token) => {
            let (class, position) = token?;
            Err(Error::UnexpectedToken(class, position))
        }
        None => Ok(value),
    }
}

pub fn parse_and_deserialize<I: Iterator<Item = u8>, T: Deserialize>(
    stream: I,
) -> Result<T, Error> {
    T::deserialize(parse(stream)?, None)
}
