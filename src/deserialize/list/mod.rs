use lct_streams::SliceByteCharStream;

mod list_serializer;
mod new;

/// Deserializes a JSON array into a list
pub(super) struct ListDeserializer<'a, 'de> {
    /// The stream to serialize from
    stream: &'a mut SliceByteCharStream<'de>,

    /// Is the next element the first in the list?
    first: bool,
}
