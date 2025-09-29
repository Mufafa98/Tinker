use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    hash::Hash,
};

use crate::{automaton::efa::EFA, state_generator::StateGenerator, type_defs::State};

pub struct DFA<T> {
    automaton: HashMap<State, HashMap<T, State>>,
    start: Option<State>,
    end: Option<Vec<State>>,
}

impl<T: Eq + Hash + Debug + Clone> DFA<T> {
    pub fn from_efa(efa: &EFA<T>) -> Option<Self> {
        let start = efa.get_start()?;
        let end = efa.get_end()?;

        let mut automaton: HashMap<State, HashMap<T, State>> = HashMap::new();
        let mut closures: HashMap<State, Vec<State>> = HashMap::new();
        let mut end_states: HashSet<State> = HashSet::new();

        let mut queue: VecDeque<State> = VecDeque::new();
        let mut visited: HashSet<State> = HashSet::new();

        let mut state_generator: StateGenerator<Vec<State>, State> = StateGenerator::new();

        let closure = efa.closure(start);
        closures.insert(start, closure.clone());
        let start = state_generator.generate_for(&closure);
        queue.push_back(start);

        while let Some(current_state) = queue.pop_front() {
            automaton.insert(current_state, HashMap::new());
            let mut new_transitions: HashMap<T, Vec<State>> = HashMap::new();
            let current_states = state_generator.get_value(&current_state).unwrap();
            if current_states.contains(&end) {
                end_states.insert(current_state);
            }
            // For each independent state
            for state in current_states.iter() {
                // Get its possible transitions
                let transitions = efa.get_possible_transitions(&state);
                if transitions.is_none() {
                    continue;
                }
                let transitions = transitions.unwrap();
                let alphabet: Vec<_> = transitions.keys().collect();
                // Using the tokens, for each individual transitions
                for token_option in alphabet {
                    if token_option.is_none() {
                        continue;
                    }
                    let token = token_option.clone().unwrap();
                    // Get its destinations
                    let destinations = transitions.get(&Some(token.clone())).unwrap();
                    let mut new_destinations: HashSet<State> = HashSet::new();

                    // Calculate the closure for each destination
                    for destination in destinations {
                        new_destinations.extend(
                            closures
                                .entry(*destination)
                                .or_insert_with(|| {
                                    let mut temp = efa.closure(*destination);
                                    temp.sort();
                                    temp
                                })
                                .iter(),
                        );
                    }

                    // Generate the new possible transitions
                    new_transitions
                        .entry(token)
                        .or_default()
                        .extend(new_destinations.clone());
                }
            }
            // For each possible transition
            for key in new_transitions.keys() {
                let mut value = new_transitions.get(key).unwrap().clone();
                value.sort();

                // Check if it was visited. If not add it to the queue
                let new_state = state_generator.generate_for(&value.clone());
                if !visited.contains(&new_state) {
                    queue.push_back(new_state);
                    visited.insert(new_state);
                }
                automaton
                    .get_mut(&current_state)
                    .unwrap()
                    .entry(key.clone())
                    .or_insert(new_state);
            }
        }

        Some(DFA {
            automaton,
            start: Some(start),
            end: Some(end_states.into_iter().collect()),
        })
    }

    pub fn print(&self) {
        let start = self
            .start
            .map(|s| format!("{:?}", s))
            .unwrap_or_else(|| "N/A".to_string());
        let end = self
            .end
            .clone()
            .map(|s| format!("{:?}", s))
            .unwrap_or_else(|| "N/A".to_string());
        println!("DFA - Start {} - End {}", start, end);

        let mut sorted_states: Vec<_> = self.automaton.iter().collect();
        sorted_states.sort_by_key(|(state, _)| *state);

        let alphabet: HashSet<_> = self
            .automaton
            .values()
            .flat_map(|value| value.keys())
            .collect();

        let max_state_width = sorted_states
            .iter()
            .map(|(state, _)| format!("{:?}", state).len())
            .max()
            .unwrap_or(10)
            .max(4);

        print!("{:^width$} |", "State", width = max_state_width);
        for symbol in &alphabet {
            print!(" {:^8} |", format!("{:?}", symbol));
        }
        println!();

        print!("{:-<width$}-|", "", width = max_state_width + 1);
        for _ in &alphabet {
            print!("-----------");
        }
        println!();

        for (state, transitions) in sorted_states {
            let state_str = format!("{:?}", state);

            print!(" {:^width$} |", state_str, width = max_state_width);

            for symbol in &alphabet {
                if let Some(target_state) = transitions.get(symbol) {
                    let target_str = format!("{:?}", target_state);
                    if target_str.len() > 8 {
                        print!(" {:>7}… |", &target_str[..7]);
                    } else {
                        print!(" {:^8} |", target_str);
                    }
                } else {
                    print!(" {:^8} |", "-");
                }
            }
            println!();
        }
    }
}
