use crate::compose::State;
use crate::parse::*;

pub fn tempo(state: &mut State) -> bool {
    if !expect_char(state, 't') && !expect_char(state, 'T') {
        return false;
    }

    state.context.tempo = unsigned_int(state, 120) as f64;
    true
}
