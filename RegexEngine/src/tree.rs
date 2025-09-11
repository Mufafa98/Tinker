#[derive(Debug, Clone)]
pub struct Node<T> {
    pub value: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}

impl<T: Clone> Node<T> {
    pub fn post_order(&self, result: &mut Vec<T>) {
        if let Some(ref left) = self.left {
            left.post_order(result);
        }
        if let Some(ref right) = self.right {
            right.post_order(result);
        }
        result.push(self.value.clone());
    }
}

impl Node<char> {
    pub fn with_lr(value: char, left: Node<char>, right: Node<char>) -> Node<char> {
        return Node {
            value,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        };
    }
    pub fn with_l(value: char, left: Node<char>) -> Node<char> {
        return Node {
            value,
            left: Some(Box::new(left)),
            right: None,
        };
    }
}
