use lct_streams::SliceByteCharStream;

mod map_serializer;
mod new;

/// Deserializes a JSON object into a map
pub(super) struct MapDeserializer<'a, 'de> {
    /// The stream to deserialize from
    stream: &'a mut SliceByteCharStream<'de>,

    /// Is the next item the first item in the object?
    first: bool,

    /// Should the next call be to `next_key`?
    next_key: bool,
}
