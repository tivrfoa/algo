// Based on https://github.com/williamfiset/Algorithms/blob/master/src/main/java/com/williamfiset/algorithms/datastructures/fenwicktree/FenwickTreeRangeQueryPointUpdate.java

struct FenwickTreeRangeQueryPointUpdate {
	n: usize,
	tree: Vec<i64>, // TODO change to generic
}

impl FenwickTreeRangeQueryPointUpdate {

	fn new(size: usize) -> Self {
		Self {
			n: size,
			tree: vec![0; size + 1],
		}
	}

	fn from_values(values: &Vec<i64>) -> Self {
		let n = values.len() + 1;
		let mut tree = Vec::with_capacity(n);
		tree.push(0);
		tree.append(&mut values.clone());

		for i in 1..n {
			let parent = i + Self::lsb(i);
			if parent < n {
				tree[parent] += tree[i];
			}
		}

		Self {
			n,
			tree,
		}
	}

	fn lsb(i: usize) -> usize {
		let i = i as i64;
		(i & -i) as usize
	}

	fn prefix_sum(&self, mut i: usize) -> i64 {
		let mut sum = 0;
		while i != 0 {
			sum += self.tree[i];
			i &= !Self::lsb(i);
		}
		sum
	}

	fn sum(&self, left: usize, right: usize) -> i64 {
		assert!(left <= right);
		self.prefix_sum(right) - self.prefix_sum(left - 1)
	}

	fn get(&self, i: usize) -> i64 {
		self.sum(i, i)
	}

	fn add(&mut self, mut i: usize, v: i64) {
		while i < self.n {
			self.tree[i] += v;
			i += Self::lsb(i);
		}
	}

	fn set(&mut self, i: usize, v: i64) {
		self.add(i, v - self.sum(i, i));
	}
}

#[test]
fn test_interval_sum_positive_values() {
	let ar: Vec<i64> = (1..=6).collect();
	let ft = FenwickTreeRangeQueryPointUpdate::from_values(&ar);

	assert_eq!(ft.sum(1, 6), 21);
	assert_eq!(ft.sum(1, 5), 15);
	assert_eq!(ft.sum(1, 4), 10);
	assert_eq!(ft.sum(1, 3), 6);
	assert_eq!(ft.sum(1, 2), 3);
	assert_eq!(ft.sum(1, 1), 1);

	assert_eq!(ft.sum(3, 4), 7);
	assert_eq!(ft.sum(2, 6), 20);
	assert_eq!(ft.sum(4, 5), 9);
	assert_eq!(ft.sum(6, 6), 6);
}

#[test]
fn test_interval_sum_negative_values() {
	let ar: Vec<i64> = (-6..=-1).rev().collect();
	let ft = FenwickTreeRangeQueryPointUpdate::from_values(&ar);

	assert_eq!(ft.sum(1, 6), -21);
	assert_eq!(ft.sum(1, 5), -15);
	assert_eq!(ft.sum(1, 4), -10);
	assert_eq!(ft.sum(1, 3), -6);
	assert_eq!(ft.sum(1, 2), -3);
	assert_eq!(ft.sum(1, 1), -1);

	assert_eq!(ft.sum(3, 4), -7);
	assert_eq!(ft.sum(2, 6), -20);
	assert_eq!(ft.sum(4, 5), -9);
	assert_eq!(ft.sum(6, 6), -6);
}

#[test]
fn test_add() {
	let ar: Vec<i64> = (1..=6).collect();
	let mut ft = FenwickTreeRangeQueryPointUpdate::from_values(&ar);

	assert_eq!(ft.sum(1, 6), 21);
	assert_eq!(ft.sum(1, 5), 15);
	assert_eq!(ft.sum(1, 4), 10);

	ft.add(4, 5);
	assert_eq!(ft.sum(1, 6), 26);
	assert_eq!(ft.sum(1, 5), 20);
	assert_eq!(ft.sum(1, 4), 15);
	assert_eq!(ft.sum(1, 3), 6);
	assert_eq!(ft.sum(1, 2), 3);
	assert_eq!(ft.sum(1, 1), 1);
}

#[test]
fn test_set() {
	let ar: Vec<i64> = (1..=6).collect();
	let mut ft = FenwickTreeRangeQueryPointUpdate::from_values(&ar);

	assert_eq!(ft.sum(1, 6), 21);
	assert_eq!(ft.sum(1, 5), 15);
	assert_eq!(ft.sum(1, 4), 10);

	ft.set(4, 0);
	assert_eq!(ft.sum(1, 6), 17);
	assert_eq!(ft.sum(1, 5), 11);
	assert_eq!(ft.sum(1, 4), 6);
	assert_eq!(ft.sum(1, 3), 6);
	assert_eq!(ft.sum(1, 2), 3);
	assert_eq!(ft.sum(1, 1), 1);
}
