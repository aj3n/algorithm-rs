/**
 * [2477] Minimum Fuel Cost to Report to the Capital
 *
 * There is a tree (i.e., a connected, undirected graph with no cycles) structure country network consisting of n cities numbered from 0 to n - 1 and exactly n - 1 roads. The capital city is city 0. You are given a 2D integer array roads where roads[i] = [ai, bi] denotes that there exists a bidirectional road connecting cities ai and bi.
 * There is a meeting for the representatives of each city. The meeting is in the capital city.
 * There is a car in each city. You are given an integer seats that indicates the number of seats in each car.
 * A representative can use the car in their city to travel or change the car and ride with another representative. The cost of traveling between two cities is one liter of fuel.
 * Return the minimum number of liters of fuel to reach the capital city.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/09/22/a4c380025e3ff0c379525e96a7d63a3.png" style="width: 303px; height: 332px;" />
 * Input: roads = [[0,1],[0,2],[0,3]], seats = 5
 * Output: 3
 * Explanation:
 * - Representative1 goes directly to the capital with 1 liter of fuel.
 * - Representative2 goes directly to the capital with 1 liter of fuel.
 * - Representative3 goes directly to the capital with 1 liter of fuel.
 * It costs 3 liters of fuel at minimum.
 * It can be proven that 3 is the minimum number of liters of fuel needed.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/11/16/2.png" style="width: 274px; height: 340px;" />
 * Input: roads = [[3,1],[3,2],[1,0],[0,4],[0,5],[4,6]], seats = 2
 * Output: 7
 * Explanation:
 * - Representative2 goes directly to city 3 with 1 liter of fuel.
 * - Representative2 and representative3 go together to city 1 with 1 liter of fuel.
 * - Representative2 and representative3 go together to the capital with 1 liter of fuel.
 * - Representative1 goes directly to the capital with 1 liter of fuel.
 * - Representative5 goes directly to the capital with 1 liter of fuel.
 * - Representative6 goes directly to city 4 with 1 liter of fuel.
 * - Representative4 and representative6 go together to the capital with 1 liter of fuel.
 * It costs 7 liters of fuel at minimum.
 * It can be proven that 7 is the minimum number of liters of fuel needed.
 *
 * <strong class="example">Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/09/27/efcf7f7be6830b8763639cfd01b690a.png" style="width: 108px; height: 86px;" />
 * Input: roads = [], seats = 1
 * Output: 0
 * Explanation: No representatives need to travel to the capital city.
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 10^5
 *     roads.length == n - 1
 *     roads[i].length == 2
 *     0 <= ai, bi < n
 *     ai != bi
 *     roads represents a valid tree.
 *     1 <= seats <= 10^5
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
		let mut neigs = vec![vec![]; roads.len() + 1];
		for r in roads {
			neigs[r[0] as usize].push(r[1]);
			neigs[r[1] as usize].push(r[0]);
		}
		Self::dfs(0, -1, &neigs, seats).0
	}

	fn dfs(cur: i32, from: i32, neigs: &[Vec<i32>], seats: i32) -> (i64, i32) {
		let (mut ans, mut sum) = (0, 1);
		for &i in neigs[cur as usize].iter() {
			if i != from {
				let (a, b) = Self::dfs(i, cur, neigs, seats);
				ans += a;
				sum += b;
			}
		}
		if from >= 0 {
			ans += (sum / seats + (sum % seats != 0) as i32) as i64;
		}
		(ans, sum)
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2477() {
		assert_eq!(Solution::minimum_fuel_cost(matrix![[0,1],[0,2],[0,3]], 5), 3);
		assert_eq!(Solution::minimum_fuel_cost(matrix![[3,1],[3,2],[1,0],[0,4],[0,5],[4,6]], 2), 7);
	}
}
