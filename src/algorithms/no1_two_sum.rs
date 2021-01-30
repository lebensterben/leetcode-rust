//! Two Sum
//!
//! Given an array of integers `nums` and an integer `target`, return indices of the two numbers
//! such that that add up to `target`.
//!
//! You may assume that each input would have **exactly one solution**, and you may not use the same
//! element twice.
//!
//! Problem: https://leetcode.com/problems/two-sum/

pub mod brute_force {
    /// This solution uses nested loop and has `O(n^2)`complexity.
    ///
    /// Time complexity: `O(n^2)`.
    /// Space complexity: `O(1)`.
    /// ```
    /// # use leetcode_rust::algorithms::no1_two_sum::brute_force::*;
    /// assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    /// assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    /// assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    /// ```
    pub struct Solution {}

    impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            for i in 0..nums.len() {
                for j in (i + 1)..nums.len() {
                    if nums[j] == target - nums[i] {
                        return vec![i as i32, j as i32];
                    }
                }
            }
            unreachable!()
        }
    }
}

pub mod two_pass_hashmap {
    use std::collections::HashMap;

    /// This solution traverses `nums`, and stores complement of each input in a `HashMap`.
    /// On the second pass, if any of the input is required by another number, i.e. it's
    /// found in the complement map, the solution is found.
    ///
    /// The near-constant lookup of `HashMap` reduces the overall time complxity to `O(n)`.
    ///
    /// Time complexity: `O(n)`.
    /// Space complexity: `O(n)`.
    ///
    /// ```
    /// # use leetcode_rust::algorithms::no1_two_sum::two_pass_hashmap::*;
    /// assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    /// assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    /// assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    /// ```
    pub struct Solution {}

    impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            let complements: HashMap<i32, usize> = nums
                .iter()
                .enumerate()
                .map(|(i, x)| (target - x, i))
                .collect();
            for (j, y) in nums.iter().enumerate() {
                match complements.get_key_value(y) {
                    Some((_x, i)) if *i != j => return vec![j as i32, *i as i32],
                    _ => continue,
                }
            }
            unreachable!()
        }
    }
}

pub mod one_pass_hashmap {
    use std::collections::HashMap;

    /// This solution caches the complements required for each input, but it's done on-the-fly,
    /// i.e. it's done in one pass.
    ///
    /// Time complexity: `O(n)`.
    /// Space complexity: `O(n)`.
    ///
    /// ```
    /// # use leetcode_rust::algorithms::no1_two_sum::two_pass_hashmap::*;
    /// assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    /// assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    /// assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    /// ```
    pub struct Solution {}

    impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            let mut complements: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
            for (i, x) in nums.iter().enumerate() {
                match complements.get_key_value(x) {
                    Some((_y, j)) => return vec![*j as i32, i as i32],
                    None => {
                        complements.insert(target - x, i);
                    }
                }
            }
            unreachable!()
        }
    }
}
