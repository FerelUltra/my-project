#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn main() {}

pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut head = head;

    let mut curr_as_ref = head.as_ref();
    let mut len: usize = 0;
    while let Some(node) = curr_as_ref {
        len += 1;
        curr_as_ref = node.next.as_ref();
    }

	if len == 0 {
		return head;
	}

	let k = k as usize % len;

	if k == 0 {
		return head
	}

    let last_not_rotate_element_index = len - k - 1;

    let mut curr = head.as_mut();

    for _ in 0..last_not_rotate_element_index {
        curr = curr.unwrap().next.as_mut();
    }

	let mut new_head = curr.unwrap().next.take();

	let mut tail = new_head.as_mut().unwrap();

	while tail.next.is_some() {
		tail = tail.next.as_mut().unwrap();
	}

	tail.next = head;

    new_head
}
