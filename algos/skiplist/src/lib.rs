use arrayvec::ArrayVec;
use std::{cell::Cell, ptr::NonNull};
mod iter;

const MAX_HEIGHT: usize = 32;
type NodeRef<T> = Cell<Option<NonNull<Node<T>>>>;
struct Node<T> {
	key: T,
	nexts: Box<[NodeRef<T>]>,
}

impl<'a, T: std::cmp::Ord + 'a> Node<T> {
	fn new(key: T) -> Node<T> {
		let height = (rand::random::<i32>().trailing_ones() as usize + 1).min(MAX_HEIGHT);
		Node {
			key,
			nexts: vec![Cell::new(None); height].into_boxed_slice(),
		}
	}

	unsafe fn find_ge<'b: 'a>(&'b self, key: &'_ T, max_height: usize) -> Option<&'b Node<T>> {
		let mut co = self;
		for lev in (0..max_height).rev() {
			while let Some(no) = co.nexts[lev].get() {
				if no.as_ref().key >= *key {
					break;
				}
				co = &*no.as_ptr();
			}
		}
		co.nexts[0].get().map(|co| &*co.as_ptr())
	}

	unsafe fn find_ge_with_prevs<'b: 'a>(
		&'b self,
		key: &'_ T,
		max_height: usize,
		prev: &mut [*const Node<T>],
	) -> Option<&'b mut Node<T>> {
		let mut co = self;
		for lev in (0..max_height).rev() {
			while let Some(no) = co.nexts[lev].get() {
				if no.as_ref().key >= *key {
					break;
				}
				co = no.as_ref();
			}

			prev[lev] = co as *const _;
		}
		co.nexts[0].get().map(|co| &mut *co.as_ptr())
	}
}

pub struct SkipList<T> {
	head: Node<T>,
	max_height: usize,
	len: usize,
}
impl<T> SkipList<T> {
	pub fn new(some_key: T) -> SkipList<T> {
		SkipList {
			head: Node {
				key: some_key,
				nexts: vec![Cell::new(None); MAX_HEIGHT].into_boxed_slice(),
			},
			max_height: 1,
			len: 0,
		}
	}
}

impl<T: std::cmp::Ord> SkipList<T> {
	pub fn len(&self) -> usize { self.len }

	pub fn is_empty(&self) -> bool { self.len == 0 }

	pub fn find(&self, key: &T) -> Option<()> {
		unsafe {
			if self.head.find_ge(key, self.max_height - 1)?.key == *key {
				Some(())
			} else {
				None
			}
		}
	}

	// FIXME: insertion is terribly slow
	pub fn insert(&mut self, key: T) {
		unsafe {
			//is there a simple way to init prev?
			let mut head_iter = std::iter::repeat(&self.head as *const _);
			let mut prevs: ArrayVec<[*const Node<T>; MAX_HEIGHT]> =
				head_iter.by_ref().take(self.max_height).collect();

			let so = self
				.head
				.find_ge_with_prevs(&key, self.max_height, &mut prevs);
			if so.is_none() || so.unwrap().key > key {
				let new_node = Box::new(Node::new(key));
				let height = new_node.nexts.len();
				let new_node = NonNull::new_unchecked(Box::into_raw(new_node));

				if height > self.max_height {
					prevs.extend(head_iter.take(height - self.max_height));
					self.max_height = height;
				}

				for (lev, (prev, new_node_next)) in prevs
					.into_iter()
					.zip(new_node.as_ref().nexts.iter())
					.enumerate()
				{
					new_node_next.set((*prev).nexts[lev].replace(Some(new_node)));
				}

				self.len += 1;
			}
		}
	}

	pub fn remove(&mut self, key: &T) -> Option<T> {
		unsafe {
			let mut prev: ArrayVec<[*const Node<T>; MAX_HEIGHT]> =
				std::iter::repeat(&self.head as *const _)
					.take(self.max_height)
					.collect();
			let co = self
				.head
				.find_ge_with_prevs(key, self.max_height, &mut prev)?;
			if co.key == *key {
				let height = co.nexts.len();
				for (lev, (prev, co_next)) in prev
					.into_iter()
					.zip(co.nexts.iter_mut())
					.take(height)
					.enumerate()
				{
					(*prev).nexts[lev].set(co_next.take());
				}

				let to_ret = Some(Box::from_raw(co as *mut Node<T>).key);
				while self.head.nexts[self.max_height - 1].get().is_none() && self.max_height > 1 {
					self.max_height -= 1;
				}
				self.len -= 1;
				to_ret
			} else {
				None
			}
		}
	}
}

impl<T> Drop for SkipList<T> {
	fn drop(&mut self) {
		unsafe {
			let mut cur_node = self.head.nexts[0].take();
			while let Some(mut co) = cur_node {
				cur_node = co.as_mut().nexts[0].take();
				drop(Box::from_raw(co.as_ptr()));
			}
		}
	}
}

impl<T: Default> Default for SkipList<T> {
	fn default() -> Self { Self::new(Default::default()) }
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn it_works() {
		for _ in 0..1 {
			let mut list = SkipList::default();
			for i in -100..100 {
				list.insert(i);
			}
			check_ok(&list);
			for i in -50..50 {
				assert!(list.find(&i).is_some());
				assert_eq!(list.remove(&i), Some(i));
				assert!(list.find(&i).is_none());
			}
			check_ok(&list);
			let v: Vec<_> = (0..100).collect();
			let mut list = SkipList::new(&v[0]);
			for i in v.iter() {
				list.insert(i);
			}

			for i in v.iter().skip(20) {
				assert!(list.find(&i).is_some());
			}
		}
	}

	fn check_ok<T: std::cmp::Ord>(list: &SkipList<T>) {
		unsafe {
			// check key sorted and continuous in every level
			let mut prevs: Vec<_> = list.head.nexts.iter().filter_map(Cell::get).collect();
			let mut link_cnt = 0;
			let mut link_dist = [0; MAX_HEIGHT];
			if let Some(mut no) = list.head.nexts[0].get() {
				while let Some(o) = no.as_ref().nexts[0].get() {
					for (i, prev) in prevs.iter_mut().enumerate().take(o.as_ref().nexts.len()) {
						assert!(prev.as_ref().key <= o.as_ref().key);
						// continuous
						if prev.as_ref().key < o.as_ref().key {
							assert!(prev.as_ref().nexts[i].get() == Some(o));
						}
						*prev = o;
					}
					link_cnt += no.as_ref().nexts.len();
					link_dist[no.as_ref().nexts.len()] += 1;
					no = o;
				}
			}
			let cnt: i32 = link_dist.iter().copied().sum();
			println!(
				"Checks Ok! (total link: {}, total node: {} , cnt: {}",
				link_cnt,
				list.len(),
				cnt
			);
			println!("link distribution: {:?}", link_dist);
		}
	}

	impl<T: std::cmp::Ord + std::fmt::Display> std::fmt::Display for SkipList<T> {
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			let mut no = self.head.nexts[0].get();
			unsafe {
				while let Some(o) = no {
					write!(f, "{:4}:", o.as_ref().key)?;
					for oo in o.as_ref().nexts.iter().map(Cell::get) {
						match oo {
							Some(o) => write!(f, "{:4},", o.as_ref().key)?,
							None => write!(f, "____,")?,
						};
					}
					writeln!(f)?;
					no = o.as_ref().nexts[0].get();
				}
			}
			Ok(())
		}
	}
}
