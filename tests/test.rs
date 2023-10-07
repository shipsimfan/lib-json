use json::ToJSON;

#[derive(ToJSON)]
pub(crate) struct Test1<'a> {
    name: std::string::String,
    value: usize,
    maybe: ::std::option::Option<Vec<usize>>,
    reference: &'a str,
    r#type: std::borrow::Cow<'a, str>,
}

#[derive(ToJSON)]
pub enum Test2 {
    Example1,
    Example2 = 2,
}

#[test]
fn test() {}
