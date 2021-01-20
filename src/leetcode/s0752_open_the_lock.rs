/**
 * [752] Open the Lock
 *
 * You have a lock in front of you with 4 circular wheels. Each wheel has 10 slots: '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'. The wheels can rotate freely and wrap around: for example we can turn '9' to be '0', or '0' to be '9'. Each move consists of turning one wheel one slot.
 * The lock initially starts at '0000', a string representing the state of the 4 wheels.
 * You are given a list of deadends dead ends, meaning if the lock displays any of these codes, the wheels of the lock will stop turning and you will be unable to open it.
 * Given a target representing the value of the wheels that will unlock the lock, return the minimum total number of turns required to open the lock, or -1 if it is impossible.
 *
 * Example 1:
 *
 * Input: deadends = ["0201","0101","0102","1212","2002"], target = "0202"
 * Output: 6
 * Explanation:
 * A sequence of valid moves would be "0000" -> "1000" -> "1100" -> "1200" -> "1201" -> "1202" -> "0202".
 * Note that a sequence like "0000" -> "0001" -> "0002" -> "0102" -> "0202" would be invalid,
 * because the wheels of the lock become stuck after the display becomes the dead end "0102".
 *
 * Example 2:
 *
 * Input: deadends = ["8888"], target = "0009"
 * Output: 1
 * Explanation:
 * We can turn the last wheel in reverse to move from "0000" -> "0009".
 *
 * Example 3:
 *
 * Input: deadends = ["8887","8889","8878","8898","8788","8988","7888","9888"], target = "8888"
 * Output: -1
 * Explanation:
 * We can't reach the target without getting stuck.
 *
 * Example 4:
 *
 * Input: deadends = ["0000"], target = "8888"
 * Output: -1
 *
 *
 * Constraints:
 *
 * 	1 <= deadends.length <= 500
 * 	<font face="monospace">deadends[i].length == 4</font>
 * 	<font face="monospace">target.length == 4</font>
 * 	target will not be in the list deadends.
 * 	target and deadends[i] consist of digits only.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/open-the-lock/
// discuss: https://leetcode.com/problems/open-the-lock/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        use std::collections::{HashSet, VecDeque};

        let mut de = HashSet::new();
        de.extend(deadends.iter());
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back("0000".to_string());
        visited.insert("0000".to_string());
        let mut levels = 0;
        while (!queue.is_empty()) {
            let qlen = queue.len();
            for i in 0..qlen {
                let val = queue.pop_front().unwrap();
                if de.contains(&val) {
                    continue;
                }
                if val.eq(&target) {
                    return levels;
                }
                for j in 0..4 {
                    let t = val.chars().nth(j).unwrap().to_digit(10).unwrap();
                    let ut = if t == 9 { 0 } else { t + 1 };
                    let dt = if t == 0 { 9 } else { t - 1 };
                    let new_str1 = format!("{}{}{}", &val[0..j], ut, &val[j + 1..]);
                    let new_str2 = format!("{}{}{}", &val[0..j], dt, &val[j + 1..]);
                    if !de.contains(&new_str1) && !visited.contains(&new_str1) {
                        queue.push_back(new_str1.clone());
                        visited.insert(new_str1);
                    }
                    if !de.contains(&new_str2) && !visited.contains(&new_str2) {
                        queue.push_back(new_str2.clone());
                        visited.insert(new_str2);
                    }
                }
            }
            levels += 1;
        }
        return -1;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_752() {
        assert_eq!(
            Solution::open_lock(vec!["8888".to_string()], "0009".to_string()),
            1
        );
        assert_eq!(
            Solution::open_lock(
                vec![
                    "0201".to_string(),
                    "0101".to_string(),
                    "0102".to_string(),
                    "1212".to_string(),
                    "2002".to_string(),
                ],
                "0202".to_string()
            ),
            6
        );
        assert_eq!(
            Solution::open_lock(
                vec![
                    "8887".to_string(),
                    "8889".to_string(),
                    "8878".to_string(),
                    "8898".to_string(),
                    "8788".to_string(),
                    "8988".to_string(),
                    "7888".to_string(),
                    "9888".to_string(),
                ],
                "8888".to_string()
            ),
            -1
        );
        assert_eq!(
            Solution::open_lock(vec!["0000".to_string()], "8888".to_string()),
            -1
        );
    }
}
