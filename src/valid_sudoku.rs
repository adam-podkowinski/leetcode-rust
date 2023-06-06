#![allow(dead_code)]

use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut col_sets: Vec<HashSet<u32>> = vec![HashSet::with_capacity(9); 9];
        let mut row_sets: Vec<HashSet<u32>> = vec![HashSet::with_capacity(9); 9];
        let mut block_set: HashSet<u32> = HashSet::with_capacity(9);

        // Check row and column rule
        for (i, row) in board.iter().enumerate() {
            for (j, &ch) in row.iter().enumerate() {
                if ch == '.' {
                    continue;
                }
                let num = ch.to_digit(10).unwrap();
                if !row_sets[i].insert(num) {
                    return false;
                }
                if !col_sets[j].insert(num) {
                    return false;
                }
            }
        }
        // Check 3x3 block rule
        for i in 0..3 {
            let three_rows = &board[i * 3..i * 3 + 3];
            for j in 0..3 {
                let block: Vec<_> = three_rows.iter().map(|x| &x[j * 3..j * 3 + 3]).collect();
                let block: Vec<u32> = block
                    .concat()
                    .iter()
                    .filter_map(|x| if *x == '.' { None } else { x.to_digit(10) })
                    .collect();
                for n in block {
                    if !block_set.insert(n) {
                        return false;
                    }
                }
                block_set.clear();
            }
        }
        true
    }
}
