/**
 * [5] Longest Palindromic Substring
 *
 * Given a string s, return the longest palindromic substring in s.
 *
 * Example 1:
 *
 * Input: s = "babad"
 * Output: "bab"
 * Note: "aba" is also a valid answer.
 *
 * Example 2:
 *
 * Input: s = "cbbd"
 * Output: "bb"
 *
 * Example 3:
 *
 * Input: s = "a"
 * Output: "a"
 *
 * Example 4:
 *
 * Input: s = "ac"
 * Output: "a"
 *
 *
 * Constraints:
 *
 * 	1 <= s.length <= 1000
 * 	s consist of only digits and English letters (lower-case and/or upper-case),
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-palindromic-substring/
// discuss: https://leetcode.com/problems/longest-palindromic-substring/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();
        if len <= 1 {
            return s;
        }
        let mut cur_start = 0;
        let mut cur_len = 0;
        let mut state = vec![vec![false; len]; len];
        for i in 0..len {
            state[i][i] = true;
        }
        for j in 0..len {
            for i in 0..=j {
                if chars[i] != chars[j] {
                    state[i][j] = false;
                } else {
                    if j - i < 3 {
                        state[i][j] = true;
                    } else {
                        state[i][j] = state[i + 1][j - 1];
                    }
                }
                if state[i][j] && j - i + 1 > cur_len {
                    cur_start = i;
                    cur_len = j - i + 1;
                }
            }
        }
        s[cur_start..cur_start + cur_len].to_owned()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_5() {
        assert_eq!(Solution::longest_palindrome("babad".to_string()), "bab");
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb");
        assert_eq!(Solution::longest_palindrome("".to_string()), "");
        assert_eq!(Solution::longest_palindrome("a".to_string()), "a");
        assert_eq!(Solution::longest_palindrome("ac".to_string()), "a");
    }
}
