use super::Stream;
use crate::{Input, ParseError};

const HORIZONTAL_TAB: char = '\x09';
const LINE_FEED: char = '\x0A';
const CARRIAGE_RETURN: char = '\x0D';
const SPACE: char = '\x20';

pub(super) fn parse<I: Input>(stream: &mut Stream<I>) -> Result<(), ParseError<I::Error>> {
    while let Some(c) = stream.peek() {
        if !is_json_whitespace(c) {
            break;
        }

        stream.next()?;
    }

    Ok(())
}

fn is_json_whitespace(c: char) -> bool {
    match c {
        HORIZONTAL_TAB | LINE_FEED | CARRIAGE_RETURN | SPACE => true,
        _ => false,
    }
}
