use std::collections::VecDeque;

mod tree;
use tree::Node;

fn hierarchy(ch: char) -> u8 {
    match ch {
        '(' | ')' => 0,
        '|' => 1,
        '·' => 2,
        '*' => 3,
        _ => 4,
    }
}

fn is_operator(ch: char) -> bool {
    if ch == '|' || ch == '·' || ch == '*' {
        return true;
    }
    return false;
}

fn parse_regex(
    r: &str,
    op_stack: &mut VecDeque<char>,
    tr_stack: &mut VecDeque<Node<char>>,
) -> Node<char> {
    for ch in r.chars() {
        match ch {
            '(' => op_stack.push_back(ch),
            ')' => loop {
                build_tree(op_stack, tr_stack);
                let op = op_stack.back();
                if op.is_none() || *op.unwrap() == '(' {
                    op_stack.pop_back();
                    break;
                }
            },
            _ => {
                if is_operator(ch) {
                    while hierarchy(*op_stack.back().unwrap_or(&' ')) >= hierarchy(ch) {
                        build_tree(op_stack, tr_stack);
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
        build_tree(op_stack, tr_stack);
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

fn main() {
    let temp_test = "(a*·(a|b)·b*)";
    let mut op_stack: VecDeque<char> = VecDeque::new(); // operator stack
    let mut tr_stack: VecDeque<Node<char>> = VecDeque::new(); // tree stack
    let t = parse_regex(temp_test, &mut op_stack, &mut tr_stack);
    let mut result: Vec<char> = Vec::new();
    t.post_order(&mut result);
    println!("{:?}", result);
}
