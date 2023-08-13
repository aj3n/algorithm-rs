/**
 * [2920] Maximum Points After Collecting Coins From All Nodes
 *
 * There exists an undirected tree rooted at node 0 with n nodes labeled from 0 to n - 1. You are given a 2D integer array edges of length n - 1, where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the tree. You are also given a 0-indexed array coins of size n where coins[i] indicates the number of coins in the vertex i, and an integer k.
 * Starting from the root, you have to collect all the coins such that the coins at a node can only be collected if the coins of its ancestors have been already collected.
 * Coins at nodei can be collected in one of the following ways:
 *
 *     Collect all the coins, but you will get coins[i] - k points. If coins[i] - k is negative then you will lose abs(coins[i] - k) points.
 *     Collect all the coins, but you will get floor(coins[i] / 2) points. If this way is used, then for all the nodej present in the subtree of nodei, coins[j] will get reduced to floor(coins[j] / 2).
 *
 * Return the maximum points you can get after collecting the coins from all the tree nodes.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/09/18/ex1-copy.png" style="width: 60px; height: 316px; padding: 10px; background: rgb(255, 255, 255); border-radius: 0.5rem;" />
 * Input: edges = [[0,1],[1,2],[2,3]], coins = [10,10,3,3], k = 5
 * Output: 11                        
 * Explanation:
 * Collect all the coins from node 0 using the first way. Total points = 10 - 5 = 5.
 * Collect all the coins from node 1 using the first way. Total points = 5 + (10 - 5) = 10.
 * Collect all the coins from node 2 using the second way so coins left at node 3 will be floor(3 / 2) = 1. Total points = 10 + floor(3 / 2) = 11.
 * Collect all the coins from node 3 using the second way. Total points = 11 + floor(1 / 2) = 11.
 * It can be shown that the maximum points we can get after collecting coins from all the nodes is 11.
 *
 * <strong class="example">Example 2:
 * <strong class="example"> <img alt="" src="https://assets.leetcode.com/uploads/2023/09/18/ex2.png" style="width: 140px; height: 147px; padding: 10px; background: #fff; border-radius: .5rem;" />
 *
 * Input: edges = [[0,1],[0,2]], coins = [8,4,4], k = 0
 * Output: 16
 * Explanation:
 * Coins will be collected from all the nodes using the first way. Therefore, total points = (8 - 0) + (4 - 0) + (4 - 0) = 16.
 *
 *  
 * Constraints:
 *
 *     n == coins.length
 *     2 <= n <= 10^5
 *     <font face="monospace">0 <= coins[i] <= 10^4</font>
 *     edges.length == n - 1
 *     <font face="monospace">0 <= edges[i][0], edges[i][1] < n</font>
 *     <font face="monospace">0 <= k <= 10^4</font>
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::HashMap;
impl Solution {
	pub fn maximum_points(edges: Vec<Vec<i32>>, coins: Vec<i32>, k: i32) -> i32 {
		let mut neigs = HashMap::new();
		for e in edges {
			neigs.entry(e[0]).or_insert_with(Vec::new).push(e[1]);
			neigs.entry(e[1]).or_insert_with(Vec::new).push(e[0]);
		}
		Self::dfs(0, 0, -1, &neigs, &coins, k, &mut HashMap::new())
	}

	fn dfs(
		cur: i32,
		j: i32,
		par: i32,
		neigs: &HashMap<i32, Vec<i32>>,
		coins: &[i32],
		k: i32,
		cache: &mut HashMap<(i32, i32), i32>,
	) -> i32 {
		if let Some(ans) = cache.get(&(cur, j)) {
			return *ans;
		}
		let mut a1 = (coins[cur as usize] >> j) - k;
		let mut a2 = coins[cur as usize] >> (j + 1);

		for &nei in neigs.get(&cur).unwrap() {
			if nei != par {
				a1 += Self::dfs(nei, j, cur, neigs, coins, k, cache);
				if j < 14 {
					a2 += Self::dfs(nei, j + 1, cur, neigs, coins, k, cache);
				}
			}
		}
		cache.insert((cur, j), a1.max(a2));

		a1.max(a2)
	}
}

// submission codes end
