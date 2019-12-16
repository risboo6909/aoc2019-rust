use failure::{Error, format_err};

use super::Computer;
use utils::{split_by_comma, ParseResult};

pub(crate) fn consume_until_break(c: &mut Computer) -> Result<Vec<isize>, Error> {
    let mut result = Vec::new();

    c.step()?;

    while !c.is_finished() {
        result.push(c.get_output()?);
        c.step()?;
    }

    Ok(result)
}

pub(crate) fn parse_intcode(input_raw: &str) -> ParseResult<Vec<isize>> {
    split_by_comma(input_raw, &|e: &str| e.parse::<isize>()
        .or_else(|err| Err(format_err!("Failed to parse input: {}", err))))
}
