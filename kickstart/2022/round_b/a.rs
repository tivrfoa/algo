use std::fmt::Debug;
use std::io::{stdin,stdout,Write};
use std::str::FromStr;
use std::collections::{
	HashSet,
	HashMap,
};
use std::f64::consts::PI;

fn solve(test_case: usize) -> f64 {
	// println!("Solving test case {}", test_case);
	let tmp: Vec<usize> = read_n_int();
	let mut r = tmp[0];
	let A = tmp[1];
	let B = tmp[2];

	let mut sum: f64 = PI * (r * r) as f64;

	loop {
		r *= A;
		sum += PI * (r * r) as f64;

		r /= B;
		if r == 0 { break; }
		sum += PI * (r * r) as f64;
	}

	sum
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
