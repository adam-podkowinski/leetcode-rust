#![allow(dead_code)]
use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = vec![];
        let opening: HashMap<char, char> = HashMap::from([(']', '['), (')', '('), ('}', '{')]);
        for ch in s.chars() {
            if opening.values().any(|x| *x == ch) {
                stack.push(ch);
            } else {
                if stack.iter().last() == opening.get(&ch) {
                    stack.pop();
                } else {
                    return false;
                }
            }
        }
        stack.is_empty()
    }
}
