use crate::compose::State;

pub fn envelope(state: &mut State) -> bool {
    state.transaction();
    if !state.expect_char('n') && !state.expect_char('N') {
        return false;
    }

    let mut envelope = [0.0, 0.0, 0.0, 0.0];

    for i in 0..=3 {
        let value = state.unsigned_int(0) as f64 / 100.0;
        envelope[i] = value;

        if i == 3 {
            break;
        }

        let current_char = state.take_char();
        if current_char != ',' {
            state.rollback();
            return false;
        }
        state.position += 1;
    }

    state.context.envelope.attack = envelope[0];
    state.context.envelope.decay = envelope[1];
    state.context.envelope.sustain = envelope[2];
    state.context.envelope.release = envelope[3];
    true
}
