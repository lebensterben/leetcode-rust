//! # Median of Two Sorted Arrays
//!
//! Given two sorted arrays `nums1` and `nums2` of size `m` and `n` respectively, return
//! **the median** of the two sorted arrays.
//!
//! **Follow up**: The overall run time complexity should be O(log (m+n)).
//!
//! Problem: https://leetcode.com/problems/median-of-two-sorted-arrays/

pub mod binary_search {
    /// This solution relies on mathematical properties to find the median.
    ///
    /// ## Motivation: Find Median via Partioning
    ///
    /// For any 0-indexed sorted array `X` of size `|X|`, the median is defined as:
    /// - If `|X|` is even, `m(X) ≡ (a[n] + a[n-1]) / 2`
    /// - If `|X|` is odd, `m(X) ≡ a[(|x|-1) / 2]`.
    ///
    /// Define the partition of `X` as `∃ i ∈ (0, |X|) s.t.
    /// - `X_left ≡ { x | x = X[i'] ∀ i' ≤ i}`
    /// - `X_right ≡ { x | x = X[i'] ∀ i' > i}`
    ///
    /// It follows that:
    /// - If `X` is even, `m(X) ≡ (max(X_left) + min(X_right)) / 2` and `i = |X| - i`
    /// - If `X` is odd, `m(X) ≡ min(X_right)` and `i = |X| - i + 1`
    ///
    /// To find the median of two sorted arrays `X` and `Y`, we need to ensure two identities,
    /// so that we can find the median via a paritioning `(i, j)` where `i ∈ (0, |X|)` and
    /// `j ∈ (0, |X|)`, and assume `|X| ≤ |Y|` for simplicity:
    /// - No number in left partitions can be larger than any number in right partitions:
    ///   - `max(X_left ∪ Y_left) ≤ min(X_right ∪ Y_right)`
    ///   - Since both arrays are sorted, it follows that `max(X[i-1], Y[j-1]) ≤ min(X[i], Y[j])`
    ///   - Since it is guaranteed to have `X[i-1] ≤ X[i]` and `Y[j-1] ≤ Y[j]`, this is equivalent
    ///     to `X[i-1] ≤ Y[j]` and `Y[j-1] ≤ X[i]`
    /// - When `|X| + |Y|` is even, `i + j = |X| + |Y| - i -j`; and when `|X| + |Y| is odd`,
    ///   `i + j = |X| + |Y| - i - j + 1`
    ///   - Equivalently, `j = (|X| + |Y|) / 2 - i` when it's even and `j = (|X| + |Y| + 1) / 2 - i`
    ///     when it's odd
    ///   - This can be simpliefied to `j = ⌊(|X| + |Y| + 1) / 2⌋ - i`
    ///
    /// To sum up, we need to find a partitioning s.t.
    /// - `X[i-1] ≤ Y[j]` and `Y[j-1] ≤ X[i]`
    /// - `j = ⌊(|X| + |Y| + 1) / 2⌋ - i`
    ///
    /// ### Search Loop
    ///
    /// Search loop is defined as below:
    /// - Define the search space of `i` as `[i_min, i_max]` and initialise it to `[0, |X|]`.
    /// - Use binary partioning, in each iteration, set `i` to `(i_min + i_max) / 2`, and therefore
    ///   set `j` to `⌊(|X| + |Y| + 1) / 2⌋ - i` to satisfy the second equality.
    ///   - If `X[i-1] ≤ Y[j]` and `Y[j-1] ≤ X[i]`, we've found the paritioning.
    ///   - If `X[i-1] > Y[j]`, we need to decrease `i` (and therefore increase `j`) to try to
    ///     decrease the LHS. That is, we need to update the search space as `[i_min, i - 1]`
    ///   - If `Y[j-1] > X[i]`, we need to increase `i`, i.e. to update the search space as
    ///     `[i + 1, i_max]`
    ///
    /// ### Edge Cases
    ///
    /// In the search loop, there are a few edge cases.
    /// - When `X[i-1] > Y[j]` and we need to decrease `i` but it becomes `0`:
    ///   - In last iteration `X[1] > Y[j(1)]`, where `j(1)` is the value of `j` when `i = 1`. And
    ///     then `X[1]` and `Y[j(1)]` are inserted to `X_right` and `Y_left` respectively.
    ///   - In this iteration, `max(X_left ∪ Y_left) = Y[j(1)]` and `min(X_right ∪ Y_right) = X[1]`.
    ///     It follows that `max(X_left ∪ Y_left) < min(X_right ∪ Y_right)`.
    ///   - So `i = 0` is the partitioning we want.
    /// - When `Y[j-1] > X[i]` and we need to increase `i` but it becomes `|X|`:
    ///   - In last iteration `Y[j(|X| - 1)] > x[|X| - 1]`. And then `Y[j(|X| - 1)]` and
    ///     `X[|X| - 1]` are inserted to `Y_right` and `X_left` respectively.
    ///   - In this iteration, `max(X_left ∪ Y_left) = X[|X - 1|]` and
    ///     `min(X_right ∪ Y_right) = Y[j(|X| - 1)]`. It follows that
    ///     `max(X_left ∪ Y_left) < min(X_right ∪ Y_right)`.
    ///   - So `i = |X|` is the partitioning we want.
    /// - For edge cases where `j` becomes either `0` or `|Y|`, the logic is similar.
    ///
    /// ## Solution
    ///
    /// To sum up, the algorithm is the following:
    ///
    /// 1. First make sure `|X| ≤ |Y|` and rename them if needed.
    /// 2. Define the search space of `i` as `[i_min, i_max]` and initialise it to `[0, |X|]`.
    /// 3. Use binary partioning, in each iteration, set `i` to `(i_min + i_max) / 2`, and set `j`
    ///    to `⌊(|X| + |Y| + 1) / 2⌋ - i`:
    ///    - If one of the following condition is met, we've found target partitioning:
    ///      - `X[i-1] ≤ Y[j]` and `Y[j-1] ≤ X[i]`
    ///      - `i = 0` or `i = |X|`
    ///    - If `X[i-1] > Y[j]`, update the search space as `[i_min, i - 1]` and continue.
    ///    - If `Y[j-1] > X[i]`, update the search space as `[i + 1, i_max]` and continue.
    /// 4. With target partitioning `i` and `j`:
    ///    - If `|X| + |Y|` is even, `m(X ∪ Y) = (max(X[i-1], Y[j-1]) + min(X[i], Y[j])) / 2`.
    ///    - If `|X| + |Y|` is odd, `m(X ∪ Y) = (max(X[i-1], Y[j-1])`.
    ///
    /// Time Complexity: `O(log(min(m, n)))`, similar to binary search.
    /// Space Complexity: `O(1)`, only a constant number of local variables are needed.
    ///
    /// ```
    /// # use leetcode_rust::algorithms::no4_median_of_two_sorted_arrays::binary_search::*;
    /// assert_eq!(
    ///     Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
    ///     2.0
    /// );
    /// assert_eq!(
    ///     Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
    ///     2.5
    /// );
    /// assert_eq!(
    ///     Solution::find_median_sorted_arrays(vec![0, 0], vec![0, 0]),
    ///     0.0
    /// );
    /// assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1.0);
    /// assert_eq!(Solution::find_median_sorted_arrays(vec![2], vec![]), 2.0);
    /// ```
    pub struct Solution {}

    impl Solution {
        pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
            // ensure `a` is never longer than `b` and m <= n
            let (a, b) = if nums1.len() <= nums2.len() {
                (&nums1, &nums2)
            } else {
                (&nums2, &nums1)
            };
            let (m, n) = (a.len(), b.len());
            let (mut i_min, mut i_max) = (0, m);

            while i_min <= i_max {
                let i = (i_min + i_max) / 2;
                let j = (m + n + 1) / 2 - i;

                if i != i_min && a[i - 1] > b[j] {
                    // i is too large
                    i_max = i - 1;
                } else if i != i_max && b[j - 1] > a[i] {
                    // i is too small
                    i_min = i + 1;
                } else {
                    // i is perfect
                    let max_left = if i == 0 {
                        b[j - 1]
                    } else if j == 0 {
                        a[i - 1]
                    } else {
                        a[i - 1].max(b[j - 1])
                    };

                    if (m + n) % 2 == 1 {
                        return max_left as f64;
                    }

                    let min_right = if i == m {
                        b[j]
                    } else if j == n {
                        a[i]
                    } else {
                        a[i].min(b[j])
                    };

                    return (max_left + min_right) as f64 / 2.0;
                }
            }
            unreachable!();
        }
    }
}

pub mod binary_search_plus {

    /// Same as the previous solution, but added a short-circuit section for empty input.
    ///
    /// Time Complexity: `O(log(min(m, n)))`.
    /// Space Complexity: `O(1)`.
    ///
    /// ```
    /// # use leetcode_rust::algorithms::no4_median_of_two_sorted_arrays::binary_search_plus::*;
    /// assert_eq!(
    ///     Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
    ///     2.0
    /// );
    /// assert_eq!(
    ///     Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
    ///     2.5
    /// );
    /// assert_eq!(
    ///     Solution::find_median_sorted_arrays(vec![0, 0], vec![0, 0]),
    ///     0.0
    /// );
    /// assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1.0);
    /// assert_eq!(Solution::find_median_sorted_arrays(vec![2], vec![]), 2.0);
    /// ```
    pub struct Solution {}

    impl Solution {
        pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
            // ensure `a` is never longer than `b` and m <= n
            let (a, b) = if nums1.len() <= nums2.len() {
                (&nums1, &nums2)
            } else {
                (&nums2, &nums1)
            };

            // short-circuit
            if a.is_empty() {
                let n = b.len();
                if n % 2 == 0 {
                    return (b[n / 2] + b[n / 2 - 1]) as f64 / 2.0;
                } else {
                    return b[n / 2] as f64;
                }
            };

            let (m, n) = (a.len(), b.len());
            let (mut i_min, mut i_max) = (0, m);

            while i_min <= i_max {
                let i = (i_min + i_max) / 2;
                let j = (m + n + 1) / 2 - i;

                if i != i_min && a[i - 1] > b[j] {
                    // i is too large
                    i_max = i - 1;
                } else if i != i_max && b[j - 1] > a[i] {
                    // i is too small
                    i_min = i + 1;
                } else {
                    // i is perfect
                    let max_left = if i == 0 {
                        b[j - 1]
                    } else if j == 0 {
                        a[i - 1]
                    } else {
                        a[i - 1].max(b[j - 1])
                    };

                    if (m + n) % 2 == 1 {
                        return max_left as f64;
                    }

                    let min_right = if i == m {
                        b[j]
                    } else if j == n {
                        a[i]
                    } else {
                        a[i].min(b[j])
                    };

                    return (max_left + min_right) as f64 / 2.0;
                }
            }
            unreachable!();
        }
    }
}
