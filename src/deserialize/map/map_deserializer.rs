use crate::{
    deserialize::{expect, peek, skip_whitespace, Deserializer, MapDeserializer},
    DeserializeError,
};

impl<'a, 'de> data_format::MapDeserializer<'de> for MapDeserializer<'a, 'de> {
    type Error = DeserializeError<'de>;

    fn next_key<K: data_format::Deserialize<'de>>(&mut self) -> Result<Option<K>, Self::Error> {
        assert!(self.next_key);

        skip_whitespace(self.stream)?;

        if self.first {
            match peek(self.stream)? {
                ('}', _) => return Ok(None),
                _ => {}
            }

            self.first = false;
        } else {
            match peek(self.stream)? {
                (',', _) => {
                    self.stream.next().unwrap();
                }
                ('}', _) => return Ok(None),
                (c, pos) => return Err(DeserializeError::unexpected(c, "',' or '}'", pos)),
            }

            skip_whitespace(self.stream)?;
        }

        self.next_key = false;

        K::deserialize(Deserializer::new(self.stream)).map(|ret| Some(ret))
    }

    fn next_value<V: data_format::Deserialize<'de>>(&mut self) -> Result<V, Self::Error> {
        assert!(!self.next_key);

        skip_whitespace(self.stream)?;

        expect(self.stream, ':', "a ':'")?;

        skip_whitespace(self.stream)?;

        self.next_key = true;

        V::deserialize(Deserializer::new(self.stream))
    }
}
