use super::{Deserializer, Stream};
use crate::DeserializeError;
use data_format::Deserialize;

/// Deserializes a JSON array into a list
pub(super) struct ListDeserializer<'a, 'de> {
    /// The stream to serialize from
    stream: &'a mut Stream<'de>,

    /// The index into the stream the list started on
    start_index: usize,

    /// Is the next element the first in the list?
    first: bool,
}

impl<'a, 'de> ListDeserializer<'a, 'de> {
    /// Creates a new [`ListDeserializer`]
    pub(super) fn new(stream: &'a mut Stream<'de>, start_index: usize) -> Self {
        ListDeserializer {
            stream,
            start_index,
            first: true,
        }
    }
}

impl<'a, 'de> data_format::ListDeserializer<'de> for ListDeserializer<'a, 'de> {
    type Error = DeserializeError<'de>;

    fn next_item<T: Deserialize<'de>>(&mut self) -> Result<Option<T>, Self::Error> {
        self.stream.skip_whitespace();
        if self.first {
            match self.stream.peek().ok_or(Self::Error::UnexpectedEndOfJSON)? {
                b']' => return Ok(None),
                _ => {}
            }

            self.first = false;
        } else {
            match self.stream.peek().ok_or(Self::Error::UnexpectedEndOfJSON)? {
                b',' => {
                    self.stream.next();
                }
                b']' => return Ok(None),
                _ => {
                    return Err(Self::Error::Unexpected {
                        unexpected: self.stream.get_bytes(self.start_index),
                        expected: "',' or ']'",
                    })
                }
            }
        }

        T::deserialize(Deserializer::new(self.stream)).map(|ret| Some(ret))
    }
}
