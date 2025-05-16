use crate::deserialize::Deserializer;
use lct_streams::SliceByteCharStream;

impl<'a, 'de> Deserializer<'a, 'de> {
    /// Creates a new [`Deserializer`] over `stream`
    pub(crate) fn new(stream: &'a mut SliceByteCharStream<'de>) -> Self {
        Deserializer { stream }
    }
}
