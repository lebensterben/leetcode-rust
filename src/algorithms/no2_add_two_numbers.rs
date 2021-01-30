//! # Add Two Numbers
//!
//! You are given two non-empty linked lists representing two non-negative integers. The digits are
//! stored in reverse order, and each of their nodes contains a single digit. Add the two numbers
//! and return the sum as a linked list.
//!
//! You may assume the two numbers do not contain any leading zero, except the number 0 itself.
//!
//! Problem: https://leetcode.com/problems/add-two-numbers/

use crate::{
    linked_list, list_node,
    util::{LinkedList, ListNode},
};

/// This solution is based on the *Elementary Math* approach.
///
/// Time Complexity: `O(max(m, n))`
/// Space Complexity: `O(max(m, n))`
pub struct Solution {}

impl Solution {
    /// ```
    /// # use leetcode_rust::list_node;
    /// # use leetcode_rust::algorithms::no2_add_two_numbers::*;
    /// assert_eq!(
    ///     Solution::add_two_numbers(list_node![2, 4, 3], list_node![5, 6, 4]),
    ///     list_node![7, 0, 8]
    /// );
    /// ```
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut l1, mut l2, mut l3) = (LinkedList::from(l1), LinkedList::from(l2), linked_list![]);
        let mut tail = &mut l3.0;
        let (mut x, mut y, mut sum): (i32, i32, i32);
        let mut carry = 0;

        while l1.is_empty() || l2.is_empty() {
            x = l1.pop_front().unwrap_or(0);
            y = l2.pop_front().unwrap_or(0);
            sum = carry + x + y;

            *tail = list_node![sum % 10];
            tail = &mut tail.as_mut().unwrap().next;

            carry = sum / 10;
        }

        if carry > 0 {
            *tail = list_node![carry % 10];
        }

        l3.0
    }
}
