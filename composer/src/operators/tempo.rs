use crate::parse::*;
use crate::compose::State;

pub fn tempo(state: &mut State) -> bool {
    let current_char = take_char(state);
    if current_char != 't' && current_char != 'T' {
        return false;
    }
    state.position += 1;

    state.context.tempo = unsigned_int(state, 120) as f64;
    true
}
