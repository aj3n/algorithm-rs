/**
 * [341] Flatten Nested List Iterator
 *
 * You are given a nested list of integers nestedList. Each element is either an integer or a list whose elements may also be integers or other lists. Implement an iterator to flatten it.
 * Implement the NestedIterator class:
 *
 *     NestedIterator(List<NestedInteger> nestedList) Initializes the iterator with the nested list nestedList.
 *     int next() Returns the next integer in the nested list.
 *     boolean hasNext() Returns true if there are still some integers in the nested list and false otherwise.
 *
 * Your code will be tested with the following pseudocode:
 *
 * initialize iterator with nestedList
 * res = []
 * while iterator.hasNext()
 *     append iterator.next() to the end of res
 * return res
 *
 * If res matches the expected flattened list, then your code will be judged as correct.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nestedList = [[1,1],2,[1,1]]
 * Output: [1,1,2,1,1]
 * Explanation: By calling next repeatedly until hasNext returns false, the order of elements returned by next should be: [1,1,2,1,1].
 *
 * <strong class="example">Example 2:
 *
 * Input: nestedList = [1,[4,[6]]]
 * Output: [1,4,6]
 * Explanation: By calling next repeatedly until hasNext returns false, the order of elements returned by next should be: [1,4,6].
 *
 *  
 * Constraints:
 *
 *     1 <= nestedList.length <= 500
 *     The values of the integers in the nested list is in the range [-10^6, 10^6].
 *
 */
pub struct Solution {}

// submission codes start here

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
	Int(i32),
	List(Vec<NestedInteger>),
}
struct NestedIterator {
	v: Vec<i32>,
}

impl NestedIterator {
	fn new(list: Vec<NestedInteger>) -> Self {
		let mut v = vec![];
		Self::flatten(&mut v, list);
		Self { v }
	}
	fn flatten(it: &mut Vec<i32>, list: Vec<NestedInteger>) {
		use NestedInteger::*;
		for i in list.into_iter().rev() {
			match i {
				Int(i) => it.push(i),
				List(v) => Self::flatten(it, v),
			}
		}
	}
	fn next(&mut self) -> i32 { self.v.pop().unwrap() }
	fn has_next(&self) -> bool { !self.v.is_empty() }
}

// submission codes end
