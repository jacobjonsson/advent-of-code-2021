use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    iter::Peekable,
    ops::{Add, Deref},
    rc::{Rc, Weak},
    str::Chars,
};

use itertools::Itertools;

pub fn run() {
    let input = include_str!("./input.txt").trim();

    println!("[DAY 18] Part 1: {}", part_1(&input));
    println!("[DAY 18] Part 2: {}", part_2(&input));
}

#[derive(Default, Clone, PartialEq)]
struct NodeRef(Rc<RefCell<Node>>);

impl NodeRef {
    fn value(&self) -> Option<usize> {
        self.0.deref().borrow().value
    }

    fn depth(&self) -> usize {
        let mut parent = self.parent();
        let mut count = 0;

        while let Some(p) = parent {
            count += 1;
            parent = p.parent();
        }

        count
    }

    fn left(&self) -> Option<NodeRef> {
        self.0.deref().borrow().left.clone()
    }

    fn right(&self) -> Option<NodeRef> {
        self.0.deref().borrow().right.clone()
    }

    fn parent(&self) -> Option<NodeRef> {
        self.0
            .deref()
            .borrow()
            .parent
            .clone()
            .map(|p| p.upgrade())
            .flatten()
    }

    fn increment_depth(&mut self) {
        if let Some(mut left) = self.left() {
            left.increment_depth();
        }

        if let Some(mut right) = self.right() {
            right.increment_depth();
        }
    }

    fn set_left(&mut self, node: NodeRef) {
        self.0.borrow_mut().left = Some(node);
    }

    fn set_right(&mut self, node: NodeRef) {
        self.0.borrow_mut().right = Some(node);
    }

    fn set_value(&mut self, value: usize) {
        self.0.borrow_mut().value = Some(value);
    }

    fn set_parent(&mut self, parent: ParentRef) {
        self.0.borrow_mut().parent = Some(parent);
    }

    fn downgrade(&self) -> ParentRef {
        ParentRef(Rc::downgrade(&self.0))
    }

    fn magnitude(&self) -> usize {
        let node = self.0.deref().borrow();
        if let Some(value) = node.value {
            value
        } else {
            node.left.as_ref().unwrap().magnitude() * 3
                + node.right.as_ref().unwrap().magnitude() * 2
        }
    }

    fn left_most_value(&self) -> NodeRef {
        if let Some(_) = self.value() {
            self.clone()
        } else {
            self.left().unwrap().left_most_value()
        }
    }

    fn right_most_value(&self) -> NodeRef {
        if let Some(_) = self.value() {
            self.clone()
        } else {
            self.right().unwrap().right_most_value()
        }
    }

    fn first_left_adj(&self) -> Option<NodeRef> {
        let parent = if let Some(parent) = self.parent() {
            parent
        } else {
            return None;
        };

        let left = if let Some(left) = parent.left() {
            left
        } else {
            return None;
        };

        if Rc::ptr_eq(&self.0, &left.0) {
            return parent.first_left_adj();
        } else {
            return Some(left.right_most_value());
        }
    }

    fn first_right_adj(&self) -> Option<NodeRef> {
        let parent = if let Some(parent) = self.parent() {
            parent
        } else {
            return None;
        };

        let right = if let Some(right) = parent.right() {
            right
        } else {
            return None;
        };

        if Rc::ptr_eq(&self.0, &right.0) {
            return parent.first_right_adj();
        } else {
            return Some(right.left_most_value());
        }
    }

    fn is_pair(&self) -> bool {
        self.left().unwrap().value().is_some() && self.right().unwrap().value().is_some()
    }

    fn next_to_explode(&self) -> Option<NodeRef> {
        // We can't explode a node with a value
        if self.value().is_some() {
            return None;
        }

        // Depth 4 is the node to explode
        if self.depth() >= 4 && self.is_pair() {
            return Some(self.clone());
        }

        let left = self.left().unwrap();
        let right = self.right().unwrap();

        left.next_to_explode().or_else(|| right.next_to_explode())
    }

    fn next_to_split(&self) -> Option<NodeRef> {
        if let Some(value) = self.value() {
            if value >= 10 {
                return Some(self.clone());
            } else {
                return None;
            }
        }

        let left = self.left().unwrap();
        let right = self.right().unwrap();

        left.next_to_split().or_else(|| right.next_to_split())
    }

    fn reduce(&mut self) {
        loop {
            if let Some(mut to_explode) = self.next_to_explode() {
                to_explode.explode();
                continue;
            }

            if let Some(mut to_split) = self.next_to_split() {
                to_split.split();
                continue;
            }

            break;
        }
    }

    fn explode(&mut self) {
        let left_adj = self.first_left_adj();
        let right_adj = self.first_right_adj();

        if let Some(mut left) = left_adj {
            left.set_value(left.value().unwrap() + &self.left().unwrap().value().unwrap())
        }

        if let Some(mut right) = right_adj {
            right.set_value(right.value().unwrap() + &self.right().unwrap().value().unwrap())
        }

        let parent = self.parent().unwrap();
        let left = parent.left().unwrap();
        let right = parent.right().unwrap();

        let mut new_node = NodeRef::default();
        new_node.set_value(0);
        new_node.set_parent(parent.downgrade());

        if Rc::ptr_eq(&left.0, &self.0) {
            parent.0.deref().borrow_mut().left = Some(new_node);
        } else if Rc::ptr_eq(&right.0, &self.0) {
            parent.0.deref().borrow_mut().right = Some(new_node);
        }

        self.0.deref().borrow_mut().parent = None;
    }

    fn split(&mut self) {
        let mut new_left = NodeRef::default();
        new_left.set_value((self.value().unwrap() as f64 / 2 as f64).floor() as usize);
        new_left.set_parent(self.downgrade());
        self.set_left(new_left);

        let mut new_right = NodeRef::default();
        new_right.set_value((self.value().unwrap() as f64 / 2 as f64).ceil() as usize);
        new_right.set_parent(self.downgrade());
        self.set_right(new_right);

        self.0.deref().borrow_mut().value = None;
    }
}

#[derive(Debug, Clone)]
struct ParentRef(Weak<RefCell<Node>>);

impl PartialEq for ParentRef {
    fn eq(&self, other: &Self) -> bool {
        self.upgrade().eq(&other.upgrade())
    }
}

impl ParentRef {
    pub fn upgrade(&self) -> Option<NodeRef> {
        self.0.upgrade().map(|p| NodeRef(p))
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
struct Node {
    depth: usize,
    value: Option<usize>,
    left: Option<NodeRef>,
    right: Option<NodeRef>,
    parent: Option<ParentRef>,
}

impl Debug for NodeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for NodeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let node = &self.0.borrow_mut();
        if let Some(value) = node.value {
            write!(f, "{}", value)
        } else {
            write!(
                f,
                "[{},{}]",
                node.left.as_ref().unwrap(),
                node.right.as_ref().unwrap()
            )
        }
    }
}

impl Add for NodeRef {
    type Output = NodeRef;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        let mut parent = NodeRef::default();

        self.increment_depth();
        rhs.increment_depth();

        self.set_parent(parent.downgrade());
        rhs.set_parent(parent.downgrade());

        parent.set_left(self);
        parent.set_right(rhs);

        parent.reduce();

        parent
    }
}

fn parse(chars: &mut Peekable<Chars>) -> NodeRef {
    let mut node = NodeRef::default();

    assert_eq!(chars.next(), Some('['));
    node.set_left(match chars.peek() {
        Some('[') => {
            let mut inner = parse(chars);
            inner.set_parent(node.downgrade());
            inner
        }

        _ => {
            let mut inner = NodeRef::default();
            let mut value = String::new();
            while chars.peek().unwrap().is_digit(10) {
                value.push(chars.next().unwrap());
            }
            inner.set_value(value.parse().unwrap());
            inner.set_parent(node.downgrade());
            inner
        }
    });

    assert_eq!(chars.next(), Some(','));

    node.set_right(match chars.peek() {
        Some('[') => {
            let mut inner = parse(chars);
            inner.set_parent(node.downgrade());
            inner
        }

        _ => {
            let mut inner = NodeRef::default();
            let mut value = String::new();
            while chars.peek().unwrap().is_digit(10) {
                value.push(chars.next().unwrap());
            }
            inner.set_value(value.parse().unwrap());
            inner.set_parent(node.downgrade());
            inner
        }
    });

    assert_eq!(chars.next(), Some(']'));

    node
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| parse(&mut line.chars().peekable()))
        .reduce(|a, b| a + b)
        .unwrap()
        .magnitude()
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .permutations(2)
        .map(|mut per| (per.remove(0), per.remove(0)))
        .map(|(a, b)| {
            let a = parse(&mut a.chars().peekable());
            let b = parse(&mut b.chars().peekable());

            (a + b).magnitude()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 4140);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 3993);
    }
}
