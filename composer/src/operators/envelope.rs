use crate::compose::State;

pub fn envelope(state: &mut State) -> bool {
    state.transaction();
    if !state.expect_char('n') && !state.expect_char('N') {
        return false;
    }

    let envelope = state.unsigned_int_vec();
    if envelope.len() < 4 {
        state.rollback();
        return false;
    }

    state.context.envelope.attack = envelope[0] as f64 / 100.0;
    state.context.envelope.decay = envelope[1] as f64 / 100.0;
    state.context.envelope.sustain = envelope[2] as f64 / 100.0;
    state.context.envelope.release = envelope[3] as f64 / 100.0;
    true
}
