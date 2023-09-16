use crate::{Input, ParseError, Position};

pub(super) struct Stream<'a, I: Input> {
    iter: &'a mut I,
    peek: Option<char>,
    position: Position,
}

macro_rules! unexpected_end_of_stream {
    ($expr: expr, $position: expr) => {
        $expr.ok_or(ParseError::UnexpectedEndOfStream($position))
    };
}

fn next_code_point<I: Input>(
    iter: &mut I,
    position: Position,
) -> Result<Option<char>, ParseError<I::Error>> {
    let x = match iter.next()? {
        Some(c) => c,
        None => return Ok(None),
    };

    if x < 128 {
        return Ok(Some(x as char));
    } else if x < 0xC0 {
        return Err(ParseError::InvalidUTF8(position));
    }

    let y = unexpected_end_of_stream!(iter.next()?, position)? as u32;
    let mut c = ((x as u32) << 8) | y;

    if x >= 0xE0 {
        let z = unexpected_end_of_stream!(iter.next()?, position)? as u32;
        c <<= 8;
        c |= z;

        if x >= 0xF0 {
            let w = unexpected_end_of_stream!(iter.next()?, position)? as u32;
            c <<= 8;
            c |= w;
        }
    }

    char::from_u32(c)
        .map(|c| Some(c))
        .ok_or(ParseError::InvalidUTF8(position))
}

impl<'a, I: Input> Stream<'a, I> {
    pub(super) fn new(iter: &'a mut I) -> Result<Self, ParseError<I::Error>> {
        let position = Position::new();
        let peek = next_code_point(iter, position)?;

        Ok(Stream {
            iter,
            peek,
            position,
        })
    }

    pub(super) fn peek(&self) -> Option<char> {
        self.peek
    }

    pub(super) fn peek_err(&self) -> Result<char, ParseError<I::Error>> {
        unexpected_end_of_stream!(self.peek, self.position)
    }

    pub(super) fn position(&self) -> Position {
        self.position
    }

    pub(super) fn next(&mut self) -> Result<Option<char>, ParseError<I::Error>> {
        let ret = self.peek;
        self.peek = next_code_point(self.iter, self.position)?;

        if let Some(c) = ret {
            self.position.increment(c);
        }

        Ok(ret)
    }

    pub(super) fn next_err(&mut self) -> Result<char, ParseError<I::Error>> {
        unexpected_end_of_stream!(self.next()?, self.position)
    }
}
