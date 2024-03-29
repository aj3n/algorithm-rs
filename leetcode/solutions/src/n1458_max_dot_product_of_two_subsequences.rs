/**
 * [1569] Max Dot Product of Two Subsequences
 *
 * Given two arrays nums1 and <font face="monospace">nums2</font><font face="monospace">.</font>
 * Return the maximum dot product between non-empty subsequences of nums1 and nums2 with the same length.
 * A subsequence of a array is a new array which is formed from the original array by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (ie, [2,3,5] is a subsequence of [1,2,3,4,5] while [1,5,3] is not).
 *  
 * Example 1:
 *
 * Input: nums1 = [2,1,-2,5], nums2 = [3,0,-6]
 * Output: 18
 * Explanation: Take subsequence [2,-2] from nums1 and subsequence [3,-6] from nums2.
 * Their dot product is (2*3 + (-2)*(-6)) = 18.
 * Example 2:
 *
 * Input: nums1 = [3,-2], nums2 = [2,-6,7]
 * Output: 21
 * Explanation: Take subsequence [3] from nums1 and subsequence [7] from nums2.
 * Their dot product is (3*7) = 21.
 * Example 3:
 *
 * Input: nums1 = [-1,-1], nums2 = [1,1]
 * Output: -1
 * Explanation: Take subsequence [-1] from nums1 and subsequence [1] from nums2.
 * Their dot product is -1.
 *  
 * Constraints:
 *
 *     1 <= nums1.length, nums2.length <= 500
 *     -1000 <= nums1[i], nums2[i] <= 1000
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
		Self::solve(
			&nums1,
			&nums2,
			&mut vec![vec![i32::MIN; nums2.len() + 1]; nums1.len() + 1],
		)
	}

	fn solve(nums1: &[i32], nums2: &[i32], cache: &mut [Vec<i32>]) -> i32 {
		if cache[nums1.len()][nums2.len()] != i32::MIN {
			return cache[nums1.len()][nums2.len()];
		}
		let ans = match (nums1, nums2) {
			([a, rest1 @ ..], [b, rest2 @ ..]) => (a * b + 0.max(Self::solve(rest1, rest2, cache)))
				.max(Self::solve(nums1, rest2, cache))
				.max(Self::solve(rest1, nums2, cache)),
			_ => i32::MIN,
		};
		cache[nums1.len()][nums2.len()] = ans;
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1458() {
		assert_eq!(
			Solution::max_dot_product(vec![2, 1, -2, 5], vec![3, 0, -6]),
			18
		);
		assert_eq!(Solution::max_dot_product(vec![3, -2], vec![2, -6, 7]), 21);
		assert_eq!(Solution::max_dot_product(vec![-1, -1], vec![1, 1]), -1);
	}
}
