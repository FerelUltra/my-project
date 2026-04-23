#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut curr = head;

    //[1,2,3,4,5]
    // 1) next - [2,3,4,5]
    // 2) curr = next
    // 3) prev = [1]
    //
    // [2,3,4,5]
    // 1) next - [3,4,5]
    // 2) curr = next
    // node = [2,]
    // 3 prev = [2]
    // а надо было, чтобы уже prev был [2,1]
    // while let Some(mut node) = curr {
    //     let next = node.next.take();
    //     curr = next;
    //     node.next = prev;
    //     prev = Some(node);
    // };

    while curr.is_some() {
        let mut node = curr.take().unwrap();
        let next = node.next.take();
        curr = next;
        node.next = prev;
        prev = Some(node);
    }
    prev
}

fn main() {}
