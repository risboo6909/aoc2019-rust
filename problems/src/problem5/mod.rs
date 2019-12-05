use failure::Error;

use utils::{result, ProblemResult, Ret};


pub(crate) fn solve() -> Result<Ret<u32>, Error> {
    Ok(result(Ok(1),Ok(2)))
}
