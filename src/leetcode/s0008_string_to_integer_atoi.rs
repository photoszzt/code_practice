/**
 * [8] String to Integer (atoi)
 *
 * Implement <span>atoi</span> which converts a string to an integer.
 * The function first discards as many whitespace characters as necessary until the first non-whitespace character is found. Then, starting from this character takes an optional initial plus or minus sign followed by as many numerical digits as possible, and interprets them as a numerical value.
 * The string can contain additional characters after those that form the integral number, which are ignored and have no effect on the behavior of this function.
 * If the first sequence of non-whitespace characters in str is not a valid integral number, or if no such sequence exists because either str is empty or it contains only whitespace characters, no conversion is performed.
 * If no valid conversion could be performed, a zero value is returned.
 * Note:
 *
 * 	Only the space character ' ' is considered a whitespace character.
 * 	Assume we are dealing with an environment that could only store integers within the 32-bit signed integer range: [-2^31,  2^31 - 1]. If the numerical value is out of the range of representable values, 2^31 - 1 or -2^31 is returned.
 *
 *
 * Example 1:
 *
 * Input: str = "42"
 * Output: 42
 *
 * Example 2:
 *
 * Input: str = "   -42"
 * Output: -42
 * Explanation: The first non-whitespace character is '-', which is the minus sign. Then take as many numerical digits as possible, which gets 42.
 *
 * Example 3:
 *
 * Input: str = "4193 with words"
 * Output: 4193
 * Explanation: Conversion stops at digit '3' as the next character is not a numerical digit.
 *
 * Example 4:
 *
 * Input: str = "words and 987"
 * Output: 0
 * Explanation: The first non-whitespace character is 'w', which is not a numerical digit or a +/- sign. Therefore no valid conversion could be performed.
 *
 * Example 5:
 *
 * Input: str = "-91283472332"
 * Output: -2147483648
 * Explanation: The number "-91283472332" is out of the range of a 32-bit signed integer. Thefore INT_MIN (-2^31) is returned.
 *
 *
 * Constraints:
 *
 * 	0 <= s.length <= 200
 * 	s consists of English letters (lower-case and upper-case), digits, ' ', '+', '-' and '.'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/string-to-integer-atoi/
// discuss: https://leetcode.com/problems/string-to-integer-atoi/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut start = None;
        let mut len = 0;
        let mut positive = None;
        for (idx, c) in s.chars().enumerate() {
            match c {
                'a'..='z' | 'A'..='Z' => {
                    if start.is_some() {
                        break;
                    } else {
                        return 0;
                    }
                }
                ' ' => {
                    if start.is_some() {
                        break;
                    }
                    if positive.is_some() {
                        return 0;
                    }
                }
                '-' => {
                    if positive.is_some() {
                        break;
                    }
                    if let Some(start_n) = start {
                        if idx > start_n {
                            break;
                        }
                    }
                    positive = Some(false);
                }
                '+' => {
                    if positive.is_some() {
                        break;
                    }
                    if let Some(start_n) = start {
                        if idx > start_n {
                            break;
                        }
                    }
                    positive = Some(true);
                }
                '0'..='9' => {
                    if start.is_none() {
                        start = Some(idx);
                    }
                    len += 1;
                }
                _ => {
                    if start.is_some() {
                        len = idx;
                        break;
                    } else {
                        return 0;
                    }
                }
            }
        }
        let mut num: i32 = 0;
        let mut power: i32 = 1;
        let mut start_num = 0;
        if let Some(res) = start {
            start_num = res;
        } else {
            return 0;
        }
        let positive_ret = positive.unwrap_or(true);
        let mut overflow = false;
        let mut starting_zero = 0;
        for c in s[start_num..start_num + len].chars() {
            if c == '0' {
                starting_zero += 1;
            } else {
                break;
            }
        }
        start_num += starting_zero;
        len -= starting_zero;
        for c in s[start_num..start_num + len].chars().rev() {
            if overflow {
                if positive_ret {
                    return 2147483647;
                } else {
                    return -2147483648;
                }
            }
            let mut n = c.to_digit(10).unwrap() as i32;
            if let Some(res) = n.checked_mul(power) {
                n = res;
            } else {
                if positive_ret {
                    return 2147483647;
                } else {
                    return -2147483648;
                }
            }
            if positive_ret {
                if let Some(res) = num.checked_add(n) {
                    num = res;
                } else {
                    return 2147483647;
                }
            } else {
                if let Some(res) = num.checked_sub(n) {
                    num = res;
                } else {
                    return -2147483648;
                }
            }
            if let Some(res) = power.checked_mul(10) {
                power = res;
            } else {
                overflow = true;
            }
        }
        num
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_8() {
        assert_eq!(Solution::my_atoi("00000-42a1234".to_string()), 0);
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
        assert_eq!(Solution::my_atoi("2147483647".to_string()), 2147483647);
        assert_eq!(Solution::my_atoi("-2147483648".to_string()), -2147483648);
        assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
        assert_eq!(Solution::my_atoi("".to_string()), 0);
        assert_eq!(
            Solution::my_atoi("20000000000000000000".to_string()),
            2147483647
        );
        assert_eq!(
            Solution::my_atoi("  0000000000012345678".to_string()),
            12345678
        );
    }
}
