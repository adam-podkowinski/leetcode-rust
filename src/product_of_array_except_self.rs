#![allow(dead_code)]
use crate::Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut answer = vec![];
        for i in 0..nums.len() {
            let left: i32 = nums[..i].iter().product();
            let right: i32 = nums[i + 1..].iter().product();
            let product: i32 = left * right;
            answer.push(product);
        }
        answer
    }
}
