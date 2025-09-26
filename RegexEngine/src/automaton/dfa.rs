use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    hash::Hash,
};

use crate::{automaton::nfa::NFA, state_generator::StateGenerator, type_defs::State};

type TempDFAState = Vec<State>;

pub struct DFA<T> {
    automaton: HashMap<State, HashMap<T, State>>,
    start: Option<State>,
    end: Option<Vec<State>>,
}

impl<T: Eq + Hash + Debug + Clone> DFA<T> {
    pub fn from_nfa(nfa: &NFA<T>) -> Option<Self> {
        let mut automaton: HashMap<State, HashMap<T, State>> = HashMap::new();
        let mut end_states: Vec<State> = Vec::new();
        let mut visited: HashSet<State> = HashSet::new();
        let mut queue: VecDeque<State> = VecDeque::new();

        let mut state_generator: StateGenerator<TempDFAState, State> = StateGenerator::new();

        let start = nfa.get_start()?;
        let end = nfa.get_end()?;

        let state = state_generator.generate_for(&vec![start]);
        let start = state;

        queue.push_back(state);
        visited.insert(state);

        while let Some(current_state) = queue.pop_front() {
            let current_state_elements = state_generator.get_value(&current_state).unwrap();
            if current_state_elements.contains(&end) {
                end_states.push(current_state);
            }
            automaton.insert(current_state.clone(), HashMap::new());

            let mut new_transitions: HashMap<T, TempDFAState> = HashMap::new();

            for state in current_state_elements.iter() {
                let transitions = nfa.get_possible_transitions(state);
                if transitions.is_none() {
                    continue;
                }
                let transitions = transitions.unwrap();
                let alphabet: Vec<_> = transitions.keys().collect();
                for token in alphabet {
                    new_transitions
                        .entry((*token).clone())
                        .or_default()
                        .extend(transitions.get(token).unwrap());
                }
            }

            for key in new_transitions.keys() {
                let value = new_transitions.get(key).unwrap();

                if state_generator.get_states(value).is_some() {
                    continue;
                }
                let new_state = state_generator.generate_for(value);
                queue.push_back(new_state);
                automaton
                    .get_mut(&current_state)
                    .unwrap()
                    .entry(key.clone())
                    .or_insert(new_state);
            }
        }

        return Some(DFA {
            automaton,
            start: Some(start),
            end: Some(end_states),
        });
    }
    pub fn print_states(&self) {
        println!("DFA States:");
        println!("Start state: {:?}", self.start);
        println!("End state: {:?}", self.end);
        println!("Transitions:");

        let mut sorted_states: Vec<_> = self.automaton.iter().collect();
        sorted_states.sort_by_key(|(state, _)| *state);

        for (from_state, transitions) in sorted_states {
            for (input, to_states) in transitions {
                println!("  {:?} --{:?}--> {:?}", from_state, input, to_states);
            }
        }
    }
    pub fn print(&self) {
        println!("DFA Automaton");
        println!("Start state: {:?}", self.start);
        println!("End states: {:?}", self.end);
        println!("Transitions:");

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
