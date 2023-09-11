pub trait Output {
    type Error;

    fn write(&mut self, buffer: &[u8]) -> Result<(), Self::Error>;

    fn write_fmt(&mut self, fmt: std::fmt::Arguments<'_>) -> Result<(), Self::Error>;
}

pub struct FormatterOutput<'a, 'b>(&'a mut std::fmt::Formatter<'b>);

impl<T: std::io::Write> Output for T {
    type Error = std::io::Error;

    fn write(&mut self, buffer: &[u8]) -> Result<(), Self::Error> {
        self.write_all(buffer)
    }

    fn write_fmt(&mut self, fmt: std::fmt::Arguments<'_>) -> Result<(), Self::Error> {
        std::io::Write::write_fmt(self, fmt)
    }
}

impl<'a, 'b> Output for FormatterOutput<'a, 'b> {
    type Error = std::fmt::Error;

    fn write(&mut self, buffer: &[u8]) -> Result<(), Self::Error> {
        for &byte in buffer {
            std::fmt::Write::write_char(self.0, byte as char)?;
        }
        Ok(())
    }

    fn write_fmt(&mut self, fmt: std::fmt::Arguments<'_>) -> Result<(), Self::Error> {
        std::fmt::Write::write_fmt(self.0, fmt)
    }
}

impl<'a, 'b> From<&'a mut std::fmt::Formatter<'b>> for FormatterOutput<'a, 'b> {
    fn from(value: &'a mut std::fmt::Formatter<'b>) -> Self {
        FormatterOutput(value)
    }
}
