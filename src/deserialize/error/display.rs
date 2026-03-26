use crate::DeserializeError;

impl<'de> core::fmt::Display for DeserializeError<'de> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} at {}", self.kind, self.position.unwrap())
    }
}
