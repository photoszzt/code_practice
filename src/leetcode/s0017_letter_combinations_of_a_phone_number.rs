/**
 * [17] Letter Combinations of a Phone Number
 *
 * Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent. Return the answer in any order.
 * A mapping of digit to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.
 * <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/7/73/Telephone-keypad2.svg/200px-Telephone-keypad2.svg.png" style="width: 200px; height: 162px;" />
 *
 * Example 1:
 *
 * Input: digits = "23"
 * Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
 *
 * Example 2:
 *
 * Input: digits = ""
 * Output: []
 *
 * Example 3:
 *
 * Input: digits = "2"
 * Output: ["a","b","c"]
 *
 *
 * Constraints:
 *
 * 	0 <= digits.length <= 4
 * 	digits[i] is a digit in the range ['2', '9'].
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/letter-combinations-of-a-phone-number/
// discuss: https://leetcode.com/problems/letter-combinations-of-a-phone-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        use std::collections::{HashMap, VecDeque};
        if digits.eq("") {
            return vec![];
        }
        let mut ret = vec![];
        let mut queue = VecDeque::new();

        let init = digits.chars().nth(0).unwrap();
        let mut mapping = HashMap::new();
        mapping.insert('2', vec!['a', 'b', 'c']);
        mapping.insert('3', vec!['d', 'e', 'f']);
        mapping.insert('4', vec!['g', 'h', 'i']);
        mapping.insert('5', vec!['j', 'k', 'l']);
        mapping.insert('6', vec!['m', 'n', 'o']);
        mapping.insert('7', vec!['p', 'q', 'r', 's']);
        mapping.insert('8', vec!['t', 'u', 'v']);
        mapping.insert('9', vec!['w', 'x', 'y', 'z']);
        for item in mapping.get(&init).unwrap().iter() {
            queue.push_back((item.to_string(), 0));
        }
        while !queue.is_empty() {
            let val = queue.pop_front().unwrap();
            if val.1 == digits.len() - 1 {
                ret.push(val.0.to_string());
            } else {
                let dig = digits.chars().nth(val.1 + 1).unwrap();
                for item in mapping.get(&dig).unwrap().iter() {
                    queue.push_back((format!("{}{}", val.0, item), val.1 + 1));
                }
            }
        }
        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_17() {
        assert_eq!(
            Solution::letter_combinations("23".to_string()),
            vec![
                "ad".to_string(),
                "ae".to_string(),
                "af".to_string(),
                "bd".to_string(),
                "be".to_string(),
                "bf".to_string(),
                "cd".to_string(),
                "ce".to_string(),
                "cf".to_string(),
            ]
        );
        assert_eq!(
            Solution::letter_combinations("2".to_string()),
            vec!["a".to_string(), "b".to_string(), "c".to_string(),]
        );
        assert_eq!(
            Solution::letter_combinations("".to_string()),
            Vec::<String>::new()
        );
    }
}
