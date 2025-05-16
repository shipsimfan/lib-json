use crate::deserialize::MapDeserializer;
use lct_streams::SliceByteCharStream;

impl<'a, 'de> MapDeserializer<'a, 'de> {
    /// Creates a new [`MapDeserializer`]
    pub(crate) fn new(stream: &'a mut SliceByteCharStream<'de>) -> Self {
        MapDeserializer {
            stream,
            first: true,
            next_key: true,
        }
    }
}
