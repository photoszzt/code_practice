/**
 * [7] Reverse Integer
 *
 * Given a 32-bit signed integer, reverse digits of an integer.
 * Note:<br />
 * Assume we are dealing with an environment that could only store integers within the 32-bit signed
 * integer range: [-2^31,  2^31 - 1]. For the purpose of this problem, assume that your function
 * returns 0 when the reversed integer overflows.
 *
 * Example 1:
 * Input: x = 123
 * Output: 321
 * Example 2:
 * Input: x = -123
 * Output: -321
 * Example 3:
 * Input: x = 120
 * Output: 21
 * Example 4:
 * Input: x = 0
 * Output: 0
 *
 * Constraints:
 *
 * 	-2^31 <= x <= 2^31 - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reverse-integer/
// discuss: https://leetcode.com/problems/reverse-integer/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut digits = Vec::<u8>::with_capacity(10);
        if x == 0 {
            return x;
        }
        let (mut to_pos, positive) = if x > 0 {
            (x as u32, true)
        } else {
            let mut xx: i64 = x as i64;
            xx = -xx;
            let xxx: u32 = xx as u32;
            (xxx, false)
        };
        while to_pos != 0 {
            let num = to_pos % 10;
            digits.push(num as u8);
            to_pos = to_pos / 10;
        }
        let mut res: i32 = 0;
        let mut power: i32 = 1;
        for digit in digits.iter().rev() {
            let cur_num_ret = (*digit as i32).checked_mul(power);
            let mut cur_num = 0;
            if let Some(num) = cur_num_ret {
                cur_num = num;
            } else {
                return 0;
            }
            if positive {
                let ret = res.checked_add(cur_num);
                if let Some(n) = ret {
                    res = n;
                } else {
                    return 0;
                }
            } else {
                let ret = res.checked_sub(cur_num);
                if let Some(n) = ret {
                    res = n;
                } else {
                    return 0;
                }
            }
            let p_ret = power.checked_mul(10);
            if let Some(p) = p_ret {
                power = p;
            } else {
                break;
            }
        }
        return res;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_7() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(0), 0);
        assert_eq!(Solution::reverse(2147483647), 0);
        assert_eq!(Solution::reverse(-2147483648), 0);
    }
}
