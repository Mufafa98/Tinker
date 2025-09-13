use crate::type_defs::StateNumber;
use std::collections::HashMap;

#[derive(Debug)]
pub struct StateGenerator {
    states: HashMap<usize, (StateNumber, StateNumber)>,
    state_counter: StateNumber,
}

impl StateGenerator {
    pub fn new() -> Self {
        StateGenerator {
            states: HashMap::new(),
            state_counter: 1,
        }
    }

    pub fn generate_for(&mut self, pos: usize) -> (StateNumber, StateNumber) {
        let i_state = self.state_counter;
        let f_state = self.state_counter + 1;
        self.state_counter += 2;

        if self.states.insert(pos, (i_state, f_state)).is_some() {
            panic!("There shold not be the same position entered twice")
        }
        (i_state, f_state)
    }

    pub fn insert_with(&mut self, pos: usize, i_state: StateNumber, f_state: StateNumber) {
        if self.states.insert(pos, (i_state, f_state)).is_some() {
            panic!("There shold not be the same position entered twice")
        }
    }

    pub fn get_states(&self, pos: usize) -> (StateNumber, StateNumber) {
        let (i_state, f_state) = self.states.get(&pos).expect(&format!(
            "State was not set for character in post order string at position {}",
            pos
        ));
        (*i_state, *f_state)
    }
}
