use std::fmt::Debug;
use std::io::{stdin,stdout,Write};
use std::str::FromStr;
use std::collections::{
	HashSet,
	HashMap,
	VecDeque,
};
use std::f64::consts::PI;

fn solve(test_case: usize) -> i64 {
	// println!("Solving test case {}", test_case);
	let mut tmp: Vec<usize> = read_n_int();
	let N = tmp[0];
	let P = tmp[1];
	let mut pp: Vec<Vec<i64>> = vec![Vec::with_capacity(P); N];
	for i in 0..N {
		let mut tmp: Vec<i64> = read_n_int();
		pp[i].append(&mut tmp);
	}
	let mut dp: Vec<[i64; 2]> = Vec::with_capacity(N + 1);
	let mut l0 = 0;
	let mut l1 = 0;
	dp.push([l0, l1]);

	for i in 0..N {
		pp[i].sort();
		dp.push([(dp[i][0] + (l0 - pp[i][0]).abs() + pp[i][P - 1] - pp[i][0]).min(
				    dp[i][1] + (l1 - pp[i][0]).abs() + pp[i][P - 1] - pp[i][0]),
                   (dp[i][0] + (l0 - pp[i][P - 1]).abs() + pp[i][P - 1] - pp[i][0]).min(
				    dp[i][1] + (l1 - pp[i][P - 1]).abs() + pp[i][P - 1] - pp[i][0])]);
		l0 = pp[i][P - 1];
		l1 = pp[i][0];
	}

	dp[N][0].min(dp[N][1])
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
