use crate::parse::*;
use crate::compose::State;

pub fn volume(state: &mut State) -> bool {
    let current_char = take_char(state);
    if current_char != 'v' && current_char != 'V' {
        return false;
    }
    state.position += 1;

    state.context.volume = unsigned_int(state, 100) as f64 / 100.0 * 0.5;
    true
}
