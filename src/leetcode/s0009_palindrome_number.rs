/**
 * [9] Palindrome Number
 *
 * Determine whether an integer is a palindrome. An integer is a palindrome when it reads the same backward as forward.
 * Follow up: Could you solve it without converting the integer to a string?
 *
 * Example 1:
 *
 * Input: x = 121
 * Output: true
 *
 * Example 2:
 *
 * Input: x = -121
 * Output: false
 * Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
 *
 * Example 3:
 *
 * Input: x = 10
 * Output: false
 * Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
 *
 * Example 4:
 *
 * Input: x = -101
 * Output: false
 *
 *
 * Constraints:
 *
 * 	-2^31 <= x <= 2^31 - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/palindrome-number/
// discuss: https://leetcode.com/problems/palindrome-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut digits = Vec::<u8>::with_capacity(10);
        let mut xx = x;
        while xx != 0 {
            let num = xx % 10;
            digits.push(num as u8);
            xx = xx / 10;
        }
        let len = digits.len();
        for i in 0..len / 2 {
            if digits[i] != digits[len - i - 1] {
                return false;
            }
        }
        return true;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_9() {
        assert_eq!(Solution::is_palindrome(10), false);
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
    }
}
