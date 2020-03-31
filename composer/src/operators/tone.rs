use crate::compose::State;
use crate::parse::*;
use crate::waves::*;

static TONES: [Wave; 8] = [pulse50, pulse25, pulse125, triangle, saw, sine, noise, better_pulse];

pub fn tone(state: &mut State) -> bool {
    if !expect_char(state, '@') {
        return false;
    }

    let index = unsigned_int(state, 0) as usize;
    if index >= TONES.len() {
        return false;
    }

    state.context.tone = TONES[index];
    true
}
