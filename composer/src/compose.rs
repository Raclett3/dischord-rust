use crate::operators::{
    note::note,
    rest::rest,
    tempo::tempo,
    default_length::default_length,
    octave::octave,
};
use crate::track::Track;
use crate::parse::*;

pub struct Context {
    pub track: Track,
    pub position: f64,
    pub octave: i32,
    pub tempo: f64,
    pub default_length: u32,
}

impl Context {
    pub fn new() -> Self {
        Context {
            track: Track::new(44100),
            position: 0.0,
            octave: 0,
            tempo: 120.0,
            default_length: 8
        }
    }
}

pub struct State<'state> {
    pub input: &'state str,
    pub position: usize,
    pub context: Context,
}

impl<'state> State<'state> {
    pub fn new(input: &'state str) -> Self {
        State {
            input,
            position: 0,
            context: Context::new(),
        }
    }
}

fn score(state: &mut State) -> Option<char> {
    loop {
        if is_eof(state) {
            break None;
        }

        let result = note(state) || rest(state) || tempo(state) || default_length(state) || octave(state);

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
