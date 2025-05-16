use crate::DeserializeErrorKind;
use lct_streams::InvalidUtf8;

impl<'de> From<InvalidUtf8> for DeserializeErrorKind<'de> {
    fn from(error: InvalidUtf8) -> Self {
        DeserializeErrorKind::InvalidUtf8(error)
    }
}
