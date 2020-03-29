use crate::parse::*;
use crate::compose::State;

pub fn rewind(state: &mut State) -> bool {
    let current_char = take_char(state);
    if current_char == ';' {
        state.context.position = 0.0;
        state.context.octave = 0;
        state.context.volume = 0.5;
    } else {
        return false;
    }
    state.position += 1;
    true
}
