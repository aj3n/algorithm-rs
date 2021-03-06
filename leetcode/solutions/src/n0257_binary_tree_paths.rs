/**
 * [257] Binary Tree Paths
 *
 * Given a binary tree, return all root-to-leaf paths.
 *
 * Note: A leaf is a node with no children.
 *
 * Example:
 *
 *
 * Input:
 *
 *    1
 *  /   \
 * 2     3
 *  \
 *   5
 *
 * Output: ["1->2->5", "1->3"]
 *
 * Explanation: All root-to-leaf paths are: 1->2->5, 1->3
 *
 */
pub struct Solution {}
use super::util::tree::TreeNode;

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
use std::{cell::RefCell, rc::Rc};
#[allow(dead_code)]
impl Solution {
	pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
		let mut res = Vec::new();
		Solution::helper(root, "".to_owned(), &mut res);
		res
	}

	fn helper(node: Option<Rc<RefCell<TreeNode>>>, path: String, res: &mut Vec<String>) {
		if let Some(node) = node {
			let node = node.borrow();
			if node.left.is_none() && node.right.is_none() {
				res.push(format!("{}{}", path, node.val));
			} else {
				let path = format!("{}{}->", path, node.val);
				Solution::helper(node.left.clone(), path.clone(), res);
				Solution::helper(node.right.clone(), path, res);
			}
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::{super::util::tree::to_tree, *};

	#[test]
	fn test_257() {
		assert_eq!(
			Solution::binary_tree_paths(tree![1, 2, 3, null, 5]),
			vec_string!["1->2->5", "1->3"]
		);
	}
}
