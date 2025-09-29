use crate::{automaton::efa::EFA, type_defs::State};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    hash::Hash,
};

pub struct NFA<T> {
    automaton: HashMap<Vec<State>, HashMap<T, Vec<State>>>,
    start: Option<Vec<State>>,
    end: Option<State>,
}

impl<T: Eq + Hash + Debug + Clone> NFA<T> {
    pub fn from_efa(efa: &EFA<T>) -> Option<Self> {
        let start = efa.get_start()?;
        let end = efa.get_end()?;

        let mut automaton: HashMap<Vec<State>, HashMap<T, Vec<State>>> = HashMap::new();
        let mut closures: HashMap<State, Vec<State>> = HashMap::new();

        let mut queue: VecDeque<Vec<State>> = VecDeque::new();
        let mut visited: HashSet<Vec<State>> = HashSet::new();

        let closure = efa.closure(start);
        closures.insert(start, closure.clone());
        queue.push_back(closure.clone());

        while let Some(current_states) = queue.pop_front() {
            automaton.insert(current_states.clone(), HashMap::new());
            let mut new_transitions: HashMap<T, Vec<State>> = HashMap::new();

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
                if !visited.contains(&value.clone()) {
                    queue.push_back(value.clone());
                    visited.insert(value.clone());
                }

                automaton
                    .get_mut(&current_states.clone())
                    .unwrap()
                    .entry(key.clone())
                    .or_insert(value.clone());
            }
        }

        Some(NFA {
            automaton,
            start: Some(closure),
            end: Some(end),
        })
    }

    pub fn print(&self) {
        let start = self
            .start
            .as_ref()
            .map(|s| format!("{:?}", s))
            .unwrap_or_else(|| "N/A".to_string());
        let end = self
            .end
            .clone()
            .map(|s| format!("{:?}", s))
            .unwrap_or_else(|| "N/A".to_string());
        println!("NFA - Start {} - End {}", start, end);

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
            .max(5);

        let mut symbol_widths: HashMap<&T, usize> = HashMap::new();
        for symbol in &alphabet {
            let symbol_header_width = format!("{:?}", symbol).len();
            let max_content_width = sorted_states
                .iter()
                .map(|(_, transitions)| {
                    if let Some(target_state) = transitions.get(symbol) {
                        format!("{:?}", target_state).len()
                    } else {
                        1 // "-" character
                    }
                })
                .max()
                .unwrap_or(1);

            symbol_widths.insert(symbol, symbol_header_width.max(max_content_width).max(3));
        }

        print!(" {:^width$} |", "State", width = max_state_width);
        for symbol in &alphabet {
            let width = symbol_widths[symbol];
            print!(" {:^width$} |", format!("{:?}", symbol), width = width);
        }
        println!();

        print!("{:-<width$}-|", "", width = max_state_width + 1);
        for symbol in &alphabet {
            let width = symbol_widths[symbol];
            print!("{:-<width$}|", "", width = width + 2);
        }
        println!();

        for (state, transitions) in sorted_states {
            let state_str = format!("{:?}", state);
            print!(" {:^width$} |", state_str, width = max_state_width);

            for symbol in &alphabet {
                let width = symbol_widths[symbol];
                if let Some(target_state) = transitions.get(symbol) {
                    let target_str = format!("{:?}", target_state);
                    print!(" {:^width$} |", target_str, width = width);
                } else {
                    print!(" {:^width$} |", "-", width = width);
                }
            }
            println!();
        }
    }
}
