/**
 * [705] Design HashSet
 *
 * Design a HashSet without using any built-in hash table libraries.
 * Implement MyHashSet class:
 *
 *     void add(key) Inserts the value key into the HashSet.
 *     bool contains(key) Returns whether the value key exists in the HashSet or not.
 *     void remove(key) Removes the value key in the HashSet. If key does not exist in the HashSet, do nothing.
 *
 *  
 * <strong class="example">Example 1:
 *
 * Input
 * ["MyHashSet", "add", "add", "contains", "contains", "add", "contains", "remove", "contains"]
 * [[], [1], [2], [1], [3], [2], [2], [2], [2]]
 * Output
 * [null, null, null, true, false, null, true, null, false]
 * Explanation
 * MyHashSet myHashSet = new MyHashSet();
 * myHashSet.add(1);      // set = [1]
 * myHashSet.add(2);      // set = [1, 2]
 * myHashSet.contains(1); // return True
 * myHashSet.contains(3); // return False, (not found)
 * myHashSet.add(2);      // set = [1, 2]
 * myHashSet.contains(2); // return True
 * myHashSet.remove(2);   // set = [1]
 * myHashSet.contains(2); // return False, (already removed)
 *  
 * Constraints:
 *
 *     0 <= key <= 10^6
 *     At most 10^4 calls will be made to add, remove, and contains.
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::BTreeSet;
struct MyHashSet {
	map: BTreeSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
	fn new() -> Self {
		Self {
			map: BTreeSet::new(),
		}
	}

	fn add(&mut self, key: i32) { self.map.insert(key); }

	fn remove(&mut self, key: i32) { self.map.remove(&key); }

	fn contains(&self, key: i32) -> bool { self.map.contains(&key) }
}
