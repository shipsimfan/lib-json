use std::{borrow::Cow, ops::Deref};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct String<'a>(pub Cow<'a, str>);

impl<'a> String<'a> {
    pub fn to_static(self) -> String<'static> {
        String(Cow::Owned(match self.0 {
            Cow::Owned(string) => string,
            Cow::Borrowed(string) => string.to_owned(),
        }))
    }
}

impl<'a> Deref for String<'a> {
    type Target = Cow<'a, str>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T: Into<Cow<'a, str>>> From<T> for String<'a> {
    fn from(value: T) -> Self {
        String(value.into())
    }
}

impl<'a> Into<std::string::String> for String<'a> {
    fn into(self) -> std::string::String {
        match self.0 {
            Cow::Owned(string) => string,
            Cow::Borrowed(string) => string.to_owned(),
        }
    }
}

impl<'a, 'b> IntoIterator for &'b String<'a> {
    type Item = char;
    type IntoIter = std::str::Chars<'b>;

    fn into_iter(self) -> Self::IntoIter {
        self.chars()
    }
}

impl<'a> std::fmt::Display for String<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}

impl<'a> std::fmt::Debug for String<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
