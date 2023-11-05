use std::io::Write;

/// An escape code sequence in a string
#[repr(u32)]
#[derive(Clone, Copy)]
pub(super) enum Escape {
    /// `"`
    QuotationMark,

    /// `\`
    ReverseSolidus,

    /// `/`
    Solidus,

    /// `\b`
    Backspace,

    /// `\f`
    FormFeed,

    /// `\n`
    LineFeed,

    /// `\r`
    CarriageReturn,

    /// `\t`
    Tab,

    /// Any unicode character
    Unicode(u32),
}

const QUOTATION_MARK: u32 = 0x0022;
const REVERSE_SOLIDUS: u32 = 0x005C;
const SOLIDUS: u32 = 0x002F;
const BACKSPACE: u32 = 0x0008;
const FORM_FEED: u32 = 0x000C;
const LINE_FEED: u32 = 0x000A;
const CARRIAGE_RETURN: u32 = 0x000D;
const TAB: u32 = 0x0009;

impl Escape {
    pub(super) fn from_char(c: char) -> Option<Escape> {
        Some(match c as u32 {
            QUOTATION_MARK => Escape::QuotationMark,
            REVERSE_SOLIDUS => Escape::ReverseSolidus,
            SOLIDUS => Escape::Solidus,
            BACKSPACE => Escape::Backspace,
            FORM_FEED => Escape::FormFeed,
            LINE_FEED => Escape::LineFeed,
            CARRIAGE_RETURN => Escape::CarriageReturn,
            TAB => Escape::Tab,
            c if c < 0x20 => Escape::Unicode(c),
            _ => return None,
        })
    }

    pub(super) fn write<W: Write + ?Sized>(&self, output: &mut W) -> std::io::Result<()> {
        output.write_all(match self {
            Escape::QuotationMark => b"\\\"",
            Escape::ReverseSolidus => b"\\\\",
            Escape::Solidus => b"\\/",
            Escape::Backspace => b"\\b",
            Escape::FormFeed => b"\\f",
            Escape::LineFeed => b"\\n",
            Escape::CarriageReturn => b"\\r",
            Escape::Tab => b"\\t",
            Escape::Unicode(c) => return write!(output, "\\u{:04X}", c),
        })
    }
}
