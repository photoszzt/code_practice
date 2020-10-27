/**
 * [107] Binary Tree Level Order Traversal II
 *
 * Given a binary tree, return the bottom-up level order traversal of its nodes' values. (ie, from left to right, level by level from leaf to root).
 *
 *
 * For example:<br />
 * Given binary tree [3,9,20,null,null,15,7],<br />
 *
 *     3
 *    / \
 *   9  20
 *     /  \
 *    15   7
 *
 *
 *
 * return its bottom-up level order traversal as:<br />
 *
 * [
 *   [15,7],
 *   [9,20],
 *   [3]
 * ]
 *
 *
 */
pub struct Solution {}
use crate::{
    leetcode::util::tree::{to_tree, TreeNode},
    tree,
};

// problem: https://leetcode.com/problems/binary-tree-level-order-traversal-ii/
// discuss: https://leetcode.com/problems/binary-tree-level-order-traversal-ii/discuss/?currentPage=1&orderBy=most_votes&query=

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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut trace = vec![];
        if root.is_none() {
            return trace;
        }
        let mut queue = VecDeque::new();
        queue.push_back((root, 1));
        while (!queue.is_empty()) {
            let (front, level) = queue.pop_front().unwrap();
            if trace.len() < level {
                trace.push(vec![]);
            }
            let mut cur_trace = trace.get_mut(level - 1).unwrap();
            if let Some(front) = front {
                let front = front.borrow();
                cur_trace.push(front.val);
                if front.left.is_some() {
                    queue.push_back((front.left.clone(), level + 1));
                }
                if front.right.is_some() {
                    queue.push_back((front.right.clone(), level + 1));
                }
            }
        }
        trace.reverse();
        trace
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_107() {
        assert_eq!(
            Solution::level_order_bottom(tree!(3, 9, 20, (), (), 15, 7)),
            vec![vec![15, 7], vec![9, 20], vec![3]]
        );
    }
}
