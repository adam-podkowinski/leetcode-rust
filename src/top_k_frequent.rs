#![allow(dead_code)]
use std::collections::HashMap;
use crate::Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut num_map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        for ele in nums.iter() {
            num_map.entry(*ele).and_modify(|x| *x += 1).or_insert(1);
        }
        let mut list: Vec<(&i32, &i32)> = num_map.iter().collect();
        list.sort_by(|x, y| y.1.cmp(x.1));
        let answer: &[(&i32, &i32)] = &list[..k as usize];
        return answer.iter().map(|x| x.0.to_owned()).collect();
    }
}
