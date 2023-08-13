/**
 * [2272] Substring With Largest Variance
 *
 * The variance of a string is defined as the largest difference between the number of occurrences of any 2 characters present in the string. Note the two characters may or may not be the same.
 * Given a string s consisting of lowercase English letters only, return the largest variance possible among all substrings of s.
 * A substring is a contiguous sequence of characters within a string.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "aababbb"
 * Output: 3
 * Explanation:
 * All possible variances along with their respective substrings are listed below:
 * - Variance 0 for substrings "a", "aa", "ab", "abab", "aababb", "ba", "b", "bb", and "bbb".
 * - Variance 1 for substrings "aab", "aba", "abb", "aabab", "ababb", "aababbb", and "bab".
 * - Variance 2 for substrings "aaba", "ababbb", "abbb", and "babb".
 * - Variance 3 for substring "babbb".
 * Since the largest possible variance is 3, we return it.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "abcde"
 * Output: 0
 * Explanation:
 * No letter occurs more than once in s, so the variance of every substring is 0.
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 10^4
 *     s consists of lowercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn largest_variance(s: String) -> i32 {
		let chars = s.bytes().collect::<std::collections::HashSet<_>>();
		let mut ans = 0;
		for &a in &chars {
			for &b in chars.iter().filter(|b| **b != a) {
				let (mut cnt, mut fb, mut has_b) = (0, false, false);
				for c in s.bytes() {
					cnt += (c == a) as i32;
					if c == b {
						has_b = true;
						if fb && cnt > 0 {
							fb = false;
						} else {
							cnt -= 1;
							if cnt < 0 {
								fb = true;
								cnt = -1;
							}
						}
					}
					if has_b {
						ans = ans.max(cnt);
					}
				}
			}
		}
		ans
	}
}

// submission codes end
