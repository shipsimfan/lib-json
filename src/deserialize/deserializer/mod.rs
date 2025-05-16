use lct_streams::SliceByteCharStream;

mod deserializer;
mod new;

/// A structure which deserializes JSON from a stream of bytes
pub(super) struct Deserializer<'a, 'de> {
    /// The stream to read bytes from
    stream: &'a mut SliceByteCharStream<'de>,
}
