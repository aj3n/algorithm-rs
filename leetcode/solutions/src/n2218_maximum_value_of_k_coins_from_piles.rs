/**
 * [2218] Maximum Value of K Coins From Piles
 *
 * There are n piles of coins on a table. Each pile consists of a positive number of coins of assorted denominations.
 * In one move, you can choose any coin on top of any pile, remove it, and add it to your wallet.
 * Given a list piles, where piles[i] is a list of integers denoting the composition of the i^th pile from top to bottom, and a positive integer k, return the maximum total value of coins you can have in your wallet if you choose exactly k coins optimally.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/11/09/e1.png" style="width: 600px; height: 243px;" />
 * Input: piles = [[1,100,3],[7,8,9]], k = 2
 * Output: 101
 * Explanation:
 * The above diagram shows the different ways we can choose k coins.
 * The maximum total we can obtain is 101.
 *
 * <strong class="example">Example 2:
 *
 * Input: piles = [[100],[100],[100],[100],[100],[100],[1,1,1,1,1,1,700]], k = 7
 * Output: 706
 * Explanation:
 * The maximum total can be obtained if we choose all coins from the last pile.
 *
 *  
 * Constraints:
 *
 *     n == piles.length
 *     1 <= n <= 1000
 *     1 <= piles[i][j] <= 10^5
 *     1 <= k <= sum(piles[i].length) <= 2000
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::HashMap;
impl Solution {
	pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
		Self::solve(&piles, k, &mut HashMap::new())
	}

	fn solve(piles: &[Vec<i32>], k: i32, cache: &mut HashMap<(i32, i32), i32>) -> i32 {
		if let Some(&ans) = cache.get(&(piles.len() as i32, k)) {
			return ans;
		} else if piles.is_empty() || k == 0 {
			return 0;
		}

		let mut ans = Self::solve(&piles[1..], k, cache);
		let mut sum = 0;
		for (&n, c) in piles[0].iter().zip(1..=k) {
			sum += n;
			ans = ans.max(sum + Self::solve(&piles[1..], k - c, cache));
		}
		cache.insert((piles.len() as i32, k), ans);
		ans
	}
}

// submission codes end
