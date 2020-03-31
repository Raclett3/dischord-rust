use crate::compose::State;
use crate::parse::*;

pub fn octave(state: &mut State) -> bool {
    if expect_char(state, '<') {
        state.context.octave += 1;
    } else if expect_char(state, '>') {
        state.context.octave -= 1;
    } else {
        return false;
    }
    true
}
