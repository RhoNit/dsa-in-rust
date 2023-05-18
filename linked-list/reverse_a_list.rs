// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current_node = head;
        let mut prev_node = None;

        while let Some(mut node) = current_node {
            let mut next_node = node.next;
            node.next = prev_node;
            prev_node = Some(node);
            current_node = next_node;
        }

        prev_node
    }
}