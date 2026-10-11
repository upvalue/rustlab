use std::ops::Deref;

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

/**
 * Make a singly linked list (helper)
 */
pub fn make_sl(values: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;

    for &val in values.iter().rev() {
        head = Some(Box::new(ListNode { val, next: head }));
    }

    head
}

pub fn sl_to_vec(ls: Option<Box<ListNode>>) -> Vec<i32> {
    let mut v: Vec<i32> = vec![];
    let mut lp = ls.as_ref();

    while let Some(node) = lp {
        v.push(node.val);
        lp = node.next.as_ref();
    }

    v
}
