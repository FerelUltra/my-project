#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn main() {}

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
 

	let mut dummy = Box::new(ListNode{
		val: 0,
		next: head
	});

	let mut tail = &mut dummy;

	while tail.next.is_some() && tail.next.as_ref().unwrap().next.is_some() {
		let mut first = tail.next.take().unwrap();
		let mut second = first.next.take().unwrap();
		let rest = second.next.take();

		first.next = rest;
		second.next = Some(first);
		tail.next = Some(second);

		tail = tail.next.as_mut().unwrap().next.as_mut().unwrap();
	}

	dummy.next
}
