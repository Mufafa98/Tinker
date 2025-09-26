use crate::{debug_println, type_defs::State, EPS};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

pub struct NFA<T> {
    automaton: HashMap<State, HashMap<T, Vec<State>>>,
    start: Option<State>,
    end: Option<State>,
}

impl<T: Eq + Hash + Debug> NFA<T> {
    pub fn new() -> Self {
        NFA {
            automaton: HashMap::new(),
            start: None,
            end: None,
        }
    }
    pub fn transition(&mut self, i_state: State, input: T, f_state: State) {
        self.automaton
            .entry(i_state)
            .or_default()
            .entry(input)
            .or_default()
            .push(f_state);
    }
    pub fn empty_transition(&mut self, i_state: State) {
        self.automaton.insert(i_state, HashMap::new());
    }
    pub fn set_start(&mut self, start: State) {
        self.start = Some(start);
    }
    pub fn set_end(&mut self, end: State) {
        self.end = Some(end);
    }

    pub fn print(&self) {
        println!("EFA Automaton");
        println!("Start state: {:?}", self.start);
        println!("End state: {:?}", self.end);
        println!("Transitions:");

        let mut sorted_states: Vec<_> = self.automaton.iter().collect();
        sorted_states.sort_by_key(|(state, _)| *state);

        let alphabet: HashSet<_> = self
            .automaton
            .values()
            .flat_map(|value| value.keys())
            .collect();

        print!("State |");
        for symbol in &alphabet {
            print!(" {:^8} |", format!("{:?}", symbol));
        }
        println!();
        print!("------|");
        for _ in &alphabet {
            print!("-----------");
        }
        println!();
        // Print each state row
        for (state, transitions) in sorted_states {
            print!(" {:^4} |", state);

            for symbol in &alphabet {
                if let Some(target_states) = transitions.get(symbol) {
                    let targets: Vec<String> =
                        target_states.iter().map(|s| s.to_string()).collect();
                    print!(" {:^8} |", targets.join(","));
                } else {
                    print!(" {:^8} |", "-");
                }
            }
            println!();
        }
    }
    pub fn get_start(&self) -> Option<State> {
        self.start
    }
    pub fn get_end(&self) -> Option<State> {
        self.end
    }
    pub fn get_possible_transitions(&self, state: &State) -> Option<&HashMap<T, Vec<State>>> {
        self.automaton.get(state)
    }
}
