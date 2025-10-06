use crate::automaton::dfa::DFA;
use crate::automaton::efa::EFA;
use crate::state_generator::StateGenerator;
use crate::tree::Node;
use crate::type_defs::State;
use std::collections::VecDeque;

pub type RegexParser = GenericRegexParser<char>;

pub struct GenericRegexParser<T> {
    automaton: DFA<T>,
    efa: EFA<T>,
}

impl GenericRegexParser<char> {
    pub fn from(regex: &str) -> Self {
        let processed_regex = add_implicit_concatenation(regex);

        let tree = parse_regex(&processed_regex);
        let mut post_order: Vec<char> = Vec::new();
        tree.post_order(&mut post_order);

        let mut efa: EFA<char> = EFA::new();
        let mut state_generator: StateGenerator<usize, (State, State)> = StateGenerator::new();
        let mut tree_stack: VecDeque<usize> = VecDeque::new();

        for (pos, token) in post_order.iter().enumerate() {
            if !is_operator(token) {
                let (i_state, f_state) = state_generator.generate_for(&pos);

                efa.transition(i_state, Some(*token), f_state);
                efa.empty_transition(f_state);

                tree_stack.push_back(pos);
            } else {
                match token {
                    '*' => {
                        let (i_state, f_state) = state_generator.generate_for(&pos);
                        // child position
                        let child = tree_stack
                            .pop_back()
                            .expect("Operator \'*\' expected an operand");
                        let (child_i, child_f) = state_generator.get_states(&child).unwrap();

                        efa.transition(i_state, None, f_state);
                        efa.transition(i_state, None, child_i);

                        efa.transition(child_f, None, f_state);
                        efa.transition(child_f, None, child_i);

                        tree_stack.push_back(pos);
                    }
                    '|' => {
                        let (i_state, f_state) = state_generator.generate_for(&pos);
                        let child_r = tree_stack
                            .pop_back()
                            .expect("Operator \'·\' expected two operands");
                        let child_l = tree_stack
                            .pop_back()
                            .expect("Operator \'·\' expected two operands");

                        let (child_l_i, child_l_f) = state_generator.get_states(&child_l).unwrap();
                        let (child_r_i, child_r_f) = state_generator.get_states(&child_r).unwrap();

                        efa.transition(i_state, None, child_l_i);
                        efa.transition(i_state, None, child_r_i);

                        efa.transition(child_l_f, None, f_state);
                        efa.transition(child_r_f, None, f_state);

                        efa.empty_transition(f_state);

                        tree_stack.push_back(pos);
                    }
                    '·' => {
                        let child_r = tree_stack
                            .pop_back()
                            .expect("Operator \'·\' expected two operands");
                        let child_l = tree_stack
                            .pop_back()
                            .expect("Operator \'·\' expected two operands");

                        let (child_l_i, child_l_f) = state_generator.get_states(&child_l).unwrap();
                        let (child_r_i, child_r_f) = state_generator.get_states(&child_r).unwrap();

                        state_generator.insert_with(&pos, &(child_l_i, child_r_f));

                        efa.transition(child_l_f, None, child_r_i);

                        tree_stack.push_back(pos);
                    }
                    _ => panic!("Unknown token {}", token),
                }
            }
        }

        let root_pos = tree_stack
            .pop_back()
            .expect("Root of the expression was not pushed to the stack");
        let (start, end) = state_generator.get_states(&root_pos).unwrap();
        efa.set_start(start);
        efa.set_end(end);

        let dfa = DFA::from_efa(&efa.clone()).unwrap();
        // let automaton = dfa;
        let automaton = DFA::minimize_from(dfa).unwrap();
        // automaton.print();
        return GenericRegexParser { automaton, efa };
    }
    pub fn parse(&self, text: &str) -> Option<usize> {
        self.automaton.parse(text)
    }
    pub fn get_efa_temp(self) -> EFA<char> {
        return self.efa;
    }
    pub fn get_dfa_temp(&self) -> DFA<char> {
        return self.automaton.clone();
    }
}

fn hierarchy(ch: char) -> u8 {
    match ch {
        '(' | ')' => 1,
        '|' => 2,
        '·' => 3,
        '*' => 4,
        _ => 0,
    }
}

fn is_operator(ch: &char) -> bool {
    let ch = *ch;
    if ch == '|' || ch == '·' || ch == '*' || ch == '(' || ch == ')' {
        return true;
    }
    return false;
}

fn is_alphabet(ch: &char) -> bool {
    return !is_operator(ch);
}

fn parse_regex(r: &str) -> Node<char> {
    let mut op_stack: VecDeque<char> = VecDeque::new(); // operator stack
    let mut tr_stack: VecDeque<Node<char>> = VecDeque::new(); // tree stack
    for ch in r.chars() {
        match ch {
            '(' => op_stack.push_back(ch),
            ')' => loop {
                build_tree(&mut op_stack, &mut tr_stack);
                let op = op_stack.back();
                if op.is_none() || *op.unwrap() == '(' {
                    op_stack.pop_back();
                    break;
                }
            },
            _ => {
                if is_operator(&ch) {
                    while hierarchy(*op_stack.back().unwrap_or(&' ')) >= hierarchy(ch) {
                        build_tree(&mut op_stack, &mut tr_stack);
                    }
                    op_stack.push_back(ch);
                } else {
                    tr_stack.push_back(Node {
                        value: ch,
                        left: None,
                        right: None,
                    });
                }
            }
        }
    }

    while !op_stack.is_empty() {
        build_tree(&mut op_stack, &mut tr_stack);
    }
    return tr_stack.back().unwrap().clone();
}

fn build_tree(op_stack: &mut VecDeque<char>, tr_stack: &mut VecDeque<Node<char>>) {
    let op = op_stack.pop_back().unwrap();
    let t1 = tr_stack.pop_back().unwrap();

    match op {
        '|' | '·' => {
            let t2 = tr_stack.pop_back().unwrap();
            let t: Node<char> = Node::with_lr(op, t2, t1);
            tr_stack.push_back(t);
        }
        '*' => {
            let t: Node<char> = Node::with_l(op, t1);
            tr_stack.push_back(t);
        }

        _ => {
            println!("Unimplemented operand {}", op);
            tr_stack.push_back(t1);
        }
    }
}

fn add_implicit_concatenation(regex: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = regex.chars().collect();

    for i in 0..chars.len() {
        let current = chars[i];
        result.push(current);

        // Check if we need to add implicit concatenation
        if i < chars.len() - 1 {
            let next = chars[i + 1];

            let should_add_concat = match (current, next) {
                (c1, c2) if is_alphabet(&c1) && is_alphabet(&c2) => true,
                (')', '(') => true,
                ('*', c) if is_alphabet(&c) => true,
                ('*', '(') => true,
                _ => false,
            };
            if should_add_concat {
                result.push('·');
            }
        }
    }

    result
}
