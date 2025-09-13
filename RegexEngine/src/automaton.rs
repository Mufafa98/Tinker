use crate::type_defs::StateNumber;
use std::{collections::HashMap, hash::Hash};

pub struct Automaton<T> {
    automaton: HashMap<StateNumber, HashMap<T, Vec<StateNumber>>>,
    start: Option<StateNumber>,
    end: Option<StateNumber>,
}

impl<T: Eq + Hash> Automaton<T> {
    pub fn new() -> Self {
        Automaton {
            automaton: HashMap::new(),
            start: None,
            end: None,
        }
    }
    pub fn transition(&mut self, i_state: StateNumber, input: T, f_state: StateNumber) {
        self.automaton
            .entry(i_state)
            .or_default()
            .entry(input)
            .or_default()
            .push(f_state);
    }
    pub fn empty_transition(&mut self, i_state: StateNumber) {
        self.automaton.insert(i_state, HashMap::new());
    }
    pub fn set_start(&mut self, start: StateNumber) {
        self.start = Some(start);
    }
    pub fn set_end(&mut self, end: StateNumber) {
        self.end = Some(end);
    }
}
