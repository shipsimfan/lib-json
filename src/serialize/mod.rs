use data_format::Serialize;
use error::Result;
use escape::Escape;
use formatter::{CompactFormatter, Formatter, PrettyFormatter};
use list::ListSerializer;
use map::MapSerializer;
use serializer::Serializer;
use std::io::Write;

mod error;
mod escape;
mod formatter;
mod list;
mod map;
mod serializer;

pub use error::SerializeError;

/// Serializes `value` into a compact JSON [`String`]
pub fn to_str<T: Serialize + ?Sized>(value: &T) -> Result<String> {
    to_bytes(value).map(|bytes| unsafe { String::from_utf8_unchecked(bytes) })
}

/// Serializes `value` into a pretty JSON [`String`]
pub fn to_str_pretty<T: Serialize + ?Sized>(value: &T) -> Result<String> {
    to_bytes_pretty(value).map(|bytes| unsafe { String::from_utf8_unchecked(bytes) })
}

/// Serializes `value` into a compact JSON [`Vec<u8>`]
pub fn to_bytes<T: Serialize + ?Sized>(value: &T) -> Result<Vec<u8>> {
    let mut output = Vec::new();
    to_write(value, &mut output).map(|_| output)
}

/// Serializes `value` into a pretty JSON [`Vec<u8>`]
pub fn to_bytes_pretty<T: Serialize + ?Sized>(value: &T) -> Result<Vec<u8>> {
    let mut output = Vec::new();
    to_write_pretty(value, &mut output).map(|_| output)
}

/// Serializes `value` into compact JSON and writes it to `output`
pub fn to_write<T: Serialize + ?Sized, W: Write>(value: &T, output: W) -> Result<()> {
    let mut serializer = Serializer::compact(output);
    value.serialize(&mut serializer)
}

/// Serializes `value` into pretty JSON and writes it to `output`
pub fn to_write_pretty<T: Serialize + ?Sized, W: Write>(value: &T, output: W) -> Result<()> {
    let mut serializer = Serializer::pretty(output);
    value.serialize(&mut serializer)
}
