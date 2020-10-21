/**
 * [6] ZigZag Conversion
 *
 * The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)
 *
 * P   A   H   N
 * A P L S I I G
 * Y   I   R
 *
 * And then read line by line: "PAHNAPLSIIGYIR"
 * Write the code that will take a string and make this conversion given a number of rows:
 *
 * string convert(string s, int numRows);
 *
 *
 * Example 1:
 *
 * Input: s = "PAYPALISHIRING", numRows = 3
 * Output: "PAHNAPLSIIGYIR"
 *
 * Example 2:
 *
 * Input: s = "PAYPALISHIRING", numRows = 4
 * Output: "PINALSIGYAHRPI"
 * Explanation:
 * P     I    N
 * A   L S  I G
 * Y A   H R
 * P     I
 *
 * Example 3:
 *
 * Input: s = "A", numRows = 1
 * Output: "A"
 *
 *
 * Constraints:
 *
 * 	1 <= s.length <= 1000
 * 	s consists of English letters (lower-case and upper-case), ',' and '.'.
 * 	1 <= numRows <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/zigzag-conversion/
// discuss: https://leetcode.com/problems/zigzag-conversion/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let chars: Vec<char> = s.chars().collect();
        let mut new_str = Vec::new();
        let len = chars.len() as i32;
        if num_rows == 1 {
            return s;
        }
        for i in 0..num_rows {
            let mut gap = 2 * num_rows - 2 - 2 * i;
            if gap == 0 {
                gap = 2 * num_rows - 2;
            }
            let mut idx = i;
            while idx < len {
                eprintln!("gap {}, idx {}", gap, idx);
                new_str.push(chars[idx as usize]);
                idx += gap;
                gap = 2 * num_rows - 2 - gap;
                if gap == 0 {
                    gap = 2 * num_rows - 2;
                }
            }
        }
        new_str.iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_6() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR"
        );
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI"
        );
        assert_eq!(Solution::convert("A".to_string(), 1), "A");
    }
}
