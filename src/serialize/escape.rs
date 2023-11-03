#[derive(Clone, Copy)]
pub(super) enum Escape {
    DoubleQuote,
    BackSlash,
    ForwardSlash,
    Backspace,
    FormFeed,
    Newline,
    CarriageReturn,
    HorizontalTab,
    Unicode(u32),
}
