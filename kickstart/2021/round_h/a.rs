use std::fmt::Debug;
use std::io::{stdin,stdout,Write};
use std::str::FromStr;

fn solve(test_case: usize) -> String {
	// println!("Solving test case {}", test_case);
	let word = read_line();
	let word = word.trim().to_string();
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
