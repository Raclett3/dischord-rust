use crate::parse::*;
use crate::compose::State;

pub fn default_length(state: &mut State) -> bool {
    let current_char = take_char(state);
    if current_char != 'l' && current_char != 'L' {
        return false;
    }
    state.position += 1;

    state.context.default_length = unsigned_int(state, 8);
    true
}
