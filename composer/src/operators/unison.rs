use crate::compose::State;
use crate::parse::*;

pub fn unison(state: &mut State) -> bool {
    let initial_position = state.position;
    if !expect_char(state, 'u') && !expect_char(state, 'U') {
        return false;
    }

    let unison_count = unsigned_int(state, 1);
    if !expect_char(state, ',') {
        state.position = initial_position;
        return false;
    }
    let detune = 1.0 + (unsigned_int(state, 0) as i64 - 100) as f64 / 2000.0;

    state.context.unison_count = unison_count;
    state.context.unison_detune = detune;
    true
}
