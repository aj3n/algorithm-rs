use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand;
use rbtree::*;
use std::collections::BTreeSet;
const TEST_RANGE1: std::ops::Range<i32> = 0..1_000_000;
const TEST_RANGE2: std::ops::Range<i32> = 0..10_000_i32;

fn bench_iter(c: &mut Criterion) {
	let mut test_by_range = |test_range: _, promt: &str| {
		let mut rbt: RBTree<_> = Default::default();
		for _ in test_range {
			rbt.insert(rand::random::<i32>());
		}
		c.bench_function(promt, |b| {
			 b.iter(|| {
				  for i in &rbt {
					  black_box(i);
				  }
			  })
		 });
	};
	test_by_range(TEST_RANGE1, "1_000_000 iter");
	test_by_range(TEST_RANGE2, "10_000 iter");
}

fn bench_std_iter(c: &mut Criterion) {
	let mut test_by_range = |test_range: std::ops::Range<i32>, promt: &str| {
		let mut bt: BTreeSet<_> = BTreeSet::new();
		for _ in test_range {
			bt.insert(rand::random::<i32>());
		}
		c.bench_function(promt, |b| {
			 b.iter(|| {
				  for i in &bt {
					  black_box(i);
				  }
			  })
		 });
	};

	test_by_range(TEST_RANGE1, "1_000_000 BTreeSet");
	test_by_range(TEST_RANGE2, "10_000 BTreeSet");
}

criterion_group!(benches, bench_iter, bench_std_iter);
criterion_main!(benches);