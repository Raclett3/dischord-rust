use crate::compose::State;
use crate::parse::*;

pub fn default_length(state: &mut State) -> bool {
    if !expect_char(state, 'l') && !expect_char(state, 'L') {
        return false;
    }

    state.context.default_length = unsigned_int(state, 8);
    true
}
