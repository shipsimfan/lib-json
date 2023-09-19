use crate::Keyword;
use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

pub(crate) struct Generator(Vec<TokenTree>);

impl Generator {
    pub fn new() -> Self {
        Generator(Vec::new())
    }

    pub fn push_keyword(&mut self, keyword: Keyword) {
        self.push_ident_str(keyword.as_str());
    }

    pub fn push_ident_str(&mut self, str: &str) {
        self.push_ident(Ident::new(str, Span::call_site()));
    }

    pub fn push_ident(&mut self, ident: Ident) {
        self.0.push(TokenTree::Ident(ident));
    }

    pub fn push_punct(&mut self, char: char, spacing: Spacing) {
        self.0.push(TokenTree::Punct(Punct::new(char, spacing)));
    }

    pub fn push_group(&mut self, tokens: TokenStream, delimiter: Delimiter) {
        self.0.push(TokenTree::Group(Group::new(delimiter, tokens)));
    }

    pub fn push_type_path(&mut self, path: &[&str], prefix: bool) {
        let mut first = !prefix;
        for part in path {
            if first {
                first = false;
            } else {
                self.push_punct(':', Spacing::Joint);
                self.push_punct(':', Spacing::Alone);
            }

            self.push_ident_str(part);
        }
    }

    pub fn to_token_stream(self) -> TokenStream {
        TokenStream::from_iter(self.0.into_iter())
    }
}
