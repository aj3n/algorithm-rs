/**
 * [62] Unique Paths
 *
 * There is a robot on an m x n grid. The robot is initially located at the top-left corner (i.e., grid[0][0]). The robot tries to move to the bottom-right corner (i.e., grid[m - 1][n - 1]). The robot can only move either down or right at any point in time.
 * Given the two integers m and n, return the number of possible unique paths that the robot can take to reach the bottom-right corner.
 * The test cases are generated so that the answer will be less than or equal to 2 * 10^9.
 *  
 * <strong class="example">Example 1:
 * <img src="https://assets.leetcode.com/uploads/2018/10/22/robot_maze.png" style="width: 400px; height: 183px;" />
 * Input: m = 3, n = 7
 * Output: 28
 *
 * <strong class="example">Example 2:
 *
 * Input: m = 3, n = 2
 * Output: 3
 * Explanation: From the top-left corner, there are a total of 3 ways to reach the bottom-right corner:
 * 1. Right -> Down -> Down
 * 2. Down -> Down -> Right
 * 3. Down -> Right -> Down
 *
 *  
 * Constraints:
 *
 *     1 <= m, n <= 100
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn unique_paths(m: i32, n: i32) -> i32 {
		let (m, n) = (m.min(n), m.max(n));
		let mut dp = vec![0; m as usize];
		dp[0] = 1;
		for _ in 0..n {
			for i in 1..dp.len() {
				dp[i] += dp[i - 1];
			}
		}
		dp[m as usize - 1]
	}
}

// submission codes end
