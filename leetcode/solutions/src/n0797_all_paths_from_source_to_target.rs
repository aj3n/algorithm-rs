/**
 * [797] All Paths From Source to Target
 *
 * Given a directed acyclic graph (DAG) of n nodes labeled from 0 to n - 1, find all possible paths from node 0 to node n - 1 and return them in any order.
 * The graph is given as follows: graph[i] is a list of all nodes you can visit from node i (i.e., there is a directed edge from node i to node graph[i][j]).
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/28/all_1.jpg" style="width: 242px; height: 242px;" />
 * Input: graph = [[1,2],[3],[3],[]]
 * Output: [[0,1,3],[0,2,3]]
 * Explanation: There are two paths: 0 -> 1 -> 3 and 0 -> 2 -> 3.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/28/all_2.jpg" style="width: 423px; height: 301px;" />
 * Input: graph = [[4,3,1],[3,2,4],[3],[4],[]]
 * Output: [[0,4],[0,3,4],[0,1,3,4],[0,1,2,3,4],[0,1,4]]
 *
 *  
 * Constraints:
 *
 *     n == graph.length
 *     2 <= n <= 15
 *     0 <= graph[i][j] < n
 *     graph[i][j] != i (i.e., there will be no self-loops).
 *     All the elements of graph[i] are unique.
 *     The input graph is guaranteed to be a DAG.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
		let mut dq = vec![(0, vec![0])];
		let n = graph.len() as i32;
		let mut ans = vec![];
		while !dq.is_empty() {
			for (i, path) in std::mem::take(&mut dq) {
				for &j in &graph[i as usize] {
					let mut path = path.clone();
					path.push(j);
					dq.push((j, path));
				}
				if i == n - 1 {
					ans.push(path);
				}
			}
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_797() {}
}