use crate::compose::State;

pub fn tempo(state: &mut State) -> bool {
    if !state.expect_char('t') && !state.expect_char('T') {
        return false;
    }

    state.context.tempo = state.unsigned_int(120) as f64;
    true
}
