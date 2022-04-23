// Dependent Events

use std::fmt::Debug;
use std::io::{stdin,stdout,Write};
use std::str::FromStr;

const M: usize = 1e9 as usize + 7;

#[derive(Debug)]
struct Event {
	probability: usize,
	parent: usize,
	p_with_parent: usize,
	p_without_parent: usize,
}

impl Event {
	fn new(probability: usize, parent: usize, p_with_parent: usize, p_without_parent: usize) -> Self {
		Self {
			probability,
			parent,
			p_with_parent,
			p_without_parent,
		}
	}
}

fn solve(test_case: usize) -> String {
	// println!("Solving test case {}", test_case);
	let tmp: Vec<usize> = read_n_int();
	let N = tmp[0];
	let Q = tmp[1];
	let mut event_p: usize = get_int();
	let event1 = Event::new(event_p, 0, 0, 0);
	let mut events: Vec<Event> = Vec::with_capacity(N);
	events.push(event1);
	for i in 0..N - 1 {
		let tmp = read_n_int();
		events.push(Event::new(0, tmp[0] - 1, tmp[1], tmp[2]));
	}
	let mut queries: Vec<(usize, usize)> = Vec::with_capacity(Q);
	for i in 0..Q {
		let tmp = read_n_int();
		queries.push((tmp[0], tmp[1]));
	}

	dbg!(events);
	dbg!(queries);

	"todo".to_string()
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
	if b < a {
		let tmp = b;
		b = a;
		a = tmp;
	}

	if a == 0 { return b; }
	gcd(a, b % a)
}

// https://www.geeksforgeeks.org/convert-given-decimal-number-into-an-irreducible-fraction/
fn decimal_to_fraction(number: f64) -> (u64, u64) {
	let int_val = number.floor();
	let f_val = number - int_val;
	let p_val: u64 = 1_000_000; // TODO which number goes here?!
	let f_times_p = (f_val * p_val as f64).round() as u64;
	let gcd_val = gcd(f_times_p, p_val);
	let num = f_times_p / gcd_val;
	let deno = p_val / gcd_val;

	(int_val as u64 * deno + num, deno)
}

fn fast_power(mut base: u64, mut power: u64, MOD: u64) -> u64 {
	let mut result = 1;
	while power > 0 {
		if power & 1 == 1 {
			result = (result * base) % MOD;
		}
		base = (base * base) % MOD;
		power >>= 1;
	}
	result
}

fn modulo_multiplicative_inverse(a: u64, m: u64) -> u64 {
	fast_power(a, m - 2, m)
}

fn main() {
	assert_eq!(decimal_to_fraction(0.048), (6, 125));
	assert_eq!(fast_power(125, 10, 1_000_000_007), 220538953);
	// assert_eq!(modulo_multiplicative_inverse(125, M as u64), 10);
	assert_eq!(modulo_multiplicative_inverse(23, M as u64), 739130440);
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
