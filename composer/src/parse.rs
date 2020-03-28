use crate::compose::State;

pub fn skip_spaces(state: &mut State) {
    loop {
        if state.position >= state.input.len() {
            break;
        }
        let current_char = state.input.as_bytes()[state.position] as char;
        if current_char == ' ' || current_char == '\n' || current_char == '\r' {
            state.position += 1;
        } else {
            break;
        }
    }
}

pub fn is_eof(state: &mut State) -> bool {
    skip_spaces(state);
    state.position >= state.input.len()
}

pub fn take_char(state: &mut State) -> char {
    skip_spaces(state);
    if state.position < state.input.len() {
        state.input.as_bytes()[state.position] as char
    } else {
        '\0'
    }
}

pub fn unsigned_int(state: &mut State, default: u32) -> u32 {
    let mut result: u32 = 0;
    while !is_eof(state) {
        let current_char = take_char(state);
        if current_char >= '0' && current_char <= '9' {
            result = result * 10 + (current_char as u8 - b'0') as u32;
            state.position += 1;
        } else {
            break;
        }
    }
    if result > 0 {
        result
    } else {
        default
    }
}
