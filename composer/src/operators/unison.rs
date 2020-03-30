use crate::compose::State;
use crate::parse::*;

pub fn unison(state: &mut State) -> bool {
    let initial_position = state.position;
    let current_char = take_char(state);
    if current_char != 'u' && current_char != 'U' {
        return false;
    }
    state.position += 1;

    let unison_count = unsigned_int(state, 1);
    let current_char = take_char(state);
    if current_char != ',' {
        state.position = initial_position;
        return false;
    }
    state.position += 1;
    let detune = 1.0 + (unsigned_int(state, 0) as i64 - 100) as f64 / 2000.0;

    state.context.unison_count = unison_count;
    state.context.unison_detune = detune;
    true
}
