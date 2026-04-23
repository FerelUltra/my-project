#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: None });

    let mut tail = &mut dummy;

    let mut curr = head;
	let mut has_real_node = false;
    while let Some(mut node) = curr {
		
        if node.val == tail.val && has_real_node {
            curr = node.next;
        } else {
			has_real_node = true;
			let node_tail = node.next.take();
			tail.next = Some(node);
			tail = tail.next.as_mut().unwrap();

			curr = node_tail
		}
    }

    dummy.next
}

fn main() {}
