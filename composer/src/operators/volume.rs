use crate::compose::State;
use crate::parse::*;

pub fn volume(state: &mut State) -> bool {
    if !expect_char(state, 'v') && !expect_char(state, 'V') {
        return false;
    }

    state.context.volume = unsigned_int(state, 100) as f64 / 100.0 * 0.5;
    true
}
