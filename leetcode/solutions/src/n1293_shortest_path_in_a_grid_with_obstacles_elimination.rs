/**
 * [1293] Shortest Path in a Grid with Obstacles Elimination
 *
 * You are given an m x n integer matrix grid where each cell is either 0 (empty) or 1 (obstacle). You can move up, down, left, or right from and to an empty cell in one step.
 * Return the minimum number of steps to walk from the upper left corner (0, 0) to the lower right corner (m - 1, n - 1) given that you can eliminate at most k obstacles. If it is not possible to find such walk return -1.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/09/30/short1-grid.jpg" style="width: 244px; height: 405px;" />
 * Input: grid = [[0,0,0],[1,1,0],[0,0,0],[0,1,1],[0,0,0]], k = 1
 * Output: 6
 * Explanation:
 * The shortest path without eliminating any obstacle is 10.
 * The shortest path with one obstacle elimination at position (3,2) is 6. Such path is (0,0) -> (0,1) -> (0,2) -> (1,2) -> (2,2) -> (3,2) -> (4,2).
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/09/30/short2-grid.jpg" style="width: 244px; height: 245px;" />
 * Input: grid = [[0,1,1],[1,1,1],[1,0,0]], k = 1
 * Output: -1
 * Explanation: We need to eliminate at least two obstacles to find such a walk.
 *
 *  
 * Constraints:
 *
 *     m == grid.length
 *     n == grid[i].length
 *     1 <= m, n <= 40
 *     1 <= k <= m * n
 *     grid[i][j] is either 0 or 1.
 *     grid[0][0] == grid[m - 1][n - 1] == 0
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
		let (m, n) = (grid.len(), grid[0].len());
		let mut visited = vec![vec![k + 1; n]; m];
		visited[0][0] = 0;

		let mut dq = vec![(0, 0, 0)];
		let mut step = 0;
		while !dq.is_empty() {
			for (i, j, brk) in std::mem::take(&mut dq) {
				if i as usize == m - 1 && j as usize == n - 1 {
					return step;
				}
				for (ii, jj) in [(i, j + 1), (i, j - 1), (i + 1, j), (i - 1, j)] {
					if ii >= 0
						&& ii < m as i32 && jj >= 0
						&& jj < n as i32 && visited[ii as usize][jj as usize] > brk
					{
						let brk = grid[ii as usize][jj as usize] + brk;
						if brk <= k {
							visited[ii as usize][jj as usize] = brk;
							dq.push((ii, jj, brk));
						}
					}
				}
			}
			step += 1;
		}
		-1
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1293() {
		let mat = matrix![[0, 1, 1], [1, 1, 1], [1, 0, 0]];
		assert_eq!(Solution::shortest_path(mat, 1), -1);
		let mat = matrix![[0, 0, 0], [1, 1, 0], [0, 0, 0], [0, 1, 1], [0, 0, 0]];
		assert_eq!(Solution::shortest_path(mat, 1), 6);
		assert_eq!(Solution::shortest_path(vec![vec![0]], 0), 0);
	}
}
