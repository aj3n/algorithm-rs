/**
 * [443] String Compression
 *
 * Given an array of characters chars, compress it using the following algorithm:
 * Begin with an empty string s. For each group of consecutive repeating characters in chars:
 *
 *     If the group's length is 1, append the character to s.
 *     Otherwise, append the character followed by the group's length.
 *
 * The compressed string s should not be returned separately, but instead, be stored in the input character array chars. Note that group lengths that are 10 or longer will be split into multiple characters in chars.
 * After you are done modifying the input array, return the new length of the array.
 * You must write an algorithm that uses only constant extra space.
 *  
 * <strong class="example">Example 1:
 *
 * Input: chars = ["a","a","b","b","c","c","c"]
 * Output: Return 6, and the first 6 characters of the input array should be: ["a","2","b","2","c","3"]
 * Explanation: The groups are "aa", "bb", and "ccc". This compresses to "a2b2c3".
 *
 * <strong class="example">Example 2:
 *
 * Input: chars = ["a"]
 * Output: Return 1, and the first character of the input array should be: ["a"]
 * Explanation: The only group is "a", which remains uncompressed since it's a single character.
 *
 * <strong class="example">Example 3:
 *
 * Input: chars = ["a","b","b","b","b","b","b","b","b","b","b","b","b"]
 * Output: Return 4, and the first 4 characters of the input array should be: ["a","b","1","2"].
 * Explanation: The groups are "a" and "bbbbbbbbbbbb". This compresses to "ab12".
 *  
 * Constraints:
 *
 *     1 <= chars.length <= 2000
 *     chars[i] is a lowercase English letter, uppercase English letter, digit, or symbol.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn compress(chars: &mut Vec<char>) -> i32 {
		let (mut cnt, mut prv) = (0, 0 as char);
		for c in std::mem::take(chars) {
			if c == prv {
				cnt += 1;
			} else {
				if cnt > 1 {
					chars.extend(cnt.to_string().chars());
				}
				prv = c;
				chars.push(c);
				cnt = 1;
			}
		}
		if cnt > 1 {
			chars.extend(cnt.to_string().chars());
		}
		chars.len() as i32
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_443() {
		let mut v = vec!['a'; 1000];
		Solution::compress(&mut v);
		assert!(v.iter().copied().eq("a1000".chars()));
		let mut v = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
		assert_eq!(Solution::compress(&mut v), 6);
		assert!(v.iter().copied().eq("a2b2c3".chars()));
	}
}
