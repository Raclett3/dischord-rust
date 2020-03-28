use crate::track::Track;
use crate::waves::pulse50;

struct Context {
    track: Track,
    position: f64,
    octave: i32,
}

impl Context {
    fn new() -> Self {
        Context {
            track: Track::new(44100),
            position: 0.0,
            octave: 0,
        }
    }
}

struct State<'state> {
    input: &'state str,
    position: usize,
    context: Context,
}

impl<'state> State<'state> {
    fn new(input: &'state str) -> Self {
        State {
            input,
            position: 0,
            context: Context::new(),
        }
    }
}

fn skip_spaces(state: &mut State) {
    loop {
        if state.position >= state.input.len() {
            break;
        }
        let current_char = state.input.as_bytes()[state.position] as char;
        if current_char == ' ' || current_char == '\n' || current_char == '\r' {
            state.position += 1;
        } else {
            break;
        }
    }
}

fn is_eof(state: &mut State) -> bool {
    skip_spaces(state);
    state.position >= state.input.len()
}

fn take_char(state: &mut State) -> char {
    skip_spaces(state);
    if state.position < state.input.len() {
        state.input.as_bytes()[state.position] as char
    } else {
        '\0'
    }
}

fn note(state: &mut State) -> bool {
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

    let frequency = calc_frequency(state.context.octave, note, accidental);
    state.context.track.render_wave(state.context.position, 0.5, 0.5, pulse50, frequency);
    state.context.position += 0.5;
    true
}

fn rest(state: &mut State) -> bool {
    let current_char = take_char(state);
    if current_char != 'r' && current_char != 'R' {
        return false;
    }
    state.position += 1;

    state.context.track.render_wave(state.context.position, 0.5, 0.5, |_, _| 0.0, 0.0);
    state.context.position += 0.5;
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

fn score(state: &mut State) -> Option<char> {
    loop {
        if is_eof(state) {
            break None;
        }

        let result = note(state) || rest(state);

        if !result {
            break Some(take_char(state));
        }
    }
}

pub fn compose(input: &str) {
    let mut state = State::new(input);
    let err = score(&mut state);
    if let Some(unexpected) = err {
        eprintln!("Unexpected token: {}", unexpected);
        return;
    };
    let result = state.context.track.print_as_riff();
    match result {
        Ok(_) => (),
        Err(_) => eprintln!("Unexpected Error"),
    };
}
