#![allow(dead_code)]
use std::collections::HashMap;
use crate::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagrams: HashMap<Vec<char>, Vec<String>> = HashMap::with_capacity(30);
        for str in strs {
            let mut sorted: Vec<char> = str.chars().collect();
            sorted.sort();
            anagrams
                .entry(sorted)
                .and_modify(|x| x.push(str.to_string()))
                .or_insert([str].to_vec());
        }
        return anagrams.values().cloned().collect();
    }
}
