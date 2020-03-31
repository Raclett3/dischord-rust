use crate::compose::State;
use crate::parse::*;

pub fn rest(state: &mut State) -> bool {
    if !expect_char(state, 'r') && !expect_char(state, 'R') {
        return false;
    }

    state.context.position +=
        240.0 / state.context.tempo / unsigned_int(state, state.context.default_length) as f64;
    true
}
