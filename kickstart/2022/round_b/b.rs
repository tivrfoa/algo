use std::fmt::Debug;
use std::io::{stdin,stdout,Write};
use std::str::FromStr;
use std::collections::{
	HashSet,
	HashMap,
};


/*

max = 10_000_000_000
max / 2 = 5_000_000_000

*/
fn solve(test_case: usize, memo: &mut HashMap<usize, bool>) -> i32 {
	// println!("Solving test case {}", test_case);
	let A: usize = get_int();
	let mut ans = 1; // 1 is a palindrome

	if A == 1 { return ans; }

	let max = if A <= 4444444444 { A / 2 } else { ((A as f64).sqrt().round() as usize) + 1 };

	for i in 2..=max {
	// for i in 2..=A / 2{
		if A % i == 0 {
			match memo.get(&i) {
				Some(v) => {
					if *v { ans += 1; }
				},
				None => {
					if is_palindromic(i) {
						memo.insert(i, true);
						ans += 1;
					} else {
						memo.insert(i, false);
					}
				}
			}
		}
	}

	if is_palindromic(A as usize) {
		ans + 1
	} else {
		ans
	}
}

fn is_palindromic(i: usize) -> bool {
	let digits: Vec<char> = i.to_string().chars().collect();
	// eprintln!("i = {}, digits = {:?}", i, digits);
	let mut l = 0;
	let mut r = digits.len() - 1;
	while l < r {
		if digits[l] != digits[r] {
			return false;
		}
		l += 1;
		r -= 1;
	}
	//eprintln!("palindrome: {}", i);
	true
}

fn main() {
	let mut memo: HashMap<usize, bool> = HashMap::new();
	let T = get_int::<usize>();
	for t in 1..=T {
		println!("Case #{}: {}", t, solve(t, &mut memo));
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
