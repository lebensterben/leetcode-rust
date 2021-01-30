//! # Longest Substring without Repeating Characters
//!
//! Given a string `s`, find the length of the longest substring without repeating characters.
//!
//! problem: https://leetcode.com/problems/longest-substring-without-repeating-characters/

pub mod brute_force {
    use crate::{byte_string, util::ByteString};
    use std::collections::HashSet;

    /// This solution checks all substrings of `s`, and uses a `HashSet` to cache characteres
    /// that have been seen in each iteration.
    ///
    /// Time Complexity: `O(n^3)`.
    /// Space Complexity: `O(min(n, m))` where `m` is the size of alphabet.
    ///
    /// ```
    /// # use leetcode_rust::algorithms::no3_longest_substring_without_repeating_characters::brute_force::*;
    /// assert_eq!(Solution::length_of_longest_substring(String::from("abcabcbb")), 3);
    /// assert_eq!(Solution::length_of_longest_substring(String::from("bbbbb")), 1);
    /// assert_eq!(Solution::length_of_longest_substring(String::from("pwwkew")), 3);
    /// ```
    pub struct Solution {}

    impl Solution {
        pub fn length_of_longest_substring(s: String) -> i32 {
            let s = byte_string!(s.as_ref());
            let length = s.len();
            let (mut start, mut ans) = (0, 0);
            let mut end;

            while start < length {
                end = start + 1;
                while end <= length {
                    if Solution::all_unique(&s, start, end) {
                        ans = ans.max(end - start);
                    }
                    end += 1;
                }
                start += 1;
            }

            ans as i32
        }

        pub fn all_unique(s: &ByteString, start: usize, end: usize) -> bool {
            let mut char_set = HashSet::<u8>::new();

            for i in start..end {
                let ch = s[i];
                if char_set.contains(&ch) {
                    return false;
                }
                char_set.insert(ch);
            }
            return true;
        }
    }
}

pub mod sliding_window {
    use crate::byte_string;
    use std::collections::HashSet;

    /// This solution uses a sliding window which starts from the 0-th index and has size 1.
    /// When the next character is not duplicate, it increase the right boundary and keep searching.
    /// Otherwise, it pop out the first character and increment the left boundary.
    ///
    /// Time Complexity: `O(n)`. In worst case both boundaries of window traverse the entire range,
    /// so it's `O(2n)`.
    /// Space Complexity: `O(min(m, n))`, where `m` is the size of alphabet.
    ///
    /// ```
    /// # use leetcode_rust::algorithms::no3_longest_substring_without_repeating_characters::sliding_window::*;
    /// assert_eq!(Solution::length_of_longest_substring(String::from("abcabcbb")), 3);
    /// assert_eq!(Solution::length_of_longest_substring(String::from("bbbbb")), 1);
    /// assert_eq!(Solution::length_of_longest_substring(String::from("pwwkew")), 3);
    /// ```
    pub struct Solution {}

    impl Solution {
        pub fn length_of_longest_substring(s: String) -> i32 {
            let s = byte_string!(s.as_ref());
            let length = s.len();
            let (mut left, mut right, mut ans) = (0, 0, 0);
            let mut char_set = HashSet::<u8>::new();

            while left < length && right < length {
                if !char_set.contains(&s[right]) {
                    char_set.insert(s[right]);
                    right += 1;
                    ans = ans.max(right - left);
                } else {
                    char_set.remove(&s[left]);
                    left += 1;
                }
            }

            ans as i32
        }
    }
}

pub mod sliding_window_plus {
    use crate::byte_string;
    use std::collections::HashMap;

    /// This solution is based on the previous one but does a better job.
    ///
    /// The right boundary of the window moves one index per iteration. And it uses a `HashMap` to
    /// store each character and its last seen position (plus one).
    /// When a duplicated character is seen, the left boundary jumps to the value cached, and all
    /// other characters in between are skipped. They can be safely skipped because they belong to
    /// a substring that HAS duplicates, which is opposite to what we want.
    ///
    /// Time Complexity: `O(n)`, since right boundary moves one index per iteration.
    /// Space Complexity: `O(min(m, n))` where `m` is the size of alphabet.
    ///
    /// ```
    /// # use leetcode_rust::algorithms::no3_longest_substring_without_repeating_characters::sliding_window_plus::*;
    /// assert_eq!(Solution::length_of_longest_substring(String::from("abcabcbb")), 3);
    /// assert_eq!(Solution::length_of_longest_substring(String::from("bbbbb")), 1);
    /// assert_eq!(Solution::length_of_longest_substring(String::from("pwwkew")), 3);
    /// ```
    pub struct Solution {}

    impl Solution {
        pub fn length_of_longest_substring(s: String) -> i32 {
            let s = byte_string!(s.as_ref());
            let length = s.len();
            let (mut left, mut right, mut ans) = (0, 0, 0);
            let mut char_map = HashMap::<u8, usize>::new();

            while right < length {
                if let Some((_char, index)) = char_map.get_key_value(&s[right]) {
                    left = left.max(*index);
                }
                ans = ans.max(right - left + 1);
                char_map.insert(s[right], right + 1);

                right += 1;
            }

            ans as i32
        }
    }
}

pub mod sliding_window_plus_alt {
    use crate::byte_string;

    /// This solution is slightly different from the previous one. It assumes the size of alphabet.
    ///
    /// Time Complexity: `O(n)`.
    /// Space Complexity: `O(m)` where `m` is the size of alphabet.
    ///
    /// ```
    /// # use leetcode_rust::algorithms::no3_longest_substring_without_repeating_characters::sliding_window_plus_alt::*;
    /// assert_eq!(Solution::length_of_longest_substring(String::from("abcabcbb")), 3);
    /// assert_eq!(Solution::length_of_longest_substring(String::from("bbbbb")), 1);
    /// assert_eq!(Solution::length_of_longest_substring(String::from("pwwkew")), 3);
    /// ```
    pub struct Solution {}

    impl Solution {
        pub fn length_of_longest_substring(s: String) -> i32 {
            let s = byte_string!(s.as_ref());
            let length = s.len();
            let (mut left, mut right, mut ans) = (0, 0, 0);
            let mut char_table = [0usize; 256];

            while right < length {
                left = left.max(char_table[s[right] as usize]);
                ans = ans.max(right - left + 1);
                char_table[s[right] as usize] = right + 1;

                right += 1;
            }

            ans as i32
        }
    }
}
