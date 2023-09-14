pub trait Input {
    type Error;

    fn next(&mut self) -> Result<Option<u8>, Self::Error>;
}

impl<T: std::io::Read> Input for std::io::Bytes<T> {
    type Error = std::io::Error;

    fn next(&mut self) -> Result<Option<u8>, Self::Error> {
        match Iterator::next(self) {
            Some(byte) => byte.map(|byte| Some(byte)),
            None => Ok(None),
        }
    }
}

impl<'a> Input for std::str::Bytes<'a> {
    type Error = ();

    fn next(&mut self) -> Result<Option<u8>, Self::Error> {
        Ok(Iterator::next(self))
    }
}
