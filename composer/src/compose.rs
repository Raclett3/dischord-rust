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
    pub fn new(sampling: u32) -> Self {
        Context {
            track: Track::new(sampling),
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

pub struct State<'a> {
    pub input: &'a str,
    pub position: usize,
    pub transaction_begin: Option<usize>,
    pub context: Context,
    pub repeat_stack: Vec<StackItem>,
}

impl<'a> State<'a> {
    pub fn new(input: &'a str, sampling: u32) -> Self {
        State {
            input,
            position: 0,
            transaction_begin: None,
            context: Context::new(sampling),
            repeat_stack: Vec::new(),
        }
    }

    pub fn transaction(&mut self) {
        self.transaction_begin = Some(self.position);
    }

    pub fn rollback(&mut self) {
        self.position = self.transaction_begin.unwrap_or(self.position);
    }

    pub fn skip_spaces(&mut self) {
        loop {
            if self.position >= self.input.len() {
                break;
            }
            let current_char = self.input.as_bytes()[self.position] as char;
            if current_char == ' ' || current_char == '\n' || current_char == '\r' {
                self.position += 1;
            } else {
                break;
            }
        }
    }
    
    pub fn is_eof(&mut self) -> bool {
        self.skip_spaces();
        self.position >= self.input.len()
    }
    
    pub fn take_char(&mut self) -> char {
        self.skip_spaces();
        if self.position < self.input.len() {
            self.input.as_bytes()[self.position] as char
        } else {
            '\0'
        }
    }
    
    pub fn expect_char(&mut self, expected: char) -> bool {
        let actual = self.take_char();
        if actual != expected {
            return false;
        }
        self.position += 1;
        true
    }
    
    pub fn unsigned_int(&mut self, default: u32) -> u32 {
        let mut result: u32 = 0;
        while !self.is_eof() {
            let current_char = self.take_char();
            if current_char >= '0' && current_char <= '9' {
                result = result * 10 + (current_char as u8 - b'0') as u32;
                self.position += 1;
            } else {
                break;
            }
        }
        if result > 0 {
            result
        } else {
            default
        }
    }

    pub fn take_note_duration(&mut self) -> f64 {
        fn calc_duration(tempo: f64, nth_semibreve: u32) -> f64 {
            240.0 / tempo / nth_semibreve as f64
        };
    
        let mut sum = 0.0;
        loop {
            let next = self.unsigned_int(self.context.default_length);
    
            let mut dotted = 1.0;
            loop {
                if !self.expect_char('.') {
                    break;
                }
                dotted = (dotted + 2.0) / 2.0;
            };
    
            sum += calc_duration(self.context.tempo, next) * dotted;
    
            if !self.expect_char('&') {
                break sum;
            }
        }
    }
}

fn score(state: &mut State) -> Option<char> {
    loop {
        if state.is_eof() {
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
            break Some(state.take_char());
        }
    }
}

pub fn compose(input: &str, sampling: u32) {
    let mut state = State::new(input, sampling);
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
