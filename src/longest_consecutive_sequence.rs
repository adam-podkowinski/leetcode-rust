#![allow(dead_code)]
use crate::Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut sorted: Vec<i32> = nums;
        sorted.sort_unstable();
        sorted.dedup();

        let sorted_length = sorted.len();
        if sorted_length < 1 {
            return 0;
        }

        let mut longest = 1;
        let mut current_length = 1;

        for i in 0..sorted_length - 1 {
            if sorted[i] + 1 == sorted[i + 1] {
                current_length += 1;
                longest = current_length.max(longest);
            } else {
                current_length = 1;
            }
        }
        return longest;
    }
}
