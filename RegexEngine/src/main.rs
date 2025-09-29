mod automaton;
mod macros;
mod regex_parser;
mod state_generator;
mod tree;
mod type_defs;

#[cfg(test)]
mod tests;

use crate::{automaton::dfa::DFA, automaton::nfa::NFA, regex_parser::RegexParser};

const EPS: char = 'ε';
fn main() {
    let automaton: RegexParser = RegexParser::from("a|b*c");
    automaton.parse("asd");
    let efa = automaton.get_automaton_temp();
    efa.print();
    let nfa = NFA::from_efa(&efa).unwrap();
    nfa.print();
    let dfa = DFA::from_efa(&efa).unwrap();
    dfa.print();
}
