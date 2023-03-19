/**
 * [787] Cheapest Flights Within K Stops
 *
 * There are n cities connected by some number of flights. You are given an array flights where flights[i] = [fromi, toi, pricei] indicates that there is a flight from city fromi to city toi with cost pricei.
 * You are also given three integers src, dst, and k, return the cheapest price from src to dst with at most k stops. If there is no such route, return -1.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/03/18/cheapest-flights-within-k-stops-3drawio.png" style="width: 332px; height: 392px;" />
 * Input: n = 4, flights = [[0,1,100],[1,2,100],[2,0,100],[1,3,600],[2,3,200]], src = 0, dst = 3, k = 1
 * Output: 700
 * Explanation:
 * The graph is shown above.
 * The optimal path with at most 1 stop from city 0 to 3 is marked in red and has cost 100 + 600 = 700.
 * Note that the path through cities [0,1,2,3] is cheaper but is invalid because it uses 2 stops.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/03/18/cheapest-flights-within-k-stops-1drawio.png" style="width: 332px; height: 242px;" />
 * Input: n = 3, flights = [[0,1,100],[1,2,100],[0,2,500]], src = 0, dst = 2, k = 1
 * Output: 200
 * Explanation:
 * The graph is shown above.
 * The optimal path with at most 1 stop from city 0 to 2 is marked in red and has cost 100 + 100 = 200.
 *
 * <strong class="example">Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/03/18/cheapest-flights-within-k-stops-2drawio.png" style="width: 332px; height: 242px;" />
 * Input: n = 3, flights = [[0,1,100],[1,2,100],[0,2,500]], src = 0, dst = 2, k = 0
 * Output: 500
 * Explanation:
 * The graph is shown above.
 * The optimal path with no stops from city 0 to 2 is marked in red and has cost 500.
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 100
 *     0 <= flights.length <= (n * (n - 1) / 2)
 *     flights[i].length == 3
 *     0 <= fromi, toi < n
 *     fromi != toi
 *     1 <= pricei <= 10^4
 *     There will not be any multiple flights between two cities.
 *     0 <= src, dst, k < n
 *     src != dst
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
		let mut fli = vec![vec![]; n as usize];
		for f in flights {
			fli[f[0] as usize].push((f[1], f[2]));
		}
		Self::solve(&fli, src, dst, k + 1, &mut std::collections::HashMap::new()).unwrap_or(-1)
	}

	fn solve(
		flights: &[Vec<(i32, i32)>],
		cur: i32,
		dst: i32,
		k: i32,
		cache: &mut std::collections::HashMap<(i32, i32), Option<i32>>,
	) -> Option<i32> {
		if cur == dst {
			return Some(0);
		} else if k == 0 {
			return None;
		} else if let Some(ans) = cache.get(&(cur, k)) {
			return *ans;
		}

		let mut ans = None;
		let mut min = i32::MAX;
		for &(d, p) in &flights[cur as usize] {
			if let Some(v) = Self::solve(flights, d, dst, k - 1, cache) {
				min = min.min(v + p);
				ans = Some(min);
			}
		}
		cache.insert((cur, k), ans);

		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_787() {
		assert_eq!(
			Solution::find_cheapest_price(
				5,
				// 0 3 2 1 4
				matrix![
					[0, 1, 100],
					[0, 2, 100],
					[0, 3, 10],
					[1, 2, 100],
					[1, 4, 10],
					[2, 1, 10],
					[2, 3, 100],
					[2, 4, 100],
					[3, 2, 10],
					[3, 4, 100]
				],
				0,
				4,
				3
			),
			40
		);
		assert_eq!(
			Solution::find_cheapest_price(
				4,
				matrix![
					[0, 1, 100],
					[1, 2, 100],
					[2, 0, 100],
					[1, 3, 600],
					[2, 3, 200]
				],
				0,
				3,
				1
			),
			700
		);
	}
}