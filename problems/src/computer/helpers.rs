use failure::Error;

use super::Computer;

pub(crate) fn consume_until_break(c: &mut Computer) -> Result<Vec<isize>, Error> {
    let mut result = Vec::new();

    c.interpret()?;

    while !c.is_finished() {
        result.push(c.get_output()?);
        c.interpret()?;
    }

    Ok(result)
}
