#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut curr = head.as_ref();
    let mut len = 0;
    while let Some(node) = curr {
        len += 1;
        curr = node.next.as_ref();
    }

	let target = len - n;
	let mut dummy = Box::new(ListNode{
		val: 0,
		next: head
	});
	let mut curr = &mut dummy;

	for _ in 0..target {
		curr = curr.next.as_mut().unwrap();
	}

	let next_next = curr.next.as_mut().unwrap().next.take();
	curr.next = next_next;

    dummy.next
}

fn main() {}
