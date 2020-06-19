use crate::compose::State;

pub fn note(state: &mut State) -> bool {
    let current_char = state.take_char();
    let note = if current_char >= 'a' && current_char <= 'g' {
        current_char as u8 - 0x20
    } else if current_char >= 'A' && current_char <= 'G' {
        current_char as u8
    } else {
        return false;
    } as char;
    state.position += 1;

    let mut accidental = 0.0;
    while !state.is_eof() {
        match state.take_char() {
            '+' => accidental += 1.0,
            '#' => accidental += 1.0,
            '-' => accidental += -1.0,
            _ => break,
        }
        state.position += 1;
    }

    let duration = take_note_duration(state);
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
    state.transaction();
    if !state.expect_char('(') {
        return false;
    }

    let mut octave = 0;
    let mut frequency_list: Vec<f64> = Vec::new();

    while !state.is_eof() {
        let current_char = state.take_char();
        let note = if current_char >= 'a' && current_char <= 'g' {
            current_char as u8 - 0x20
        } else if current_char >= 'A' && current_char <= 'G' {
            current_char as u8
        } else if state.expect_char('<') {
            octave += 1;
            continue;
        } else if state.expect_char('>') {
            octave -= 1;
            continue;
        } else if state.expect_char(')') {
            break;
        } else {
            state.rollback();
            return false;
        } as char;

        let mut accidental = 0f64;
        while !state.is_eof() {
            match state.take_char() {
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

    let duration = take_note_duration(state);
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

fn take_note_duration(state: &mut State) -> f64 {
    fn calc_duration(tempo: f64, nth_semibreve: u32) -> f64 {
        240.0 / tempo / nth_semibreve as f64
    };

    let mut sum = 0.0;
    loop {
        let next = state.unsigned_int(state.context.default_length);

        let mut dotted = 1.0;
        loop {
            if !state.expect_char('.') {
                break;
            }
            dotted = (dotted + 2.0) / 2.0;
        };

        sum += calc_duration(state.context.tempo, next) * dotted;

        if !state.expect_char('&') {
            break sum;
        }
    }
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
