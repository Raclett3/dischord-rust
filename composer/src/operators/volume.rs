use crate::compose::State;

pub fn volume(state: &mut State) -> bool {
    if !state.expect_char('v') && !state.expect_char('V') {
        return false;
    }

    state.context.volume = state.unsigned_int(100) as f64 / 100.0 * 0.5;
    true
}
