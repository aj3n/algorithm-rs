/**
 * [1479] Construct Target Array With Multiple Sums
 *
 * Given an array of integers target. From a starting array, A consisting of all 1's, you may perform the following procedure :
 *
 *     let x be the sum of all elements currently in your array.
 *     choose index i, such that 0 <= i < target.size and set the value of A at index i to x.
 *     You may repeat this procedure as many times as needed.
 *
 * Return True if it is possible to construct the target array from A otherwise return False.
 *  
 * Example 1:
 *
 * Input: target = [9,3,5]
 * Output: true
 * Explanation: Start with [1, 1, 1]
 * [1, 1, 1], sum = 3 choose index 1
 * [1, 3, 1], sum = 5 choose index 2
 * [1, 3, 5], sum = 9 choose index 0
 * [9, 3, 5] Done
 *
 * Example 2:
 *
 * Input: target = [1,1,1,2]
 * Output: false
 * Explanation: Impossible to create target array from [1,1,1,1].
 *
 * Example 3:
 *
 * Input: target = [8,5]
 * Output: true
 *
 *  
 * Constraints:
 *
 *     N == target.length
 *     1 <= target.length <= 5 * 10^4
 *     1 <= target[i] <= 10^9
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn is_possible(target: Vec<i32>) -> bool {
		if target.iter().any(|i| *i <= 0) {
			return false;
		}
		if target.len() == 1 && target[0] != 1 {
			return false;
		}

		use std::collections::BinaryHeap;
		let mut bh = BinaryHeap::from(target);

		let mut h = *bh.peek().unwrap();
		if bh.iter().skip(1).any(move |&i| {
			h -= i;
			h < 0
		}) {
			return false;
		}
		let mut sum: i32 = bh.iter().sum();

		loop {
			let i = bh.pop().unwrap();
			if i == 1 {
				break true;
			}

			let other_sum = sum - i;
			if other_sum > i {
				break false;
			} else if other_sum == 1 {
				break true;
			}

			let j = i % other_sum;
			sum += j - i;

			if j == 0 {
				break false;
			} else {
				bh.push(j);
			}
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1479() {
		assert_eq!(Solution::is_possible(vec![9, 3, 5]), true);
		assert_eq!(Solution::is_possible(vec![1, 1, 1, 2]), false);
		assert_eq!(Solution::is_possible(vec![1, 1, 2]), false);
		assert_eq!(Solution::is_possible(vec![8, 5]), true);
		assert_eq!(Solution::is_possible(vec![1, 1000000000]), true);
	}
}
