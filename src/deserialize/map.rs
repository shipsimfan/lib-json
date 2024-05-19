use super::{Deserializer, Stream};
use crate::Error;

/// Deserializes a JSON object into a map
pub(super) struct MapDeserializer<'a, 'de> {
    /// The stream to deserialize from
    stream: &'a mut Stream<'de>,

    /// The index into the stream the object start on
    start_index: usize,

    /// Is the next item the first item in the object?
    first: bool,

    /// Should the next call be to `next_key`?
    next_key: bool,
}

impl<'a, 'de> MapDeserializer<'a, 'de> {
    /// Creates a new [`MapDeserializer`]
    pub(super) fn new(stream: &'a mut Stream<'de>, start_index: usize) -> Self {
        MapDeserializer {
            stream,
            start_index,
            first: true,
            next_key: true,
        }
    }
}

impl<'a, 'de> data_format::MapDeserializer<'de> for MapDeserializer<'a, 'de> {
    type Error = Error;

    fn next_key<K: data_format::Deserialize<'de>>(&mut self) -> Result<Option<K>, Self::Error> {
        assert!(self.next_key);

        self.stream.skip_whitespace();

        if self.first {
            match self.stream.peek().ok_or(Error::UnexpectedEndOfJSON)? {
                b'}' => return Ok(None),
                _ => {}
            }

            self.first = false;
        } else {
            match self.stream.peek().ok_or(Error::UnexpectedEndOfJSON)? {
                b',' => {
                    self.stream.next();
                }
                b'}' => return Ok(None),
                _ => {
                    return Err(Error::UnexpectedCharacter {
                        unexpected: self.stream.get_bytes(self.start_index).to_owned(),
                        expected: "',' or '}'",
                    })
                }
            }

            self.stream.skip_whitespace();
        }

        self.next_key = false;

        K::deserialize(Deserializer::new(self.stream)).map(|ret| Some(ret))
    }

    fn next_value<V: data_format::Deserialize<'de>>(&mut self) -> Result<V, Self::Error> {
        assert!(!self.next_key);

        self.stream.skip_whitespace();

        self.stream.expect(b':', self.start_index, "a ':'")?;

        self.stream.skip_whitespace();

        self.next_key = true;

        V::deserialize(Deserializer::new(self.stream))
    }
}
