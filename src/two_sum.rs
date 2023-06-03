#![allow(dead_code)]
use crate::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (index, ele) in nums.iter().enumerate() {
            let left = target - ele;
            if let Some(found_index) = nums.iter().position(|x| x == &left) {
                if found_index == index {
                    continue;
                };
                return [index as i32, found_index as i32].to_vec();
            }
        }
        [-1, -1].to_vec()
    }
}
