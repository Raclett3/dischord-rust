use crate::compose::State;
use crate::parse::*;

pub fn rewind(state: &mut State) -> bool {
    if expect_char(state, ';') {
        state.context.position = 0.0;
        state.context.octave = 0;
        state.context.volume = 0.5;
    } else {
        return false;
    }
    true
}
