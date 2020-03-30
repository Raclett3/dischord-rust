use crate::operators::{
    default_length::default_length,
    envelope::envelope,
    note::{chord, note},
    octave::octave,
    repeat::repeat,
    rest::rest,
    rewind::rewind,
    tempo::tempo,
    tone::tone,
    unison::unison,
    volume::volume,
};
use crate::parse::*;
use crate::track::{Track, Envelope};
use crate::waves::{pulse50, Wave};

pub struct Context {
    pub track: Track,
    pub position: f64,
    pub octave: i32,
    pub tempo: f64,
    pub default_length: u32,
    pub volume: f64,
    pub tone: Wave,
    pub unison_detune: f64,
    pub unison_count: u32,
    pub envelope: Envelope,
}

impl Context {
    pub fn new() -> Self {
        Context {
            track: Track::new(44100),
            position: 0.0,
            octave: 0,
            tempo: 120.0,
            default_length: 8,
            volume: 0.5,
            tone: pulse50,
            unison_detune: 0.0,
            unison_count: 1,
            envelope: Envelope {
                attack: 0.0,
                decay: 0.0,
                sustain: 1.0,
                release: 0.0,
            },
        }
    }
}

pub struct StackItem {
    pub position: usize,
    pub repeat_count: u32,
}

pub struct State<'state> {
    pub input: &'state str,
    pub position: usize,
    pub context: Context,
    pub repeat_stack: Vec<StackItem>,
}

impl<'state> State<'state> {
    pub fn new(input: &'state str) -> Self {
        State {
            input,
            position: 0,
            context: Context::new(),
            repeat_stack: Vec::new(),
        }
    }
}

fn score(state: &mut State) -> Option<char> {
    loop {
        if is_eof(state) {
            break None;
        }

        let result = note(state)
            || rest(state)
            || tempo(state)
            || default_length(state)
            || octave(state)
            || volume(state)
            || rewind(state)
            || repeat(state)
            || chord(state)
            || tone(state)
            || unison(state)
            || envelope(state);

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
