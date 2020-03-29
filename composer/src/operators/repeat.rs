use crate::compose::{StackItem, State};
use crate::parse::*;

pub fn find_termination(state: &State) -> Option<usize> {
    let mut position = state.position + 1;
    let mut nest: usize = 1;
    while nest > 0 {
        if position >= state.input.len() {
            return None;
        }

        let current_char = state.input.as_bytes()[position] as char;
        if current_char == '[' {
            nest += 1;
        } else if current_char == ']' {
            nest -= 1;
        }
        position += 1;
    }
    Some(position)
}

pub fn repeat(state: &mut State) -> bool {
    let current_char = take_char(state);
    if current_char == '[' {
        let found = find_termination(state);
        match found {
            Some(termination) => {
                let start = state.position;
                state.position = termination;
                let count = unsigned_int(state, 2);
                if count > 0 {
                    state.repeat_stack.push(StackItem {
                        position: start,
                        repeat_count: count - 1,
                    });
                    state.position = start + 1;
                }
            }
            None => return false,
        }
    } else if current_char == ']' {
        let popped = state.repeat_stack.pop();
        match popped {
            Some(item) => {
                if item.repeat_count > 0 {
                    state.repeat_stack.push(StackItem {
                        position: item.position,
                        repeat_count: item.repeat_count - 1,
                    });
                    state.position = item.position + 1;
                } else {
                    state.position += 1;
                    unsigned_int(state, 0);
                }
            }
            None => return false,
        }
    } else {
        return false;
    }
    true
}
