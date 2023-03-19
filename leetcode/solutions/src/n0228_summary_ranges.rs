/**
 * [228] Summary Ranges
 *
 * You are given a sorted unique integer array nums.
 * A range [a,b] is the set of all integers from a to b (inclusive).
 * Return the smallest sorted list of ranges that cover all the numbers in the array exactly. That is, each element of nums is covered by exactly one of the ranges, and there is no integer x such that x is in one of the ranges but not in nums.
 * Each range [a,b] in the list should be output as:
 *
 *     "a->b" if a != b
 *     "a" if a == b
 *
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [0,1,2,4,5,7]
 * Output: ["0->2","4->5","7"]
 * Explanation: The ranges are:
 * [0,2] --> "0->2"
 * [4,5] --> "4->5"
 * [7,7] --> "7"
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [0,2,3,4,6,8,9]
 * Output: ["0","2->4","6","8->9"]
 * Explanation: The ranges are:
 * [0,0] --> "0"
 * [2,4] --> "2->4"
 * [6,6] --> "6"
 * [8,9] --> "8->9"
 *
 *  
 * Constraints:
 *
 *     0 <= nums.length <= 20
 *     -2^31 <= nums[i] <= 2^31 - 1
 *     All the values of nums are unique.
 *     nums is sorted in ascending order.
 *
 */
pub struct Solution {}

impl Solution {
	pub fn summary_ranges(mut nums: Vec<i32>) -> Vec<String> {
		nums.push(i32::MIN);
		let mut start = 0;
		let mut ans = vec![];
		for (i, w) in nums.windows(2).enumerate() {
			if w[1] != w[0].saturating_add(1) {
				if nums[start] == w[0] {
					ans.push(w[0].to_string());
				} else {
					ans.push(format!("{}->{}", nums[start], w[0]));
				}
				start = i + 1;
			}
		}
		ans
	}
}
