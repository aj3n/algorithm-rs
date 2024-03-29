/**
 * [977] Squares of a Sorted Array
 *
 * Given an integer array nums sorted in non-decreasing order, return an array of the squares of each number sorted in non-decreasing order.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [-4,-1,0,3,10]
 * Output: [0,1,9,16,100]
 * Explanation: After squaring, the array becomes [16,1,0,9,100].
 * After sorting, it becomes [0,1,9,16,100].
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [-7,-3,2,3,11]
 * Output: [4,9,9,49,121]
 *
 *  
 * Constraints:
 *
 *     <span>1 <= nums.length <= </span>10^4
 *     -10^4 <= nums[i] <= 10^4
 *     nums is sorted in non-decreasing order.
 *
 *  
 * Follow up: Squaring each element and sorting the new array is very trivial, could you find an O(n) solution using a different approach?
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn sorted_squares1(mut nums: Vec<i32>) -> Vec<i32> {
		nums.iter_mut().for_each(|i| *i *= *i);
		nums.sort_unstable();
		nums
	}
}

// submission codes end
