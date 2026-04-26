use std::collections::HashSet;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;

		let mut set: HashSet<i32> = HashSet::new();

		let mut head_as_ref = head.as_ref();

		while let Some(node) = head_as_ref{
			if let Some(next) = node.next.as_ref(){
				if let Some(next_next) = next.next.as_ref(){
					if node.val == next_next.val{
						
					}
				}
			}
			head_as_ref = node.next.as_ref();
		}

		head
    }

fn main() {}
