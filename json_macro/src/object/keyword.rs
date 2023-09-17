pub(super) enum Keyword {
    Enum,
    Mut,
    Pub,
    Struct,
}

impl Keyword {
    pub(super) fn parse(string: &str) -> Option<Self> {
        Some(match string {
            "enum" => Keyword::Enum,
            "mut" => Keyword::Mut,
            "pub" => Keyword::Pub,
            "struct" => Keyword::Struct,
            _ => return None,
        })
    }

    pub(super) const fn as_str(&self) -> &'static str {
        match self {
            Keyword::Enum => "enum",
            Keyword::Mut => "mut",
            Keyword::Pub => "pub",
            Keyword::Struct => "struct",
        }
    }
}
