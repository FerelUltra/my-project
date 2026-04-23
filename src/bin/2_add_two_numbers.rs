#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut l1 = l1;
    let mut l2 = l2;

    let mut dummy = Box::new(ListNode::new(0));
    let mut tail = &mut dummy;
    let mut increment = 0;
    while l1.is_some() || l2.is_some() || increment != 0 {
        let mut v1 = 0;
        let mut v2 = 0;

        if let Some(mut node) = l1 {
            v1 = node.val;
            l1 = node.next.take();
        } else {
            l1 = None
        }

        if let Some(mut node) = l2 {
            v2 = node.val;
            l2 = node.next.take();
        } else {
            l2 = None;
        }

        let sum = v1 + v2 + increment;
        increment = sum / 10;
        tail.next = Some(Box::new(ListNode::new(sum % 10)));
        tail = tail.next.as_mut().unwrap();
    }
    dummy.next
}

fn main() {}
