/**
 * [2611] Mice and Cheese
 *
 * There are two mice and n different types of cheese, each type of cheese should be eaten by exactly one mouse.
 * A point of the cheese with index i (0-indexed) is:
 *
 *     reward1[i] if the first mouse eats it.
 *     reward2[i] if the second mouse eats it.
 *
 * You are given a positive integer array reward1, a positive integer array reward2, and a non-negative integer k.
 * Return the maximum points the mice can achieve if the first mouse eats exactly k types of cheese.
 *  
 * <strong class="example">Example 1:
 *
 * Input: reward1 = [1,1,3,4], reward2 = [4,4,1,1], k = 2
 * Output: 15
 * Explanation: In this example, the first mouse eats the 2^nd (0-indexed) and the 3^rd types of cheese, and the second mouse eats the 0^th and the 1^st types of cheese.
 * The total points are 4 + 4 + 3 + 4 = 15.
 * It can be proven that 15 is the maximum total points that the mice can achieve.
 *
 * <strong class="example">Example 2:
 *
 * Input: reward1 = [1,1], reward2 = [1,1], k = 2
 * Output: 2
 * Explanation: In this example, the first mouse eats the 0^th (0-indexed) and 1^st types of cheese, and the second mouse does not eat any cheese.
 * The total points are 1 + 1 = 2.
 * It can be proven that 2 is the maximum total points that the mice can achieve.
 *
 *  
 * Constraints:
 *
 *     1 <= n == reward1.length == reward2.length <= 10^5
 *     1 <= reward1[i], reward2[i] <= 1000
 *     0 <= k <= n
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn mice_and_cheese(mut reward: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32 {
		let ans: i32 = reward2.iter().sum();
		reward.iter_mut().zip(reward2).for_each(|(i, j)| *i -= j);
		reward.sort_unstable_by_key(|v| -v);
		ans + reward[..k as usize].iter().sum::<i32>()
	}
}

// submission codes end
