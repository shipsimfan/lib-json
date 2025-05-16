use crate::{
    deserialize::{peek, skip_whitespace, Deserializer, ListDeserializer},
    DeserializeError,
};
use data_format::Deserialize;

impl<'a, 'de> data_format::ListDeserializer<'de> for ListDeserializer<'a, 'de> {
    type Error = DeserializeError<'de>;

    fn next_item<T: Deserialize<'de>>(&mut self) -> Result<Option<T>, Self::Error> {
        skip_whitespace(self.stream)?;
        if self.first {
            match peek(self.stream)? {
                (']', _) => return Ok(None),
                _ => {}
            }

            self.first = false;
        } else {
            match peek(self.stream)? {
                (',', _) => {
                    self.stream.next().unwrap();
                }
                (']', _) => return Ok(None),
                (c, pos) => return Err(DeserializeError::unexpected(c, "',' or ']'", pos)),
            }
        }

        let position = self.stream.position();
        T::deserialize(Deserializer::new(self.stream))
            .map(|ret| Some(ret))
            .map_err(|mut error| {
                error.set_position(position);
                error
            })
    }
}
