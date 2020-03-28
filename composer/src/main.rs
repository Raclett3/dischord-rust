mod waves;
mod track;
mod compose;
mod operators;
mod parse;

use crate::compose::compose;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    compose(&input);
    Ok(())
}
