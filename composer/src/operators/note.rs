use crate::parse::*;
use crate::waves::pulse50;
use crate::compose::State;

pub fn note(state: &mut State) -> bool {
    let current_char = take_char(state);
    let note = if current_char >= 'a' && current_char <= 'g' {
        current_char as u8 - 0x20
    } else if current_char >= 'A' && current_char <= 'G' {
        current_char as u8
    } else {
        return false;
    } as char;
    state.position += 1;

    let mut accidental = 0.0;
    while !is_eof(state) {
        match take_char(state) {
            '+' => {accidental += 1.0},
            '#' => {accidental += 1.0},
            '-' => {accidental += -1.0},
            _ => break,
        }
        state.position += 1;
    }
    let duration = 240.0 / state.context.tempo / unsigned_int(state, state.context.default_length) as f64;
  
    let frequency = calc_frequency(state.context.octave, note, accidental);
    state.context.track.render_wave(state.context.position, duration, state.context.volume, pulse50, frequency);
    state.context.position += duration;
    true
}

fn calc_frequency(octave: i32, note_char: char, accidental_ammount: f64) -> f64 {
    let note_position = match note_char {
                            'C' => 3,
                            'D' => 5,
                            'E' => 7,
                            'F' => 8,
                            'G' => 10,
                            'A' => 12,
                            'B' => 14,
                            _ => 0,
                        } as f64;
    220.0 * 2f64.powf((note_position + accidental_ammount + 12.0 * octave as f64) / 12.0)
}
