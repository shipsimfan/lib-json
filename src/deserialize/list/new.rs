use crate::deserialize::ListDeserializer;
use lct_streams::SliceByteCharStream;

impl<'a, 'de> ListDeserializer<'a, 'de> {
    /// Creates a new [`ListDeserializer`]
    pub(crate) fn new(stream: &'a mut SliceByteCharStream<'de>) -> Self {
        ListDeserializer {
            stream,
            first: true,
        }
    }
}
