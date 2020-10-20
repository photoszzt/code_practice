/**
 * [647] Palindromic Substrings
 *
 * Given a string, your task is to count how many palindromic substrings in this string.
 *
 * The substrings with different start indexes or end indexes are counted as different substrings even they consist of same characters.
 *
 * Example 1:
 *
 *
 * Input: "abc"
 * Output: 3
 * Explanation: Three palindromic strings: "a", "b", "c".
 *
 *
 *
 *
 * Example 2:
 *
 *
 * Input: "aaa"
 * Output: 6
 * Explanation: Six palindromic strings: "a", "a", "a", "aa", "aa", "aaa".
 *
 *
 *
 *
 * Note:
 *
 * <ol>
 * 	The input string length won't exceed 1000.
 * </ol>
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/palindromic-substrings/
// discuss: https://leetcode.com/problems/palindromic-substrings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();
        if len <= 1 {
            return s.len() as i32;
        }
        let mut state = vec![vec![false; len]; len];
        let mut count = 0;
        for j in 0..len {
            for i in 0..=j {
                if chars[i] == chars[j] {
                    if j - i < 3 {
                        state[i][j] = true;
                        count += 1;
                    } else {
                        state[i][j] = state[i + 1][j - 1];
                        if state[i + 1][j - 1] {
                            count += 1;
                        }
                    }
                }
            }
        }
        count as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_647() {
        assert_eq!(Solution::count_substrings("abc".to_string()), 3);
        assert_eq!(Solution::count_substrings("aaa".to_string()), 6);
    }
}
