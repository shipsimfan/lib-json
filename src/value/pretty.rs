use crate::{FormatterOutput, Output};

pub trait PrettyPrintable: Sized {
    fn pretty_print<O: Output>(
        &self,
        output: &mut O,
        depth: usize,
        indent_size: usize,
    ) -> Result<(), O::Error>;

    fn pretty(&self) -> PrettyPrinter<Self> {
        PrettyPrinter::new(self)
    }
}

const DEFAULT_INDENT_SIZE: usize = 4;

pub(crate) fn display_indent<O: Output>(
    output: &mut O,
    depth: usize,
    indent_size: usize,
) -> Result<(), O::Error> {
    for _ in 0..depth * indent_size {
        write!(output, " ")?;
    }
    Ok(())
}

pub struct PrettyPrinter<'a, T: PrettyPrintable> {
    value: &'a T,
    indent_size: usize,
}

impl<'a, T: PrettyPrintable> PrettyPrinter<'a, T> {
    pub fn new(value: &'a T) -> Self {
        PrettyPrinter {
            value,
            indent_size: DEFAULT_INDENT_SIZE,
        }
    }

    pub fn with_ident(value: &'a T, indent_size: usize) -> Self {
        PrettyPrinter { value, indent_size }
    }

    pub fn indent_size(&self) -> usize {
        self.indent_size
    }

    pub fn set_indent_size(&mut self, indent_size: usize) {
        self.indent_size = indent_size;
    }
}

impl<'a, T: PrettyPrintable> std::fmt::Display for PrettyPrinter<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value
            .pretty_print(&mut Into::<FormatterOutput>::into(f), 0, self.indent_size)
            .map_err(|_| std::fmt::Error)
    }
}
