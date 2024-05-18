use crate::{Error, Result};

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

    /// Gets the current index of the stream
    pub(super) fn index(&self) -> usize {
        self.index
    }

    /// Gets the bytes from `start_index` to the current stream position
    pub(super) fn get_bytes(&self, start_index: usize) -> &'a [u8] {
        assert!(start_index < self.index);

        &self.bytes[start_index..self.index]
    }

    /// Gets the next byte without advancing the stream
    pub(super) fn peek(&mut self) -> Option<u8> {
        self.bytes.get(self.index).map(|c| *c)
    }

    /// Gets the next byte and advances the stream
    pub(super) fn next(&mut self) -> Option<u8> {
        self.peek().map(|c| {
            self.index += 1;
            c
        })
    }

    /// Checks the next character in the stream is `c`
    ///
    /// `start_index` is the index of the first character included in the unexpected error, if one
    /// occurs. All characters in the stream upto and including the error causing character are
    /// included in the unexpected error to give extra context.
    pub(super) fn expect(
        &mut self,
        c: u8,
        start_index: usize,
        expected: &'static str,
    ) -> Result<()> {
        let next = self.next().ok_or(Error::UnexpectedEndOfJSON)?;
        if next != c {
            return Err(Error::UnexpectedCharacter {
                unexpected: Vec::from(&self.bytes[start_index..self.index]),
                expected,
            });
        }

        Ok(())
    }

    pub(super) fn expect_str(&mut self, s: &'static str) -> Result<()> {
        let start_index = self.index;
        for c in s.bytes() {
            self.expect(c, start_index, s)?;
        }

        Ok(())
    }
}
