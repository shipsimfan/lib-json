use crate::Result;
use data_format::Serialize;
use formatter::{CompactFormatter, Formatter, PrettyFormatter};
use serializer::Serializer;
use std::io::Write;

mod formatter;
mod serializer;

pub fn to_str<T: Serialize>(value: &T) -> Result<String> {
    to_bytes(value).map(|bytes| unsafe { String::from_utf8_unchecked(bytes) })
}

pub fn to_str_pretty<T: Serialize>(value: &T) -> Result<String> {
    to_bytes_pretty(value).map(|bytes| unsafe { String::from_utf8_unchecked(bytes) })
}

pub fn to_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    let mut output = Vec::new();
    to_write(value, &mut output).map(|_| output)
}

pub fn to_bytes_pretty<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    let mut output = Vec::new();
    to_write_pretty(value, &mut output).map(|_| output)
}

pub fn to_write<T: Serialize, W: Write>(value: &T, output: W) -> Result<()> {
    let mut serializer = Serializer::compact(output);
    value.serialize(&mut serializer)
}

pub fn to_write_pretty<T: Serialize, W: Write>(value: &T, output: W) -> Result<()> {
    let mut serializer = Serializer::pretty(output);
    value.serialize(&mut serializer)
}
