mod group_anagrams;
mod is_anagram;
mod product_of_array_except_self;
mod top_k_frequent;
mod two_sum;
mod valid_sudoku;

fn main() {
    // Valid sudoku
    println!(
        "{:?}",
        Solution::is_valid_sudoku(
            [
                ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                ['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]
            .map(|x| x.to_vec())
            .to_vec()
        )
    )
    // Product of Array except self
    // println!("{:?}", Solution::product_except_self(vec![1, 2, 3, 4]));
    // Tok K frequent
    // println!("{:?}", Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2));
    // Group anagrams
    // println!(
    //     "{:?}",
    //     Solution::group_anagrams(
    //         ["b", "abc", "", "", "a", "a"]
    //             .map(|x| String::from(x))
    //             .to_vec()
    //     )
    // );
    // TWO SUM
    // println!("{:?}", Solution::two_sum([3, 3, 11, 15].to_vec(), 6));
    // IS ANAGRAM?
    // println!(
    //     "{}",
    //     Solution::is_anagram("abc".to_owned(), "cba".to_owned())
    // );
}

#[allow(dead_code)]
struct Solution {}
