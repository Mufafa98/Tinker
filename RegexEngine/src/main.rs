use std::collections::VecDeque;

mod automaton;
mod state_generator;
mod tree;
mod type_defs;

use automaton::Automaton;
use state_generator::StateGenerator;
use tree::Node;

const EPS: char = 'ε';

fn hierarchy(ch: char) -> u8 {
    match ch {
        '(' | ')' => 0,
        '|' => 1,
        '·' => 2,
        '*' => 3,
        _ => 4,
    }
}

fn is_operator(ch: &char) -> bool {
    let ch = *ch;
    if ch == '|' || ch == '·' || ch == '*' {
        return true;
    }
    return false;
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

fn build_automaton(regex: &str) {
    let tree = parse_regex(regex);
    let mut post_order: Vec<char> = Vec::new();
    tree.post_order(&mut post_order);

    let mut automaton: Automaton<char> = Automaton::new();
    let mut state_generator: StateGenerator = StateGenerator::new();
    let mut tree_stack: VecDeque<usize> = VecDeque::new();

    for (pos, token) in post_order.iter().enumerate() {
        if !is_operator(token) {
            let (i_state, f_state) = state_generator.generate_for(pos);

            automaton.transition(i_state, *token, f_state);
            automaton.empty_transition(f_state);

            tree_stack.push_back(pos);
        } else {
            match token {
                '*' => {
                    let (i_state, f_state) = state_generator.generate_for(pos);
                    // child position
                    let child = tree_stack
                        .pop_back()
                        .expect("Operator \'*\' expected an operand");
                    let (child_i, child_f) = state_generator.get_states(child);

                    automaton.transition(i_state, EPS, f_state);
                    automaton.transition(i_state, EPS, child_i);

                    automaton.transition(child_f, EPS, f_state);
                    automaton.transition(child_f, EPS, child_i);

                    tree_stack.push_back(pos);
                }
                '|' => {
                    let (i_state, f_state) = state_generator.generate_for(pos);
                    let child_r = tree_stack
                        .pop_back()
                        .expect("Operator \'·\' expected two operands");
                    let child_l = tree_stack
                        .pop_back()
                        .expect("Operator \'·\' expected two operands");

                    let (child_l_i, child_l_f) = state_generator.get_states(child_l);
                    let (child_r_i, child_r_f) = state_generator.get_states(child_r);

                    automaton.transition(i_state, EPS, child_l_i);
                    automaton.transition(i_state, EPS, child_r_i);

                    automaton.transition(child_l_f, EPS, f_state);
                    automaton.transition(child_r_f, EPS, f_state);

                    automaton.empty_transition(f_state);

                    tree_stack.push_back(pos);
                }
                '·' => {
                    let child_r = tree_stack
                        .pop_back()
                        .expect("Operator \'·\' expected two operands");
                    let child_l = tree_stack
                        .pop_back()
                        .expect("Operator \'·\' expected two operands");

                    let (child_l_i, child_l_f) = state_generator.get_states(child_l);
                    let (child_r_i, child_r_f) = state_generator.get_states(child_r);

                    state_generator.insert_with(pos, child_l_i, child_r_f);

                    automaton.transition(child_l_f, EPS, child_r_i);

                    tree_stack.push_back(pos);
                }
                _ => panic!("Unknown token {}", token),
            }
        }
    }

    let root_pos = tree_stack
        .pop_back()
        .expect("Root of the expression was not pushed to the stack");
    let (start, end) = state_generator.get_states(root_pos);
    automaton.set_start(start);
    automaton.set_end(end);

    // // Printing
    // let mut automaton_keys: Vec<&StateNumber> = automaton.automaton.keys().collect();
    // automaton_keys.sort();
    // for key in automaton_keys {
    //     println!("{}: {:?}", key, automaton.automaton.get(key).unwrap());
    // }
}

fn main() {
    // let temp_test = "(a*·(a|b)·b*)";
    let temp_test = "(a|b*·c)";
    build_automaton(temp_test);
}
