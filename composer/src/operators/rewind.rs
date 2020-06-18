use crate::compose::State;

pub fn rewind(state: &mut State) -> bool {
    if state.expect_char(';') {
        state.context.position = 0.0;
        state.context.octave = 0;
        state.context.volume = 0.5;
    } else {
        return false;
    }
    true
}
