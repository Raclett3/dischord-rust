use crate::compose::State;

pub fn rest(state: &mut State) -> bool {
    if !state.expect_char('r') && !state.expect_char('R') {
        return false;
    }

    state.context.position += state.take_note_duration();
    true
}
