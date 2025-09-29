use crate::type_defs::State;
use std::{collections::HashMap, fmt::Debug, hash::Hash};

pub trait StateGeneration {
    fn generate(counter: &mut State) -> Self;
}

impl StateGeneration for State {
    fn generate(counter: &mut State) -> Self {
        let state = *counter;
        *counter += 1;
        state
    }
}

impl StateGeneration for (State, State) {
    fn generate(counter: &mut State) -> Self {
        let state_1 = *counter;
        let state_2 = *counter + 1;
        *counter += 2;
        (state_1, state_2)
    }
}

#[derive(Debug)]
pub struct StateGenerator<ValueType, ValueState> {
    states: HashMap<ValueType, ValueState>,
    values: HashMap<ValueState, ValueType>,
    state_counter: State,
}

impl<
        ValueType: Eq + Hash + Clone + Debug,
        ValueState: StateGeneration + Clone + Eq + Hash + Debug,
    > StateGenerator<ValueType, ValueState>
{
    pub fn new() -> Self {
        StateGenerator {
            states: HashMap::new(),
            values: HashMap::new(),
            state_counter: 1,
        }
    }
    pub fn generate_for(&mut self, value: &ValueType) -> ValueState {
        let state = {
            if self.states.contains_key(value) {
                return self.states.get(value).unwrap().clone();
            } else {
                let state = self.generate_return();
                if self.states.insert(value.clone(), state.clone()).is_some() {
                    panic!("There shold not be the same value entered twice")
                }
                state
            }
        };

        if self.values.insert(state.clone(), value.clone()).is_some() {
            panic!("An error occured. There should not be two states with equal values. Check your StateGeneration trait implementation")
        }
        state
    }

    pub fn insert_with(&mut self, value: &ValueType, state: &ValueState) {
        if self.states.insert(value.clone(), state.clone()).is_some() {
            panic!("There shold not be the same value entered twice")
        }
        if self.values.insert(state.clone(), value.clone()).is_some() {
            panic!("An error occured. There should not be two states with equal values. Check your StateGeneration trait implementation")
        }
    }

    pub fn get_states(&self, value: &ValueType) -> Option<ValueState> {
        Some(self.states.get(&value)?.clone())
    }

    pub fn get_value(&self, state: &ValueState) -> Option<ValueType> {
        Some(self.values.get(&state)?.clone())
    }
    fn generate_return(&mut self) -> ValueState {
        ValueState::generate(&mut self.state_counter)
    }
}
