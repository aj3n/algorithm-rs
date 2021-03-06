/**
 * [136] Single Number
 *
 * Given a non-empty array of integers, every element appears twice except for one. Find that single one.
 *
 * Note:
 *
 * Your algorithm should have a linear runtime complexity. Could you implement it without using extra memory?
 *
 * Example 1:
 *
 *
 * Input: [2,2,1]
 * Output: 1
 *
 *
 * Example 2:
 *
 *
 * Input: [4,1,2,1,2]
 * Output: 4
 *
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn single_number(nums: Vec<i32>) -> i32 { nums.iter().fold(0, |acc, n| acc ^ n) }
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_136() {
		assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
		assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
	}
}
