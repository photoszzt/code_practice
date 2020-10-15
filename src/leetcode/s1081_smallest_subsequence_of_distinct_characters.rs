/**
 * [1081] Smallest Subsequence of Distinct Characters
 *
 * Return the lexicographically smallest subsequence of s that contains all the distinct characters of s exactly once.
 * Note: This question is the same as 316: <a href="https://leetcode.com/problems/remove-duplicate-letters/" target="_blank">https://leetcode.com/problems/remove-duplicate-letters/</a>
 *
 * Example 1:
 *
 * Input: s = "bcabc"
 * Output: "abc"
 *
 * Example 2:
 *
 * Input: s = "cbacdcbc"
 * Output: "acdb"
 *
 *
 * Constraints:
 *
 * 	1 <= s.length <= 1000
 * 	s consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/smallest-subsequence-of-distinct-characters/
// discuss: https://leetcode.com/problems/smallest-subsequence-of-distinct-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn smallest_subsequence(s: String) -> String {
        let mut last_idx: [i16; 26] = [-1; 26];
        let mut is_used: [bool; 26] = [false; 26];
        let mut stack = Vec::with_capacity(s.len());
        let a_num = 'a' as usize;
        for (idx, c) in s.chars().enumerate() {
            last_idx[c as usize - a_num] = idx as i16;
        }
        for (idx, c) in s.chars().enumerate() {
            let c_num = c as usize;
            while !stack.is_empty() && !is_used[c_num - a_num] {
                let top = stack[stack.len() - 1];
                if top >= c && last_idx[top as usize - a_num] >= idx as i16 {
                    stack.pop();
                    is_used[top as usize - a_num] = false;
                } else {
                    break;
                }
            }
            if !is_used[c_num - a_num] {
                stack.push(c);
                is_used[c_num - a_num] = true;
            }
        }
        stack.into_iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1081() {
        assert_eq!(Solution::smallest_subsequence("bcabc".to_string()), "abc");
        assert_eq!(
            Solution::smallest_subsequence("cbacdcbc".to_string()),
            "acdb"
        );
    }
}
