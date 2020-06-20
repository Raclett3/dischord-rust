mod compose;
mod filters;
mod operators;
mod track;
mod waves;

use crate::compose::compose;
use std::io::{self, Read};

static SAMPLING: usize = 44100;

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    args.next();
    let sampling = if let Some(arg) = args.next() {
        arg.parse().unwrap_or(SAMPLING)
    } else {
        SAMPLING
    };

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    compose(&input, sampling);
    Ok(())
}
