use crate::*;
use std::iter::FromIterator;
pub struct Iter<'a, T: std::cmp::Ord + 'a> {
	cur_node: Option<NonNull<Node<T>>>,
	marker_: std::marker::PhantomData<&'a Node<T>>,
}

impl<'a, T: std::cmp::Ord + 'a> Iterator for Iter<'a, T> {
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		if self.cur_node.is_none() {
			None
		} else {
			self.cur_node.map(|node| unsafe {
				let node = &*node.as_ptr();
				self.cur_node = node.nexts[0];
				&node.key
			})
		}
	}
}

impl<'a, T: std::cmp::Ord + 'a> IntoIterator for &'a SkipList<T> {
	type IntoIter = Iter<'a, T>;
	type Item = &'a T;

	fn into_iter(self) -> Self::IntoIter {
		Iter {
			cur_node: self.head.nexts[0],
			marker_: std::marker::PhantomData,
		}
	}
}

impl<T: std::cmp::Ord> SkipList<T> {
	pub fn iter(&self) -> Iter<'_, T> { self.into_iter() }
}

pub struct IntoIter<T: std::cmp::Ord> {
	cur_node: Option<Box<Node<T>>>,
}

impl<T: std::cmp::Ord> Iterator for IntoIter<T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		let cur_node = self.cur_node.take()?;

		unsafe {
			self.cur_node = cur_node.nexts[0].map(|ptr| Box::from_raw(ptr.as_ptr()));
		}

		Some(cur_node.key)
	}
}

impl<T: std::cmp::Ord> IntoIterator for SkipList<T> {
	type Item = T;
	type IntoIter = IntoIter<T>;

	fn into_iter(mut self) -> Self::IntoIter {
		unsafe {
			IntoIter {
				cur_node: self.head.nexts[0]
					.take()
					.map(|ptr| Box::from_raw(ptr.as_ptr())),
			}
		}
	}
}

impl<T: std::cmp::Ord + Default> FromIterator<T> for SkipList<T> {
	fn from_iter<I: IntoIterator<Item = T>>(it: I) -> Self {
		let mut list = SkipList::default();
		list.extend(it);
		list
	}
}

impl<T: std::cmp::Ord> std::iter::Extend<T> for SkipList<T> {
	fn extend<I: IntoIterator<Item = T>>(&mut self, it: I) {
		for i in it {
			self.insert(i);
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn test_iter() {
		const TEST_RANGE: std::ops::Range<i32> = -1_000_000..1_000_000;
		let mut list = SkipList::default();
		for i in TEST_RANGE {
			list.insert(i);
		}
		assert!((TEST_RANGE).eq(list.into_iter()));
	}
}
