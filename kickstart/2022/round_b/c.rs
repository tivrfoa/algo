use std::fmt::Debug;
use std::io::{stdin,stdout,Write};
use std::str::FromStr;
use std::collections::{
	HashSet,
	HashMap,
};
use std::f64::consts::PI;

#[derive(Debug)]
struct State {
	l: usize,
	r: usize,
	operations: u32,
	v: u32,
}

fn solve(test_case: usize) -> u32 {
	// println!("Solving test case {}", test_case);
	let tmp: Vec<u32> = read_n_int();
	let N = tmp[0] as usize;
	let D = tmp[1] - 1;
	let vv: Vec<u32> = read_n_int();

	let mut ok = true;
	for i in 0..N {
		if vv[i] != 0 { ok = false; }
	}
	if ok { return 0; }

	// let mut min = u32::MAX; // Google version doesn't have this
	let mut min: u32 = 4294967295;
	let mut states = vec![];
	let mut l = 0;
	while l < N-1 {
		let mut r = l;
		for j in l+1..N {
			if vv[l] != vv[j] {
				break;
			}
			r = j;
		}
		if l == 0 && r == N - 1 {
			return vv[l];
		}

		states.push(State { l, r, operations: 0, v: vv[l] });
		l = r + 1;
	}
	// eprintln!("{:?}", states);

	while !states.is_empty() {
		let s = states.pop().unwrap();
		// eprintln!("{:?}", s);
		if s.v == 0 {
			// check if left and right are also zeros
			// left
			let mut ok = true;
			for i in 0..s.l {
				if vv[i] != 0 { ok = false; }
			}
			if ok {
				// check right
				for i in s.r+1..N {
					if vv[i] != 0 { ok = false; }
				}
			}
			if ok && s.operations < min {
				min = s.operations;
				continue;
			}
		}

		if s.l == 0 && s.r == N - 1 {
			let tmp = s.operations + s.v.min(D - s.v + 1);
			if tmp < min {
				min = tmp;
			}
			continue;
		}

		// left
		if s.l > 0 {
			let mut operations = {
				let a = if vv[s.l] >= vv[s.l - 1] {
					vv[s.l] - vv[s.l - 1]
				} else {
					vv[s.l - 1] - vv[s.l]
				};
				a.min(D - vv[s.l] + 1 + vv[s.l - 1])
			};
			operations += s.operations;
			// find new l
			let mut new_l = s.l - 1;
			while new_l > 0 && vv[new_l] == vv[new_l - 1] {
				new_l -= 1;
			}

			// check if value in new_r == new_l
			let mut new_r = s.r;
			if s.r < N - 1 && vv[s.r + 1] == vv[s.l - 1] {
				// find new r
				let mut new_r = s.r + 1;
				while new_r < N - 1 && vv[new_r] == vv[new_r + 1] {
					new_r += 1;
				}
				states.push(State { l: new_l, r: new_r, operations, v: vv[s.l - 1] });
				continue;
			} else {
				states.push(State { l: new_l, r: s.r, operations, v: vv[s.l - 1] });
			}
		}

		// right
		if s.r < N - 1 {
			let mut operations = {
				let a = if vv[s.r] >= vv[s.r + 1] {
					vv[s.r] - vv[s.r + 1]
				} else {
					vv[s.r + 1] - vv[s.r]
				};
				a.min(D - vv[s.r] + 1 + vv[s.r + 1])
			};
			operations += s.operations;
			// find new r
			let mut new_r = s.r + 1;
			while new_r < N - 1 && vv[new_r] == vv[new_r + 1] {
				new_r += 1;
			}
			states.push(State { l: s.l, r: new_r, operations, v: vv[s.r + 1] });
		}
	}

	min
}

fn main() {
	let T = get_int::<usize>();
	for t in 1..=T {
		println!("Case #{}: {}", t, solve(t));
	}
}

fn read_n_int<T: FromStr>() -> Vec<T>
		where <T as FromStr>::Err: Debug {
	let line = read_line();
	// println!("Parsing line: {}", line);
	line.split_whitespace()
			.map(|i| i.parse().expect("to work"))
			.collect()
}

fn read_line() -> String {
	let mut line = String::new();
    stdin().read_line(&mut line).expect("Did not enter a correct string");
	line
}

fn get_int<T: FromStr>() -> T
		where <T as FromStr>::Err: Debug {
	let line = read_line();
	let t: T = line.trim().parse().expect(&line);
	t
}
