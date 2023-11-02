use std::{borrow::Cow, ops::Deref};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct String<'a>(pub Cow<'a, str>);

impl<'a> String<'a> {
    pub fn borrow<'b>(&'b self) -> String<'b> {
        String(
            match &self.0 {
                Cow::Owned(string) => string.as_str(),
                Cow::Borrowed(string) => *string,
            }
            .into(),
        )
    }

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

impl<'a> From<char> for String<'a> {
    fn from(string: char) -> Self {
        String(string.to_string().into())
    }
}

impl<'a> From<std::string::String> for String<'a> {
    fn from(string: std::string::String) -> Self {
        String(string.into())
    }
}

impl<'a> From<&'a str> for String<'a> {
    fn from(string: &'a str) -> Self {
        String(string.into())
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
        '"'.fmt(f)?;
        for c in self.chars() {
            if c == '\\' || c == '"' {
                '\\'.fmt(f)?;
            }
            c.fmt(f)?;
        }
        '"'.fmt(f)
    }
}

impl<'a> std::fmt::Debug for String<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
