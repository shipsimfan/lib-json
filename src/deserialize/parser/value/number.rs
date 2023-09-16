use super::{Stream, MINUS};
use crate::{Input, ParseError, Position, Value};

const ZERO: char = '\x30'; // 0
const DECIMAL_POINT: char = '\x2E'; // .
const PLUS: char = '\x2B'; // +

pub(super) fn parse<I: Input>(
    stream: &mut Stream<I>,
    first_char: char,
    initial_position: Position,
) -> Result<Value<'static>, ParseError<I::Error>> {
    let mut number = String::new();
    let (position, first_digit) = if first_char == MINUS {
        number.push(MINUS);
        (stream.position(), stream.next_err()?)
    } else {
        (initial_position, first_char)
    };

    if first_digit == ZERO {
        return Ok(Value::Number(0.));
    } else if !digit_1_9(first_digit) {
        return Err(ParseError::UnexpectedCharacter(first_digit, position));
    }
    number.push(first_digit);

    digits(stream, &mut number)?;

    if let Some(c) = stream.peek() {
        if c == DECIMAL_POINT {
            number.push(c);
            stream.next()?;
            return parse_frac(stream, number, initial_position);
        } else if e(c) {
            number.push(c);
            stream.next()?;
            return parse_exp(stream, number, initial_position);
        }
    }

    finalize::<I>(number, initial_position)
}

fn parse_frac<I: Input>(
    stream: &mut Stream<I>,
    mut number: String,
    initial_position: Position,
) -> Result<Value<'static>, ParseError<I::Error>> {
    if digits(stream, &mut number)? == 0 {
        let position = stream.position();
        return Err(ParseError::UnexpectedCharacter(
            stream.next_err()?,
            position,
        ));
    }

    if let Some(c) = stream.peek() {
        if e(c) {
            number.push(c);
            stream.next()?;
            return parse_exp(stream, number, initial_position);
        }
    }

    finalize::<I>(number, initial_position)
}

fn parse_exp<I: Input>(
    stream: &mut Stream<I>,
    mut number: String,
    initial_position: Position,
) -> Result<Value<'static>, ParseError<I::Error>> {
    if let Some(c) = stream.peek() {
        if c == MINUS || c == PLUS {
            stream.next()?;
            number.push(c);
        }
    }

    if digits(stream, &mut number)? == 0 {
        let position = stream.position();
        return Err(ParseError::UnexpectedCharacter(
            stream.next_err()?,
            position,
        ));
    }

    finalize::<I>(number, initial_position)
}

fn finalize<I: Input>(
    number: String,
    initial_position: Position,
) -> Result<Value<'static>, ParseError<I::Error>> {
    number
        .parse::<f64>()
        .map(|number| number.into())
        .map_err(|_| ParseError::InvalidNumber(initial_position))
}

fn digits<I: Input>(
    stream: &mut Stream<I>,
    number: &mut String,
) -> Result<usize, ParseError<I::Error>> {
    let mut count = 0;
    while let Some(c) = stream.peek() {
        if !c.is_digit(10) {
            break;
        }

        number.push(c);
        stream.next()?;
        count += 1;
    }

    Ok(count)
}

// 1 - 9
fn digit_1_9(c: char) -> bool {
    c >= '\x31' && c <= '\x39'
}

// e E
fn e(c: char) -> bool {
    c == '\x65' || c == '\x45'
}
