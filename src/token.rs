pub enum Token {
    String(String),
    Number(f64),
    BeginArray,
    BeginObject,
    EndArray,
    EndObject,
    NameSeperator,
    ValueSeperator,
    False,
    Null,
    True,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Number(number) => write!(f, "number \"{}\"", number),
            Token::String(string) => write!(f, "string \"{}\"", string),
            Token::BeginArray => write!(f, "'['"),
            Token::BeginObject => write!(f, "'{{'"),
            Token::EndArray => write!(f, "']'"),
            Token::EndObject => write!(f, "'}}'"),
            Token::NameSeperator => write!(f, "':'"),
            Token::ValueSeperator => write!(f, "','"),
            Token::False => write!(f, "\"false\""),
            Token::Null => write!(f, "\"null\""),
            Token::True => write!(f, "\"true\""),
        }
    }
}
