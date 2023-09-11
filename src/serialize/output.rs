use std::io::Write;

pub trait Output {
    type Error;

    fn write(&mut self, buffer: &[u8]) -> Result<(), Self::Error>;

    fn write_fmt(&mut self, fmt: std::fmt::Arguments<'_>) -> Result<(), Self::Error>;
}

impl<T: Write> Output for T {
    type Error = std::io::Error;

    fn write(&mut self, buffer: &[u8]) -> Result<(), Self::Error> {
        self.write_all(buffer)
    }

    fn write_fmt(&mut self, fmt: std::fmt::Arguments<'_>) -> Result<(), Self::Error> {
        self.write_fmt(fmt)
    }
}
