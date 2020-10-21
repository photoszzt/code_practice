/**
 * [69] Sqrt(x)
 *
 * Implement int sqrt(int x).
 *
 * Compute and return the square root of x, where x is guaranteed to be a non-negative integer.
 *
 * Since the return type is an integer, the decimal digits are truncated and only the integer part of the result is returned.
 *
 * Example 1:
 *
 *
 * Input: 4
 * Output: 2
 *
 *
 * Example 2:
 *
 *
 * Input: 8
 * Output: 2
 * Explanation: The square root of 8 is 2.82842..., and since
 *              the decimal part is truncated, 2 is returned.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sqrtx/
// discuss: https://leetcode.com/problems/sqrtx/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2 {
            return x;
        }
        let mut low = 1;
        let mut high = x;
        let mut mid = 0;
        while low + 1 < high {
            mid = low + (high - low) / 2;
            if mid > x / mid {
                high = mid;
            } else if mid < x / mid {
                low = mid;
            } else {
                return mid;
            }
        }
        low
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_69() {
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
    }
}
