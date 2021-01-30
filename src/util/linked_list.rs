#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    /// ```
    /// # use leetcode_rust::util::linked_list::*;
    /// assert_eq!(ListNode::new(42), ListNode{val: 42, next: None});
    /// ```
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LinkedList(pub Option<Box<ListNode>>);

impl LinkedList {
    #[inline]
    pub fn new() -> Self {
        Self(None)
    }

    /// ```
    /// # use leetcode_rust::linked_list;
    /// # use leetcode_rust::util::linked_list::*;
    /// let mut l1 = linked_list![];
    /// l1.push_front(43);
    /// assert_eq!(l1, linked_list![43]);
    ///
    /// let mut l2 = linked_list![43, 44];
    /// l2.push_front(42);
    /// assert_eq!(l2, linked_list![42, 43, 44]);
    /// ```
    #[inline]
    pub fn push_front(&mut self, elt: i32) {
        let node = ListNode {
            val: elt,
            next: self.0.take(),
        };
        *self = Self::from(node)
    }
    /// ```
    /// # use leetcode_rust::linked_list;
    /// # use leetcode_rust::util::linked_list::*;
    /// let mut l1 = linked_list![];
    /// assert_eq!(l1.pop_front(), None);
    /// assert_eq!(l1, linked_list![]);
    ///
    /// let mut l2 = linked_list![42];
    /// l2.push_front(43);
    /// assert_eq!(l2.pop_front(), Some(43));
    /// assert_eq!(l2, linked_list![42]);
    /// ```
    #[inline]
    pub fn pop_front(&mut self) -> Option<i32> {
        self.0.take().map(|l| {
            let cur_val: i32 = l.val;
            if let Some(next_node) = l.next {
                self.0.replace(next_node);
            } else {
                *self = Self(None);
            }
            cur_val
        })
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_some()
    }
}

impl From<ListNode> for LinkedList {
    #[inline]
    fn from(node: ListNode) -> Self {
        Self(Some(Box::new(node)))
    }
}

impl From<Option<Box<ListNode>>> for LinkedList {
    #[inline]
    fn from(node: Option<Box<ListNode>>) -> Self {
        Self(node)
    }
}

impl From<&[i32]> for LinkedList {
    #[inline]
    fn from(vals: &[i32]) -> Self {
        let mut list = LinkedList::new();
        vals.into_iter().rev().for_each(|&elt| list.push_front(elt));
        list
    }
}

#[macro_export]
macro_rules! linked_list {
    ($($e:expr), *) => {
        $crate::util::LinkedList::from([$($e), *].as_ref())
    };
}

#[macro_export]
macro_rules! list_node {
    ($($e:expr), *) => {
        $crate::util::LinkedList::from([$($e), *].as_ref()).0
    };
}
