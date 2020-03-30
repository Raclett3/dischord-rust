use crate::waves::Wave;
use std::io::{self, Write};
use std::vec::Vec;

pub struct Track {
    track: Vec<f64>,
    allocated: usize,
    sampling: u32,
}

pub struct Envelope {
    pub attack: f64,
    pub decay: f64,
    pub sustain: f64,
    pub release: f64,
}

impl Track {
    pub fn new(sampling: u32) -> Self {
        Track {
            track: vec![],
            allocated: 0,
            sampling,
        }
    }

    pub fn print_as_riff(&self) -> io::Result<()> {
        let length = (self.track.len() * 2) as u32;
        print!("RIFF");
        print_u32(length + 36)?;
        print!("WAVEfmt ");
        print_u32(16)?;
        print_u16(1)?;
        print_u16(1)?;
        print_u32(self.sampling)?;
        print_u32(self.sampling * 2)?;
        print_u16(2)?;
        print_u16(16)?;
        print!("data");
        print_u32(length)?;
        for sample in &self.track {
            let value = fit_number(-32767.0, *sample * 32767.0, 32767.0) as i16;
            print_i16(value)?;
        }
        Ok(())
    }

    pub fn render_wave(
        &mut self,
        start: f64,
        duration: f64,
        volume: f64,
        wave: Wave,
        frequency: f64,
        envelope: &Envelope,
    ) {
        let start_index = (start * self.sampling as f64) as usize;
        let length = (duration * self.sampling as f64) as usize;

        let attack_length = (envelope.attack * self.sampling as f64) as usize;
        let decay_length = (envelope.decay * self.sampling as f64) as usize;
        let release_length = (envelope.release * self.sampling as f64) as usize;
        let release_origin = if length <= attack_length {
            length as f64 / attack_length as f64
        } else if length <= attack_length + decay_length {
            1.0 - (1.0 - envelope.sustain) * (length - attack_length) as f64 / decay_length as f64
        } else {
            envelope.sustain
        };

        let end_index = start_index + length + release_length;
        self.fill_zero_until(end_index);

        for i in 0..(length + release_length) {
            let envelope_volume = if i >= length {
                release_origin * (length + release_length - i) as f64 / release_length as f64
            } else if i < attack_length {
                i as f64 / attack_length as f64
            } else if i < attack_length + decay_length {
                1.0 - (1.0 - envelope.sustain) * (i - attack_length) as f64 / decay_length as f64
            } else {
                envelope.sustain
            };
            let position = i as f64 / self.sampling as f64;
            let value = wave(frequency, position) * volume * envelope_volume;
            self.add_value(start_index + i, value);
        }
    }

    fn reserve(&mut self) {
        let additional = if self.allocated != 0 {
            self.allocated
        } else {
            256
        };
        self.track.reserve(additional);
        self.allocated += additional;
    }

    fn fill_zero_until(&mut self, index: usize) {
        let length = self.track.len();

        if index >= length {
            for _ in 0..(index - length + 1) {
                self.track.push(0.0);
            }
        }
    }

    fn add_value(&mut self, index: usize, value: f64) {
        while index >= self.allocated {
            self.reserve();
        }

        self.track[index] += value;
    }
}

fn fit_number(min: f64, value: f64, max: f64) -> f64 {
    if min > value {
        min
    } else if max < value {
        max
    } else {
        value
    }
}

fn print_i16(value: i16) -> io::Result<()> {
    let msb = (value >> 8) as u8;
    let lsb = (value & 0xFF) as u8;
    io::stdout().write_all(&[lsb, msb])?;
    Ok(())
}

fn print_u16(value: u16) -> io::Result<()> {
    let msb = ((value >> 8) & 0xFF) as u8;
    let lsb = (value & 0xFF) as u8;
    io::stdout().write_all(&[lsb, msb])?;
    Ok(())
}

fn print_u32(value: u32) -> io::Result<()> {
    for i in 0..4 {
        let byte = ((value >> (8 * i)) & 0xFF) as u8;
        io::stdout().write_all(&[byte])?;
    }
    Ok(())
}
