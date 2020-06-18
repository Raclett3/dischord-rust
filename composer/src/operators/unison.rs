use crate::compose::State;

pub fn unison(state: &mut State) -> bool {
    state.transaction();
    if !state.expect_char('u') && !state.expect_char('U') {
        return false;
    }

    let unison_count = state.unsigned_int(1);
    if !state.expect_char(',') {
        state.rollback();
        return false;
    }
    let detune = 1.0 + (state.unsigned_int(0) as i64 - 100) as f64 / 2000.0;

    state.context.unison_count = unison_count;
    state.context.unison_detune = detune;
    true
}
