mod compact;
mod pretty;

pub(super) use compact::CompactFormatter;
pub(super) use pretty::PrettyFormatter;

pub(super) trait Formatter {}
