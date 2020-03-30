use crate::compose::State;
use crate::parse::*;

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
            '+' => accidental += 1.0,
            '#' => accidental += 1.0,
            '-' => accidental += -1.0,
            _ => break,
        }
        state.position += 1;
    }
    let duration =
        240.0 / state.context.tempo / unsigned_int(state, state.context.default_length) as f64;

    let frequency = calc_frequency(state.context.octave, note, accidental);
    let unison_count = state.context.unison_count;
    let detune = state.context.unison_detune;
    for i in 0..unison_count {
        let multiplier = if unison_count == 1 {
            1.0
        } else {
            detune.powf(-1.0 + (i as f64 / (unison_count - 1) as f64) * 2.0)
        };
        render_note(state, frequency * multiplier, duration, 1.0 / unison_count as f64);
    }
    state.context.position += duration;
    true
}

pub fn chord(state: &mut State) -> bool {
    let current_char = take_char(state);
    if current_char != '(' {
        return false;
    }
    let start_position = state.position;
    state.position += 1;

    let mut octave = 0;
    let mut frequency_list: Vec<f64> = Vec::new();

    while !is_eof(state) {
        let current_char = take_char(state);
        let note = if current_char >= 'a' && current_char <= 'g' {
            current_char as u8 - 0x20
        } else if current_char >= 'A' && current_char <= 'G' {
            current_char as u8
        } else if current_char == '<' {
            state.position += 1;
            octave += 1;
            continue;
        } else if current_char == '>' {
            state.position += 1;
            octave -= 1;
            continue;
        } else if current_char == ')' {
            state.position += 1;
            break;
        } else {
            state.position = start_position;
            return false;
        } as char;

        let mut accidental = 0f64;
        while !is_eof(state) {
            match take_char(state) {
                '+' => accidental += 1.0,
                '#' => accidental += 1.0,
                '-' => accidental += -1.0,
                _ => break,
            }
            state.position += 1;
        }

        let frequency = calc_frequency(state.context.octave + octave, note, accidental);
        frequency_list.push(frequency);
        state.position += 1;
    }

    let duration =
        240.0 / state.context.tempo / unsigned_int(state, state.context.default_length) as f64;
    for frequency in frequency_list {
        let unison_count = state.context.unison_count;
        let detune = state.context.unison_detune;
        for i in 0..unison_count {
            let multiplier = if unison_count == 1 {
                1.0
            } else {
                detune.powf(-1.0 + (i as f64 / (unison_count - 1) as f64) * 2.0)
            };
            render_note(state, frequency * multiplier, duration, 1.0 / unison_count as f64);
        }
    }
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

fn render_note(state: &mut State, frequency: f64, duration: f64, volume: f64) {
    state.context.track.render_wave(
        state.context.position,
        duration,
        state.context.volume * volume,
        state.context.tone,
        frequency,
        &state.context.envelope,
    );
}
