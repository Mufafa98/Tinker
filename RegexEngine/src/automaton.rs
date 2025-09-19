use crate::{debug_println, type_defs::StateNumber, EPS};
use std::{collections::HashMap, fmt::Debug, hash::Hash};

pub struct Automaton<T> {
    automaton: HashMap<StateNumber, HashMap<T, Vec<StateNumber>>>,
    start: Option<StateNumber>,
    end: Option<StateNumber>,
}

impl<T: Eq + Hash + Debug> Automaton<T> {
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
    pub fn print_states(&self) {
        println!("Automaton States:");
        println!("Start state: {:?}", self.start);
        println!("End state: {:?}", self.end);
        println!("Transitions:");

        let mut sorted_states: Vec<_> = self.automaton.iter().collect();
        sorted_states.sort_by_key(|(state, _)| *state);

        for (from_state, transitions) in sorted_states {
            for (input, to_states) in transitions {
                for to_state in to_states {
                    println!("  {} --{:?}--> {}", from_state, input, to_state);
                }
            }
        }
    }
}
impl Automaton<char> {
    pub fn parse(&self, text: &str) -> Option<usize> {
        let start = self.start.unwrap();

        for (pos, _) in text.chars().enumerate() {
            debug_println!("{:?} {:?}", &text[pos..], start);
            if self.recursive_parse(&text[pos..], start).is_some() {
                return Some(pos);
            }
        }
        // Necessary for empty string that would be matched
        // by "a*" where a is a token from the alphabet
        if self.recursive_parse(text, start).is_some() {
            return Some(0);
        }
        return None;
    }
    fn recursive_parse(&self, text: &str, state: StateNumber) -> Option<()> {
        if state == self.end.unwrap() {
            return Some(());
        }

        let possible_transitions = self
            .automaton
            .get(&state)
            .expect("There should not be empty transitions except into final state");
        let current_token = text.chars().next();

        // Necessary for empty string that would be matched
        // by "a*" where a is a token from the alphabet
        if current_token.is_none() {
            let eps_transition = possible_transitions.get(&EPS)?;
            for transition in eps_transition {
                debug_println!("  {} --{:?}--> {:2} {}", state, EPS, transition, text);

                let result = self.recursive_parse(text, *transition);
                if result.is_some() {
                    return result;
                }
            }
            return None;
        }

        let current_token = current_token.unwrap();

        let direct_transitions = possible_transitions.get(&current_token);
        if direct_transitions.is_some() {
            let direct_transition = direct_transitions.unwrap();
            for transition in direct_transition {
                debug_println!(
                    "  {} --{:?}--> {:2} {}",
                    state,
                    current_token,
                    transition,
                    text
                );

                let result = self.recursive_parse(&text[1..], *transition);
                if result.is_some() {
                    return result;
                }
            }
            return None;
        }

        let eps_transition = possible_transitions.get(&EPS)?;
        for transition in eps_transition {
            debug_println!("  {} --{:?}--> {:2} {}", state, EPS, transition, text);
            let result = self.recursive_parse(text, *transition);
            if result.is_some() {
                return result;
            }
        }
        return None;
    }
}
