/**
 * [2302] Count Subarrays With Score Less Than K
 *
 * The score of an array is defined as the product of its sum and its length.
 *
 *     For example, the score of [1, 2, 3, 4, 5] is (1 + 2 + 3 + 4 + 5) * 5 = 75.
 *
 * Given a positive integer array nums and an integer k, return the number of non-empty subarrays of nums whose score is strictly less than k.
 * A subarray is a contiguous sequence of elements within an array.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [2,1,4,3,5], k = 10
 * Output: 6
 * Explanation:
 * The 6 subarrays having scores less than 10 are:
 * - [2] with score 2 * 1 = 2.
 * - [1] with score 1 * 1 = 1.
 * - [4] with score 4 * 1 = 4.
 * - [3] with score 3 * 1 = 3.
 * - [5] with score 5 * 1 = 5.
 * - [2,1] with score (2 + 1) * 2 = 6.
 * Note that subarrays such as [1,4] and [4,3,5] are not considered because their scores are 10 and 36 respectively, while we need scores strictly less than 10.
 * <strong class="example">Example 2:
 *
 * Input: nums = [1,1,1], k = 5
 * Output: 5
 * Explanation:
 * Every subarray except [1,1,1] has a score less than 5.
 * [1,1,1] has a score (1 + 1 + 1) * 3 = 9, which is greater than 5.
 * Thus, there are 5 subarrays having scores less than 5.
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 10^5
 *     1 <= nums[i] <= 10^5
 *     1 <= k <= 10^15
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
		let (mut i, mut s) = (0, 0);
		nums.iter()
			.enumerate()
			.map(|(j, &n)| {
				s += n as i64;
				while s * (j + 1 - i) as i64 >= k {
					s -= nums[i] as i64;
					i += 1;
				}
				(j + 1 - i) as i64
			})
			.sum()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2302() {
		assert_eq!(Solution::count_subarrays(vec![2, 1, 4, 3, 5], 10), 6);
	}
}
