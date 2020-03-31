use crate::compose::State;
use crate::parse::*;

pub fn envelope(state: &mut State) -> bool {
    let initial_position = state.position;
    if !expect_char(state, 'n') && !expect_char(state, 'N') {
        return false;
    }

    let mut envelope = [0.0, 0.0, 0.0, 0.0];

    for i in 0..=3 {
        let value = unsigned_int(state, 0) as f64 / 100.0;
        envelope[i] = value;

        if i == 3 {
            break;
        }

        let current_char = take_char(state);
        if current_char != ',' {
            state.position = initial_position;
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
