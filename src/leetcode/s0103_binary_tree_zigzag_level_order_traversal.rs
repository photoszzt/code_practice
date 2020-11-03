/**
 * [103] Binary Tree Zigzag Level Order Traversal
 *
 * Given a binary tree, return the zigzag level order traversal of its nodes' values. (ie, from left to right, then right to left for the next level and alternate between).
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
 * return its zigzag level order traversal as:<br />
 *
 * [
 *   [3],
 *   [20,9],
 *   [15,7]
 * ]
 *
 *
 */
pub struct Solution {}
use crate::{
    leetcode::util::tree::{to_tree, TreeNode},
    tree,
};

// problem: https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/
// discuss: https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
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
        for (idx, cur_trace) in trace.iter_mut().enumerate() {
            if idx % 2 == 1 {
                cur_trace.reverse();
            }
        }
        trace
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_103_1() {
        assert_eq!(
            Solution::zigzag_level_order(tree!(3, 9, 20, (), (), 15, 7)),
            vec![vec![3], vec![20, 9], vec![15, 7]]
        )
    }

    #[test]
    fn test_103_2() {
        assert_eq!(
            Solution::zigzag_level_order(tree!(1, 2, 3, 4, (), (), 5)),
            vec![vec![1], vec![3, 2], vec![4, 5]]
        )
    }

    #[test]
    fn test_103_3() {
        assert_eq!(
            Solution::zigzag_level_order(tree!(0, 2, 4, 1, (), 3, -1, 5, 1, (), 6, (), 8)),
            vec![vec![0], vec![4, 2], vec![1, 3, -1], vec![8, 6, 1, 5]]
        )
    }
}
