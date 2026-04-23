use std::cell::{Ref, RefCell};
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
struct Node {
    val: i32,
    next: Link,
}

impl Node {
    fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { val, next: None }))
    }
}

fn has_cycle(head: Link) -> bool {
    let mut slow = head.clone();
    let mut fast = head;

    loop {
        slow = match slow {
            Some(node) => node.borrow().next.clone(),
            None => return false,
        };

        fast = match fast {
            Some(node) => {
                let next1 = node.borrow().next.clone();
                match next1 {
                    Some(node2) => node2.borrow().next.clone(),
                    None => return false,
                }
            }
            None => return false,
        };

        match (&slow, &fast) {
            (Some(s), Some(f)) if Rc::ptr_eq(s, f) => return true,
            (None, _) | (_, None) => return false,
            _ => {}
        }
    }
}

fn build_list(values: &[i32], pos: Option<usize>) -> Link {
    if values.is_empty() {
        return None;
    }

    let nodes: Vec<Rc<RefCell<Node>>> = values.iter().map(|&v| Node::new(v)).collect();

    for i in 0..nodes.len() - 1 {
        nodes[i].borrow_mut().next = Some(nodes[i + 1].clone());
    }

    if let Some(p) = pos {
        assert!(p < nodes.len(), "cycle position out of range");
        let tail = nodes.last().unwrap().clone();
        tail.borrow_mut().next = Some(nodes[p].clone());
    }

    Some(nodes[0].clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_cycle_to_index_1() {
        let head = build_list(&[3, 2, 0, -4], Some(1));
        assert!(has_cycle(head));
    }

    #[test]
    fn example_2_cycle_to_index_0() {
        let head = build_list(&[1, 2], Some(0));
        assert!(has_cycle(head))
    }

    #[test]
    fn example_3_no_cycle_single_node() {
        let head = build_list(&[1], None);
        assert!(!has_cycle(head));
    }

    #[test]
    fn empty_list() {
        let head = build_list(&[], None);
        assert!(!has_cycle(head));
    }

    #[test]
    fn one_node_self_cycle() {
        let head = build_list(&[42], Some(0));
        assert!(has_cycle(head))
    }

    #[test]
    fn long_list_no_cycle() {
        let head = build_list(&[1, 2, 3, 4, 5, 6, 7], None);
        assert!(!has_cycle(head));
    }

    #[test]
    fn cycle_in_middle() {
        let head = build_list(&[10, 20, 30, 40, 50], Some(2));
        assert!(has_cycle(head))
    }
}

fn main() {}
