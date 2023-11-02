use super::Formatter;

pub(in crate::serialize) struct PrettyFormatter {}

impl PrettyFormatter {
    pub(in crate::serialize) const fn new() -> Self {
        PrettyFormatter {}
    }
}

impl Formatter for PrettyFormatter {}
