/**
 * [516] Longest Palindromic Subsequence
 *
 * Given a string s, find the longest palindromic subsequence's length in s. You may assume that the maximum length of s is 1000.
 * Example 1:<br />
 * Input:
 *
 * "bbbab"
 * Output:
 *
 * 4
 * One possible longest palindromic subsequence is "bbbb".
 *
 * Example 2:<br />
 * Input:
 *
 * "cbbd"
 * Output:
 *
 * 2
 * One possible longest palindromic subsequence is "bb".
 *
 * Constraints:
 *
 * 	1 <= s.length <= 1000
 * 	s consists only of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-palindromic-subsequence/
// discuss: https://leetcode.com/problems/longest-palindromic-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();
        if len <= 1 {
            return len as i32;
        }
        let mut state = vec![vec![0; len]; len];
        for i in 0..len {
            state[i][i] = 1;
        }
        for i in (0..=len - 1).rev() {
            for j in i + 1..len {
                if chars[i] == chars[j] {
                    state[i][j] = state[i + 1][j - 1] + 2;
                } else {
                    state[i][j] = std::cmp::max(state[i + 1][j], state[i][j - 1]);
                }
            }
        }
        state[0][len - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_516() {
        assert_eq!(Solution::longest_palindrome_subseq("babad".to_string()), 3);
        assert_eq!(Solution::longest_palindrome_subseq("cbbd".to_string()), 2);
        assert_eq!(Solution::longest_palindrome_subseq("".to_string()), 0);
        assert_eq!(Solution::longest_palindrome_subseq("a".to_string()), 1);
        assert_eq!(Solution::longest_palindrome_subseq("ac".to_string()), 1);
        assert_eq!(Solution::longest_palindrome_subseq("bbbab".to_string()), 4);
    }
}
