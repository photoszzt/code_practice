/**
 * [111] Minimum Depth of Binary Tree
 *
 * Given a binary tree, find its minimum depth.
 * The minimum depth is the number of nodes along the shortest path from the root node down to the nearest leaf node.
 * Note: A leaf is a node with no children.
 *
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/12/ex_depth.jpg" style="width: 432px; height: 302px;" />
 * Input: root = [3,9,20,null,null,15,7]
 * Output: 2
 *
 * Example 2:
 *
 * Input: root = [2,null,3,null,4,null,5,null,6]
 * Output: 5
 *
 *
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [0, 10^5].
 * 	-1000 <= Node.val <= 1000
 *
 */
pub struct Solution {}
use crate::{
    leetcode::util::tree::{to_tree, TreeNode},
    tree,
};

// problem: https://leetcode.com/problems/minimum-depth-of-binary-tree/
// discuss: https://leetcode.com/problems/minimum-depth-of-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        } else {
            let r = root.unwrap();
            let r = r.borrow();
            let left_min = Solution::min_depth(r.left.clone());
            let right_min = Solution::min_depth(r.right.clone());
            if (left_min == 0) {
                return right_min + 1;
            } else if (right_min == 0) {
                return left_min + 1;
            } else {
                if left_min <= right_min {
                    return left_min + 1;
                } else {
                    return right_min + 1;
                }
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_111() {
        assert_eq!(Solution::min_depth(tree!(3, 9, 20, (), (), 15, 7)), 2);
        assert_eq!(Solution::min_depth(tree!(2, (), 3, (), 4, (), 5, (), 6)), 5);
    }
}
