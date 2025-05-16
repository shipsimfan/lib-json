use crate::{DeserializeError, DeserializeErrorKind};
use data_format::{Expected, Unexpected};
use std::borrow::Cow;

impl<'de> data_format::DeserializeError<'de> for DeserializeError<'de> {
    fn custom<T: std::fmt::Display>(error: T) -> Self {
        DeserializeErrorKind::Custom(error.to_string()).into()
    }

    fn invalid_type<U: Into<Unexpected>, E: Expected + ?Sized>(
        unexpected: U,
        expected: &E,
    ) -> Self {
        DeserializeErrorKind::InvalidType {
            unexpected: unexpected.into(),
            expected: expected.to_string(),
        }
        .into()
    }

    fn invalid_value<'a, U: Into<Unexpected>, E: Expected + ?Sized>(
        unexpected: U,
        expected: &E,
    ) -> Self {
        DeserializeErrorKind::InvalidValue {
            unexpected: unexpected.into(),
            expected: expected.to_string(),
        }
        .into()
    }

    fn invalid_length<E: Expected + ?Sized>(unexpected: usize, expected: &E) -> Self {
        DeserializeErrorKind::InvalidLength {
            unexpected,
            expected: expected.to_string(),
        }
        .into()
    }

    fn unknown_field<S: Into<Cow<'de, str>>>(field: S, expected: &'static [&'static str]) -> Self {
        DeserializeErrorKind::UnknownField {
            field: field.into(),
            expected,
        }
        .into()
    }

    fn missing_field(field: &'static str) -> Self {
        DeserializeErrorKind::MissingField(field).into()
    }

    fn duplicate_field(field: &'static str) -> Self {
        DeserializeErrorKind::DuplicateField(field).into()
    }
}
