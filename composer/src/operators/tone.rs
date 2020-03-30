use crate::compose::State;
use crate::parse::*;
use crate::waves::*;

static TONES: [Wave; 8] = [pulse50, pulse25, pulse125, triangle, saw, sine, noise, better_pulse];

pub fn tone(state: &mut State) -> bool {
    let current_char = take_char(state);
    if current_char != '@' {
        return false;
    }
    state.position += 1;

    let index = unsigned_int(state, 0) as usize;
    if index >= TONES.len() {
        return false;
    }

    state.context.tone = TONES[index];
    true
}
