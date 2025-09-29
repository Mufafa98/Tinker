use crate::{debug_println, type_defs::State, EPS};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};
pub struct EFA<T> {
    pub(crate) automaton: HashMap<State, HashMap<Option<T>, Vec<State>>>,
    pub(crate) start: Option<State>,
    pub(crate) end: Option<State>,
}

impl<T: Eq + Hash + Debug> EFA<T> {
    pub fn new() -> Self {
        EFA {
            automaton: HashMap::new(),
            start: None,
            end: None,
        }
    }
    pub fn transition(&mut self, i_state: State, input: Option<T>, f_state: State) {
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
        let start = self
            .start
            .map(|s| s.to_string())
            .unwrap_or_else(|| "N/A".to_string());
        let end = self
            .end
            .map(|s| s.to_string())
            .unwrap_or_else(|| "N/A".to_string());
        println!("EFA - Start {} - End {}", start, end);

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

    pub fn get_possible_transitions(
        &self,
        state: &State,
    ) -> Option<&HashMap<Option<T>, Vec<State>>> {
        self.automaton.get(state)
    }

    pub fn closure(&self, state: State) -> Vec<State> {
        let mut result: Vec<State> = vec![state];
        if let Some(transitions) = self.automaton.get(&state) {
            if let Some(eps_transitions) = transitions.get(&None) {
                for transition in eps_transitions {
                    result.extend_from_slice(&self.closure(*transition));
                }
            }
        }
        result
    }
}
impl EFA<char> {
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
    fn recursive_parse(&self, text: &str, state: State) -> Option<()> {
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
            let eps_transition = possible_transitions.get(&None)?;
            for transition in eps_transition {
                debug_println!("  {} --{:?}--> {:2} {:?}", state, EPS, transition, text);

                let result = self.recursive_parse(text, *transition);
                if result.is_some() {
                    return result;
                }
            }
            return None;
        }

        let current_token = current_token.unwrap();

        let direct_transitions = possible_transitions.get(&Some(current_token));
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

        let eps_transition = possible_transitions.get(&None)?;
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
