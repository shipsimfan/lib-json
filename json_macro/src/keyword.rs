pub(crate) enum Keyword {
    Enum,
    Fn,
    For,
    Impl,
    Mut,
    Pub,
    Struct,
}

impl Keyword {
    pub(crate) fn parse(string: &str) -> Option<Self> {
        Some(match string {
            "enum" => Keyword::Enum,
            "fn" => Keyword::Fn,
            "for" => Keyword::For,
            "impl" => Keyword::Impl,
            "mut" => Keyword::Mut,
            "pub" => Keyword::Pub,
            "struct" => Keyword::Struct,
            _ => return None,
        })
    }

    pub(crate) const fn as_str(&self) -> &'static str {
        match self {
            Keyword::Enum => "enum",
            Keyword::Fn => "fn",
            Keyword::For => "for",
            Keyword::Impl => "impl",
            Keyword::Mut => "mut",
            Keyword::Pub => "pub",
            Keyword::Struct => "struct",
        }
    }
}
