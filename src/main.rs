// IS ANAGRAM?
// use std::collections::HashMap;

// fn count_chars(s: String) -> HashMap<char, i32> {
//     s.chars().fold(HashMap::<char, i32>::new(), |mut map, ch| {
//         map.entry(ch).and_modify(|count| *count += 1).or_insert(1);
//         map
//     })
// }

// impl Solution {
//     pub fn is_anagram(s: String, t: String) -> bool {
//         count_chars(s) == count_chars(t)
//     }
// }

//TWO SUM
// impl Solution {
//     pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//         for (index, ele) in nums.iter().enumerate() {
//             let left = target - ele;
//             if let Some(found_index) = nums.iter().position(|x| x == &left) {
//                 if found_index == index {
//                     continue;
//                 };
//                 return [index as i32, found_index as i32].to_vec();
//             }
//         }
//         return [-1, -1].to_vec();
//     }
// }

// Group Anagrams
// TODO: Dont use HashSet, use only one HashMap
use std::collections::{HashMap, HashSet};

fn is_anagram(s: String, t: String) -> bool {
    let counted_s = s.chars().fold(HashMap::<char, i32>::new(), |mut map, ch| {
        map.entry(ch).and_modify(|count| *count += 1).or_insert(1);
        map
    });
    let counted_t = t.chars().fold(HashMap::<char, i32>::new(), |mut map, ch| {
        map.entry(ch).and_modify(|count| *count += 1).or_insert(1);
        map
    });
    counted_s == counted_t
}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let length = strs.len();
        let mut all: Vec<Vec<String>> = Vec::new();
        let mut exist: HashSet<String> = HashSet::with_capacity(length);

        for (i, ele) in strs.iter().enumerate() {
            if exist.contains(ele) {
                continue;
            }
            if i == length - 1 {
                all.push(vec![ele.to_owned()]);
                break;
            }
            exist.insert(ele.to_owned());
            let mut anagrams = vec![ele.to_owned()];
            let left = &strs[i + 1..];
            for check_me in left {
                if is_anagram(check_me.to_string(), ele.to_string()) {
                    anagrams.push(check_me.to_owned());
                    exist.insert(check_me.to_owned());
                }
            }
            all.push(anagrams);
        }
        return all;
    }
}

fn main() {
    // Group anagrams
    println!(
        "{:?}",
        Solution::group_anagrams(
            ["b", "", "", "", "a", "a"]
                .map(|x| String::from(x))
                .to_vec()
        )
    );
    // TWO SUM
    // println!("{:?}", Solution::two_sum([3, 3, 11, 15].to_vec(), 6))
    // IS ANAGRAM?
    // println!(
    //     "{}",
    //     Solution::is_anagram("abc".to_owned(), "cba".to_owned())
    // );
}

struct Solution {}
