use crate::compose::State;

pub fn default_length(state: &mut State) -> bool {
    if !state.expect_char('l') && !state.expect_char('L') {
        return false;
    }

    state.context.default_length = state.unsigned_int(8);
    true
}
