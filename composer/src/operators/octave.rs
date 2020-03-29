use crate::compose::State;
use crate::parse::*;

pub fn octave(state: &mut State) -> bool {
    let current_char = take_char(state);
    if current_char == '<' {
        state.context.octave += 1;
    } else if current_char == '>' {
        state.context.octave -= 1;
    } else {
        return false;
    }
    state.position += 1;
    true
}
