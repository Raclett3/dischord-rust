use crate::compose::State;
use crate::waves::*;

static TONES: [Wave; 8] = [pulse50, pulse25, pulse125, triangle, saw, sine, noise, better_pulse];

pub fn tone(state: &mut State) -> bool {
    if !state.expect_char('@') {
        return false;
    }

    let index = state.unsigned_int(0) as usize;
    if index >= TONES.len() {
        return false;
    }

    state.context.tone = TONES[index];
    true
}
