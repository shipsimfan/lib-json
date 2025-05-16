use crate::DeserializeError;

impl<'de> std::fmt::Display for DeserializeError<'de> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}", self.kind, self.position.unwrap())
    }
}
