use crate::compose::State;
use crate::parse::*;

pub fn rest(state: &mut State) -> bool {
    let current_char = take_char(state);
    if current_char != 'r' && current_char != 'R' {
        return false;
    }
    state.position += 1;

    state.context.position +=
        240.0 / state.context.tempo / unsigned_int(state, state.context.default_length) as f64;
    true
}
