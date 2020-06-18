use crate::compose::State;

pub fn octave(state: &mut State) -> bool {
    if state.expect_char('<') {
        state.context.octave += 1;
    } else if state.expect_char('>') {
        state.context.octave -= 1;
    } else {
        return false;
    }
    true
}
