use crate::DeserializeErrorKind;

impl<'de> std::fmt::Display for DeserializeErrorKind<'de> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeserializeErrorKind::Custom(error) => f.write_str(error),

            DeserializeErrorKind::InvalidType {
                unexpected,
                expected,
            } => write!(
                f,
                "unexpected type \"{}\", expected {}",
                unexpected, expected
            ),
            DeserializeErrorKind::InvalidValue {
                unexpected,
                expected,
            } => write!(
                f,
                "unexpected value \"{}\", expected {}",
                unexpected, expected
            ),
            DeserializeErrorKind::InvalidLength {
                unexpected,
                expected,
            } => write!(f, "unexpected length {}, expected {}", unexpected, expected),
            DeserializeErrorKind::UnknownField { field, expected } => {
                write!(f, "unknown field \"{}\", expected ", field)?;
                for i in 0..expected.len() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", expected[i])?;
                }
                Ok(())
            }
            DeserializeErrorKind::MissingField(field) => write!(f, "missing field \"{}\"", field),
            DeserializeErrorKind::DuplicateField(field) => {
                write!(f, "\"{}\" appears more than once", field)
            }
            DeserializeErrorKind::Unexpected {
                unexpected,
                expected,
            } => write!(
                f,
                "unexpected \"{}\", expected \"{}\"",
                unexpected, expected
            ),
            DeserializeErrorKind::UnexpectedEndOfJSON => write!(f, "unexpected end of JSON"),
            DeserializeErrorKind::InvalidUtf8(error) => error.fmt(f),
        }
    }
}
