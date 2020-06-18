mod compose;
mod operators;
mod track;
mod waves;

use crate::compose::compose;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    compose(&input);
    Ok(())
}
