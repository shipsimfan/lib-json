use super::{ident_eq, Keyword};
use proc_macro::{
    token_stream::IntoIter, Delimiter, Group, Ident, Punct, Spacing, TokenStream, TokenTree,
};
use std::iter::Peekable;

pub(super) struct Stream {
    iter: Peekable<IntoIter>,
}

impl Stream {
    pub(super) fn new(stream: TokenStream) -> Self {
        Stream {
            iter: stream.into_iter().peekable(),
        }
    }

    pub(super) fn peek(&mut self) -> Option<&TokenTree> {
        self.iter.peek()
    }

    pub(super) fn next_ident(&mut self) -> Result<Ident, Option<TokenTree>> {
        match match self.iter.next() {
            Some(token) => token,
            None => return Err(None),
        } {
            TokenTree::Ident(ident) => Ok(ident),
            token => Err(Some(token)),
        }
    }

    pub(super) fn next_keyword(&mut self, expected: &str) -> Result<Keyword, Option<TokenTree>> {
        let ident = self.next_ident()?;
        match Keyword::parse(&ident.to_string()) {
            Some(keyword) => Ok(keyword),
            None => panic!("expected {}, instead found '{}'", expected, ident),
        }
    }

    pub(super) fn next_group(&mut self, delimiter: Delimiter) -> Result<Stream, Option<TokenTree>> {
        match match self.iter.next() {
            Some(token) => token,
            None => return Err(None),
        } {
            TokenTree::Group(group) => {
                if group.delimiter() == delimiter {
                    Ok(Stream::new(group.stream()))
                } else {
                    Err(Some(TokenTree::Group(group)))
                }
            }
            token => Err(Some(token)),
        }
    }

    pub(super) fn next_punct(
        &mut self,
        char: char,
        spacing: Option<Spacing>,
    ) -> Result<Punct, Option<TokenTree>> {
        match match self.iter.next() {
            Some(token) => token,
            None => return Err(None),
        } {
            TokenTree::Punct(punct) => {
                if punct.as_char() == char && punct.spacing() == spacing.unwrap_or(punct.spacing())
                {
                    Ok(punct)
                } else {
                    Err(Some(TokenTree::Punct(punct)))
                }
            }
            token => Err(Some(token)),
        }
    }

    pub(super) fn take_keyword(&mut self, keyword: Keyword) -> Option<Ident> {
        if !ident_eq!(self.iter.peek()?, keyword.as_str()) {
            return None;
        }

        Some(match self.iter.next().unwrap() {
            TokenTree::Ident(ident) => ident,
            _ => unreachable!(),
        })
    }

    pub(super) fn take_group(&mut self, delimiter: Delimiter) -> Option<Group> {
        if !match self.iter.peek()? {
            TokenTree::Group(group) => group.delimiter() == delimiter,
            _ => false,
        } {
            return None;
        }

        Some(match self.iter.next().unwrap() {
            TokenTree::Group(group) => group,
            _ => unreachable!(),
        })
    }

    pub(super) fn take_punct(&mut self, char: char, spacing: Option<Spacing>) -> Option<Punct> {
        if !match self.iter.peek()? {
            TokenTree::Punct(punct) => {
                punct.as_char() == char && punct.spacing() == spacing.unwrap_or(punct.spacing())
            }
            _ => false,
        } {
            return None;
        }

        Some(match self.iter.next().unwrap() {
            TokenTree::Punct(punct) => punct,
            _ => unreachable!(),
        })
    }
}
