// Definition for singly-linked list.
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

pub struct Solution; //unit struct

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (None, l2) => l2,
            (l1, None) => l1,
            (Some(mut l1), Some(mut l2)) => {
                if l1.val < l2.val {
                    l1.next = Solution::merge_two_lists(l1.next, Some(l2));
                    return Some(l1);
                } else {
                    l2.next = Solution::merge_two_lists(Some(l1), l2.next);
                    return Some(l2);
                }
            }
        }
    }
}

fn main() {
    let l1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode::new(4))),
        })),
    }));
    let l2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode::new(4))),
        })),
    }));

    let result = Solution::merge_two_lists(l1, l2);
    println!("{result:#?}");
}
