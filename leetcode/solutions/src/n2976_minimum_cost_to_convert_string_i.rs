/**
 * [2976] Minimum Cost to Convert String I
 *
 * You are given two 0-indexed strings source and target, both of length n and consisting of lowercase English letters. You are also given two 0-indexed character arrays original and changed, and an integer array cost, where cost[i] represents the cost of changing the character original[i] to the character changed[i].
 * You start with the string source. In one operation, you can pick a character x from the string and change it to the character y at a cost of z if there exists any index j such that cost[j] == z, original[j] == x, and changed[j] == y.
 * Return the minimum cost to convert the string source to the string target using any number of operations. If it is impossible to convert source to target, return -1.
 * Note that there may exist indices i, j such that original[j] == original[i] and changed[j] == changed[i].
 *  
 * <strong class="example">Example 1:
 *
 * Input: source = "abcd", target = "acbe", original = ["a","b","c","c","e","d"], changed = ["b","c","b","e","b","e"], cost = [2,5,5,1,2,20]
 * Output: 28
 * Explanation: To convert the string "abcd" to string "acbe":
 * - Change value at index 1 from 'b' to 'c' at a cost of 5.
 * - Change value at index 2 from 'c' to 'e' at a cost of 1.
 * - Change value at index 2 from 'e' to 'b' at a cost of 2.
 * - Change value at index 3 from 'd' to 'e' at a cost of 20.
 * The total cost incurred is 5 + 1 + 2 + 20 = 28.
 * It can be shown that this is the minimum possible cost.
 *
 * <strong class="example">Example 2:
 *
 * Input: source = "aaaa", target = "bbbb", original = ["a","c"], changed = ["c","b"], cost = [1,2]
 * Output: 12
 * Explanation: To change the character 'a' to 'b' change the character 'a' to 'c' at a cost of 1, followed by changing the character 'c' to 'b' at a cost of 2, for a total cost of 1 + 2 = 3. To change all occurrences of 'a' to 'b', a total cost of 3 * 4 = 12 is incurred.
 *
 * <strong class="example">Example 3:
 *
 * Input: source = "abcd", target = "abce", original = ["a"], changed = ["e"], cost = [10000]
 * Output: -1
 * Explanation: It is impossible to convert source to target because the value at index 3 cannot be changed from 'd' to 'e'.
 *
 *  
 * Constraints:
 *
 *     1 <= source.length == target.length <= 10^5
 *     source, target consist of lowercase English letters.
 *     1 <= cost.length == original.length == changed.length <= 2000
 *     original[i], changed[i] are lowercase English letters.
 *     1 <= cost[i] <= 10^6
 *     original[i] != changed[i]
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn minimum_cost(
		source: String,
		target: String,
		original: Vec<char>,
		changed: Vec<char>,
		cost: Vec<i32>,
	) -> i64 {
		let mut nei = vec![vec![i64::MAX; 26]; 26];
		for (i, ne) in (0..).zip(&mut nei) {
			ne[i] = 0;
		}
		for ((f, t), c) in original.into_iter().zip(changed).zip(cost) {
			let cost = &mut nei[(f as u8 - b'a') as usize][(t as u8 - b'a') as usize];
			*cost = (c as i64).min(*cost);
		}
		Self::floyd(&mut nei);
		let mut ans = 0;
		for (s, t) in source.bytes().zip(target.bytes()) {
			let cost = nei[(s - b'a') as usize][(t - b'a') as usize];
			if cost == i64::MAX {
				dbg!(s as char, t as char);
				return -1;
			} else {
				ans += cost;
			}
		}
		ans
	}
	fn floyd(graph: &mut [Vec<i64>]) {
		let n = graph.len();
		for k in 0..n {
			for x in 0..n {
				for y in 0..n {
					graph[x][y] = graph[x][y].min(graph[x][k].saturating_add(graph[k][y]));
				}
			}
		}
	}
}

// submission codes end
