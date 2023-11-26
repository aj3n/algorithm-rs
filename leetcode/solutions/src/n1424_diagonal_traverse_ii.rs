/**
 * [1424] Diagonal Traverse II
 *
 * Given a 2D integer array nums, return all elements of nums in diagonal order as shown in the below images.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/04/08/sample_1_1784.png" style="width: 158px; height: 143px;" />
 * Input: nums = [[1,2,3],[4,5,6],[7,8,9]]
 * Output: [1,4,2,7,5,3,8,6,9]
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/04/08/sample_2_1784.png" style="width: 230px; height: 177px;" />
 * Input: nums = [[1,2,3,4,5],[6,7],[8],[9,10,11],[12,13,14,15,16]]
 * Output: [1,6,2,8,7,3,9,4,12,10,5,13,11,14,15,16]
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 10^5
 *     1 <= nums[i].length <= 10^5
 *     1 <= sum(nums[i].length) <= 10^5
 *     1 <= nums[i][j] <= 10^5
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
		let mut ans = vec![];
		for i in 0..nums.len() * 2 {
			for j in 0..=i {
				let k = i - j;
				if let Some(n) = nums.get(k).and_then(|v| v.get(j)) {
					ans.push(*n);
				}
			}
		}
		ans
	}
}

// submission codes end
