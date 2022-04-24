use std::fmt::Debug;
use std::io::{stdin,stdout,Write};
use std::str::FromStr;
use std::collections::{
	HashSet,
	HashMap,
	VecDeque,
};
use std::f64::consts::PI;

fn solve(test_case: usize) -> u32 {
	// println!("Solving test case {}", test_case);
	let N: usize = get_int();
	let mut tmp: Vec<u32> = read_n_int();
	let mut pp: VecDeque<u32> = VecDeque::with_capacity(N);
	for v in tmp { pp.push_back(v); }
	let mut ans = 1;
	let mut max = if pp[0] < pp[N - 1] {
		pp.pop_front().unwrap()
	} else {
		pp.pop_back().unwrap()
	};

	while !pp.is_empty() {
		if pp.len() == 1 {
			if pp[0] >= max {
				ans += 1;
			}
			break;
		}


		let mut tmp = if pp[0] < pp[pp.len() - 1] {
			pp.pop_front().unwrap()
		} else {
			pp.pop_back().unwrap()
		};
		if tmp >= max {
			ans += 1;
			max = tmp;
		}
	}

	ans
}

fn main() {
	let T = get_int::<usize>();
	for t in 1..=T {
		println!("Case #{}: {:.6}", t, solve(t));
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
