use std::cmp::Ordering;

pub fn merge_sort<T>(nums: &mut [T])
where
	T: std::cmp::Ord + Clone,
{
	if nums.len() > 32 {
		let (lower, upper) = nums.split_at_mut(nums.len() / 2);
		merge_sort(lower);
		merge_sort(upper);
		let lower: Vec<_> = lower.to_vec();
		merge_into(nums, lower.into_iter());
	} else {
		insert_sort(nums)
	}
}

pub fn bottomup_sort<T>(nums: &mut [T])
where
	T: std::cmp::Ord + Clone,
{
	if nums.len() <= 1 {
		return;
	}
	let l = nums.len();
	let mut tmp_vec = Vec::with_capacity(l / 2);
	for blk in (0..).map(|i| 1 << i).take_while(|&i| i < l) {
		for chunk in nums
			.chunks_mut(blk * 2)
			.take_while(|chunk| chunk.len() > blk)
		{
			tmp_vec.extend(chunk[..blk].iter().cloned());
			merge_into(chunk, tmp_vec.drain(..));
		}
	}
}

fn merge_into<T: std::cmp::Ord, I>(nums: &mut [T], nums_lower: I)
where
	I: ExactSizeIterator<Item = T>,
{
	let mut i = nums_lower.len();
	let mut nums_lower = nums_lower.peekable();
	for k in 0..nums.len() {
		match (nums.get(i), nums_lower.peek()) {
			(_, None) => break,
			(Some(n), Some(p)) if n < p => {
				nums.swap(i, k);
				i += 1;
			}
			(_, Some(_)) => nums[k] = nums_lower.next().unwrap(),
		}
	}
}

pub fn insert_sort<T>(nums: &mut [T])
where
	T: std::cmp::Ord,
{
	for i in 2..(nums.len() + 1) {
		if let [l @ .., r] = &mut nums[..i] {
			let mut prev = r;
			for b in l.iter_mut().rev() {
				if b < prev {
					break;
				}
				std::mem::swap(prev, b);
				prev = b;
			}
		}
	}
}

pub fn select_sort<T>(nums: &mut [T])
where
	T: std::cmp::Ord,
{
	if nums.len() <= 1 {
		return;
	}
	for i in 0..(nums.len() - 1) {
		if let [l, r @ ..] = &mut nums[i..] {
			if let Some(min) = r.iter_mut().min() {
				if min < l {
					std::mem::swap(l, min)
				}
			}
		}
	}
}

pub fn quick_sort<T>(v: &mut [T])
where
	T: std::cmp::Ord,
{
	if let [o, rest @ ..] = v {
		let (mut i, mut j, mut k) = (0, 0, rest.len());
		while j < k {
			match rest[j].cmp(o) {
				Ordering::Less => {
					rest.swap(j, i);
					i += 1;
					j += 1;
				}
				Ordering::Greater => {
					rest.swap(j, k - 1);
					k -= 1;
				}
				Ordering::Equal => j += 1,
			}
		}
		v.swap(0, i);
		quick_sort(&mut v[..i]);
		quick_sort(&mut v[k + 1..]);
	}
}

pub fn heap_sort<T>(nums: &mut [T])
where
	T: std::cmp::Ord,
{
	fn shift_down<T: std::cmp::Ord>(tree: &mut [T], mut i: usize) {
		let child_idxs = |i: usize| (i * 2 + 1, i * 2 + 2);
		while i < tree.len() {
			let (l, r) = child_idxs(i);
			let larger_child = match (tree.get(l), tree.get(r)) {
				(None, _) => return,
				(Some(lv), Some(rv)) if lv < rv => r,
				(Some(_), _) => l,
			};
			if tree.get(i) < tree.get(larger_child) {
				tree.swap(i, larger_child);
				i = larger_child;
			} else {
				return;
			}
		}
	}

	for i in (0..nums.len() / 2).rev() {
		shift_down(nums, i);
	}

	for i in (1..nums.len()).rev() {
		nums.swap(0, i);
		shift_down(&mut nums[0..i], 0);
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use rand::prelude::*;
	#[test]
	fn it_works() {
		check_sort(quick_sort::<i32>);
		println!("quick sort test ok!");

		check_sort(merge_sort::<i32>);
		println!("merge sort test ok!");

		check_sort(bottomup_sort::<i32>);
		println!("bottomup sort test ok!");

		check_sort(select_sort::<i32>);
		println!("select sort test ok!");

		check_sort(insert_sort::<i32>);
		println!("insert sort test ok!");

		check_sort(heap_sort::<i32>);
		println!("heap sort test ok!");
	}

	fn check_sort(sort_fn: fn(&mut [i32])) {
		let mut nums: Vec<i32> = (1..10000).collect();
		nums.extend(1..100);
		nums.shuffle(&mut rand::thread_rng());
		sort_fn(&mut nums);
		assert!(nums.windows(2).all(|w| w[0] <= w[1]));
	}
}
