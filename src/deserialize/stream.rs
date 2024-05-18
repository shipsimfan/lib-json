/// A stream of bytes to parse
pub(super) struct Stream<'a> {
    /// The bytes themselves
    bytes: &'a [u8],

    /// The offset of the next byte to read in the stream
    index: usize,
}

impl<'a> Stream<'a> {
    /// Creates a new [`Stream`] over `bytes`
    pub(super) fn new(bytes: &'a [u8]) -> Self {
        Stream { bytes, index: 0 }
    }
}
