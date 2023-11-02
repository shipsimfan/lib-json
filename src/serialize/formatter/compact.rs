use super::Formatter;

pub(in crate::serialize) struct CompactFormatter {}

impl CompactFormatter {
    pub(in crate::serialize) const fn new() -> Self {
        CompactFormatter {}
    }
}

impl Formatter for CompactFormatter {}
