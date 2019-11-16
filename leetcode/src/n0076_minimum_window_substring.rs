/**
 * [76] Minimum Window Substring
 *
 * Given a string S and a string T, find the minimum window in S which will contain all the characters in T in complexity O(n).
 *
 * Example:
 *
 *
 * Input: S = "ADOBECODEBANC", T = "ABC"
 * Output: "BANC"
 *
 *
 * Note:
 *
 *
 * 	If there is no such window in S that covers all characters in T, return the empty string "".
 * 	If there is such window, you are guaranteed that there will always be only one unique minimum window in S.
 *
 *
 */
pub struct Solution {}

// submission codes start here

// 太丑了
impl Solution {
	pub fn min_window(s: String, t: String) -> String {
		if s.len() == 0 || s.len() < t.len() {
			return Default::default();
		}
		let mut res = String::new();
		let mut cmap = vec![0; 128];
		let mut imap = vec![false; 128];
		for c in t.bytes().map(|c| c as usize) {
			cmap[c] = cmap[c] + 1;
			imap[c] = true;
		}

		let mut i = 0;
		let mut start = 0;
		let mut minl = std::usize::MAX;
		for (j, c) in s.bytes().enumerate() {
			let c = c as usize;
			if imap[c] {
				cmap[c] = cmap[c] - 1;
			}
			while (!imap[s.as_bytes()[i] as usize] || cmap[s.as_bytes()[i] as usize] < 0) && i < j {
				if imap[s.as_bytes()[i] as usize] {
					cmap[s.as_bytes()[i] as usize] = cmap[s.as_bytes()[i] as usize] + 1;
				}
				i = i + 1;
			}
			if cmap.iter().fold(true, |acc, x| acc && *x <= 0) {
				if j - i < minl {
					start = i;
					minl = j - i + 1;
				}
			}
		}
		if minl > s.len() {
			return Default::default();
		}
		s[start..start + minl].to_owned()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_76() {
		assert_eq!(
			Solution::min_window("ADOBECODEBANCQGHJMNL".to_owned(), "ABC".to_owned()),
			"BANC".to_owned()
		);
	}
}
