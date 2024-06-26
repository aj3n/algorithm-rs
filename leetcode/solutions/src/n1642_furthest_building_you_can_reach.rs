/**
 * [1762] Furthest Building You Can Reach
 *
 * You are given an integer array heights representing the heights of buildings, some bricks, and some ladders.
 * You start your journey from building 0 and move to the next building by possibly using bricks or ladders.
 * While moving from building i to building i+1 (0-indexed),
 *
 *     If the current building's height is greater than or equal to the next building's height, you do not need a ladder or bricks.
 *     If the current building's height is less than the next building's height, you can either use one ladder or (h[i+1] - h[i]) bricks.
 *
 * Return the furthest building index (0-indexed) you can reach if you use the given ladders and bricks optimally.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/27/q4.gif" style="width: 562px; height: 561px;" />
 * Input: heights = [4,2,7,6,9,14,12], bricks = 5, ladders = 1
 * Output: 4
 * Explanation: Starting at building 0, you can follow these steps:
 * - Go to building 1 without using ladders nor bricks since 4 >= 2.
 * - Go to building 2 using 5 bricks. You must use either bricks or ladders because 2 < 7.
 * - Go to building 3 without using ladders nor bricks since 7 >= 6.
 * - Go to building 4 using your only ladder. You must use either bricks or ladders because 6 < 9.
 * It is impossible to go beyond building 4 because you do not have any more bricks or ladders.
 *
 * Example 2:
 *
 * Input: heights = [4,  12,2,  7,3,  18, 20,3,  19], bricks = 10, ladders = 2
 * Output: 7
 *
 * Example 3:
 *
 * Input: heights = [14,3,19,3], bricks = 17, ladders = 0
 * Output: 3
 *
 *  
 * Constraints:
 *
 *     1 <= heights.length <= 10^5
 *     1 <= heights[i] <= 10^6
 *     0 <= bricks <= 10^9
 *     0 <= ladders <= heights.length
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn furthest_building(heights: Vec<i32>, mut bricks: i32, ladders: i32) -> i32 {
		let mut pq = std::collections::BinaryHeap::new();
		for (i, w) in (0..).zip(heights.windows(2)) {
			let height_diff = w[1] - w[0];
			if height_diff > 0 {
				pq.push(-height_diff);
			}
			if pq.len() as i32 > ladders {
				bricks += pq.pop().unwrap();
			}
			if bricks < 0 {
				return i;
			}
		}
		heights.len() as i32 - 1
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1762() {
		assert_eq!(
			Solution::furthest_building(vec![4, 2, 7, 6, 9, 14, 12], 5, 1),
			4
		);
		assert_eq!(
			Solution::furthest_building(vec![4, 12, 2, 7, 3, 18, 20, 3, 19], 10, 2),
			7
		);
		assert_eq!(Solution::furthest_building(vec![14, 3, 19, 3], 17, 0), 3);
	}
}
