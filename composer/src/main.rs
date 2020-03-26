mod waves;
mod track;

use crate::track::Track;
use crate::waves::pulse50;
use std::io;

fn main() -> io::Result<()> {
    let mut track = Track::new(44100);
    track.render_wave(0.0, 1.0, 0.5, pulse50, 440.0);
    track.print_as_riff()?;
    Ok(())
}
