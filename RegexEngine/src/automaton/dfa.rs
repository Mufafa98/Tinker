use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    hash::Hash,
};

use crate::{
    automaton::efa::EFA, debug_println, state_generator::StateGenerator, type_defs::State,
};
#[derive(Clone, Debug)]
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

    fn get_alphabet(&self) -> HashSet<T> {
        self.automaton
            .values()
            .flat_map(|v| v.keys())
            .cloned()
            .collect()
    }

    fn get_transitions_with(&self, token: T) -> Vec<(State, State)> {
        let mut result: Vec<(State, State)> = Vec::new();
        for key in self.automaton.keys() {
            if let Some(destination) = self.automaton.get(key).unwrap().get(&token) {
                result.push((*key, *destination));
            }
        }
        result
    }

    pub fn minimize_from(dfa: DFA<T>) -> Option<Self> {
        let mut dfa = dfa;
        dfa.remove_unreachable();

        let q_states: HashSet<State> = dfa.automaton.keys().cloned().collect();
        let f_vec: Vec<State> = dfa.end.clone().unwrap_or_default();

        let f_states: HashSet<State> = f_vec.iter().cloned().collect();
        let q_diff_f: HashSet<State> = &q_states - &f_states;

        let mut partitions: VecDeque<HashSet<State>> = VecDeque::new();
        let mut work_list: VecDeque<HashSet<State>> = VecDeque::new();

        partitions.push_back(f_states.clone());
        partitions.push_back(q_diff_f.clone());

        work_list.push_back(f_states);
        work_list.push_back(q_diff_f);

        let alphabet = dfa.get_alphabet();

        while let Some(a) = work_list.pop_front() {
            for c in alphabet.iter() {
                let x: HashSet<State> = dfa
                    .get_transitions_with(c.clone())
                    .iter()
                    .filter(|value| a.contains(&value.1))
                    .map(|value| value.0)
                    .collect();

                let mut to_remove: Vec<(HashSet<State>, HashSet<State>, HashSet<State>)> =
                    Vec::new();
                for y in partitions.iter() {
                    let intersection: HashSet<State> = x.intersection(y).cloned().collect();
                    let difference: HashSet<State> = y - &x;
                    if !intersection.is_empty() && !difference.is_empty() {
                        to_remove.push((y.clone(), intersection, difference));
                    }
                }

                for (y, intersection, difference) in to_remove {
                    if let Some(pos) = partitions.iter().position(|x| x == &y) {
                        partitions.remove(pos);
                        partitions.push_back(intersection.clone());
                        partitions.push_back(difference.clone());
                    }

                    if let Some(pos) = work_list.iter().position(|x| x == &y) {
                        work_list.remove(pos);
                        work_list.push_back(intersection.clone());
                        work_list.push_back(difference.clone());
                    } else {
                        if intersection.len() <= difference.len() {
                            work_list.push_back(intersection);
                        } else {
                            work_list.push_back(difference);
                        }
                    }
                }
            }
        }
        println!("{:?} {:?}", partitions, work_list);
        let mut state_generator: StateGenerator<Vec<State>, State> = StateGenerator::new();
        let mut state_table: HashMap<State, State> = HashMap::new();

        let mut start = dfa.start.unwrap();
        let end = dfa.end.unwrap();
        let mut new_end: HashSet<State> = HashSet::new();
        println!("IS {:?} IE {:?}", start, end);
        for partition in partitions {
            let partition: Vec<State> = partition.iter().cloned().collect();
            let new_state = state_generator.generate_for(&partition);
            // if partition.contains(&start) {
            //     start = new_state;
            // }
            for state in partition {
                state_table.insert(state, new_state);
                // if state == start {
                //     start = new_state;
                // }
                if end.contains(&state) {
                    new_end.insert(new_state);
                }
            }
        }

        start = *state_table.get(&start).unwrap();

        println!("US {:?} UE {:?}", start, new_end);
        let mut automaton: HashMap<State, HashMap<T, State>> = HashMap::new();

        let mut queue: VecDeque<State> = VecDeque::new();
        let mut visited: HashSet<State> = HashSet::new();
        queue.push_back(start);
        visited.insert(start);
        println!("{:?}", state_table);
        println!("{:?}", state_generator);
        while let Some(current_state) = queue.pop_front() {
            automaton.insert(current_state, HashMap::new());
            // Linia urmatoare e problema
            // ar trebui sa iei state ul tin tabelul construit anterior
            let decoded_state = state_generator.get_value(&current_state).unwrap();
            println!("CS {:?} DS {:?}", current_state, decoded_state);
            let mut new_transitions: HashMap<T, State> = HashMap::new();

            for state in decoded_state {
                let current_transitions = dfa.automaton.get(&state).unwrap();
                let alphabet: Vec<T> = current_transitions.keys().cloned().collect();
                println!("ALPH {:?}", alphabet);
                println!(
                    "transitions {:?} {:?} {:?} {:?}",
                    dfa.automaton.get(&state),
                    dfa.automaton,
                    dfa.start,
                    state
                );
                for token in alphabet {
                    let destination = current_transitions.get(&token).unwrap();
                    let new_state = state_table.get(destination).unwrap();
                    let entry = new_transitions.insert(token, *new_state);
                    if let Some(temp) = entry {
                        if temp != *new_state {
                            panic!(
                                "Different final states for transition {:?} with {:?}",
                                state, entry
                            )
                        }
                    }
                }
            }

            for key in new_transitions.keys() {
                let value = new_transitions.get(key).unwrap();
                if !visited.contains(value) {
                    visited.insert(*value);
                    queue.push_back(*value);
                }
                automaton
                    .get_mut(&current_state)
                    .unwrap()
                    .entry(key.clone())
                    .or_insert(*value);
            }
        }

        Some(DFA {
            automaton,
            start: Some(start),
            end: Some(new_end.into_iter().collect()),
        })
    }

    fn remove_unreachable(&mut self) {
        let mut reachable: HashSet<State> = HashSet::new();
        let mut new_states: HashSet<State> = HashSet::new();

        reachable.insert(self.start.unwrap());
        new_states.insert(self.start.unwrap());

        loop {
            let mut temp: HashSet<State> = HashSet::new();

            for state in new_states.iter() {
                temp.extend(self.automaton.get(&state).unwrap().values().into_iter());
            }

            new_states = &temp - &new_states;
            reachable.extend(new_states.clone());

            if new_states.is_empty() {
                break;
            }
        }

        let all_states: HashSet<State> = self.automaton.keys().cloned().collect();
        let unreachable: HashSet<State> = &all_states - &reachable;

        for unreachable_state in &unreachable {
            self.automaton.remove(unreachable_state);
        }

        for (_, transitions) in self.automaton.iter_mut() {
            transitions.retain(|_, target_state| !unreachable.contains(&target_state));
        }

        if let Some(ref mut end_states) = self.end {
            end_states.retain(|state| !unreachable.contains(state));
        }
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

impl DFA<char> {
    pub fn parse(&self, text: &str) -> Option<usize> {
        self.print();
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
        if self.end.clone().unwrap().contains(&state) {
            return Some(());
        }

        let possible_transitions = self
            .automaton
            .get(&state)
            .expect("There should not be empty transitions except into final state");
        let current_token = text.chars().next()?;

        let direct_transition = possible_transitions.get(&current_token);
        if direct_transition.is_some() {
            let direct_transition = direct_transition.unwrap();
            // for transition in direct_transition {
            debug_println!(
                "  {} --{:?}--> {:2} {}",
                state,
                current_token,
                direct_transition,
                text
            );

            let result = self.recursive_parse(&text[1..], *direct_transition);
            if result.is_some() {
                return result;
            }
            // }
            return None;
        }

        // let eps_transition = possible_transitions.get(&None)?;
        // for transition in eps_transition {
        //     debug_println!("  {} --{:?}--> {:2} {}", state, EPS, transition, text);
        //     let result = self.recursive_parse(text, *transition);
        //     if result.is_some() {
        //         return result;
        //     }
        // }
        return None;
    }
}
