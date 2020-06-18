use crate::compose::State;

pub fn rest(state: &mut State) -> bool {
    if !state.expect_char('r') && !state.expect_char('R') {
        return false;
    }

    state.context.position +=
        240.0 / state.context.tempo / state.unsigned_int(state.context.default_length) as f64;
    true
}
