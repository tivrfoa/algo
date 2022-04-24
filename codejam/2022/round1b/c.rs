use std::fmt::Debug;
use std::io::{BufWriter, stdin, stdout, StdoutLock, Write};
use std::str::FromStr;
use std::collections::{
	HashSet,
	HashMap,
	VecDeque,
};
use std::f64::consts::PI;

fn query(v: String, out: &mut BufWriter<StdoutLock>) -> u8 {
	// eprintln!("querying: {}", v);
	writeln!(out, "{}", v);
	out.flush();
	let tmp: i32 = get_int();
	if tmp == -1 {
		panic!("bug: v was = {}", v);
	}
	tmp as u8
}

fn solve(test_case: usize, out: &mut BufWriter<StdoutLock>, possible_x: &[Vec<char>; 255], map: &mut HashMap<String, HashMap<u8, Vec<String>>>) {
	// println!("Solving test case {}", test_case);

	let mut v = "00110011".to_string(); // TODO
	for _ in 0..300 {
		let n = query(v.clone(), out);
		if n == 0 {
			eprintln!("Solved test case: {}", test_case);
			return;
		}

		if !map.contains_key(&v) {
			let tmp: Vec<char> = v.chars().collect();
			fill_map(possible_x, map, &tmp);
		}

		v = match map.get_mut(&v) {
			Some(m) => match m.get_mut(&n) {
				Some(vec) => vec[0].clone(),
				/*Some(vec) => match vec.pop() {
					Some(n1) => n1,
					None => { visited.clear(); "00110011".to_string() }
				},*/
				None => panic!("bug 1. No value mapped. v = {}, bits = {}", v, n),
			},
			None => panic!("bug 2. No value mapped. v = {}, bits = {}", v, n),
		};
	}
}

fn fill_map(possible_x: &[Vec<char>; 255], map: &mut HashMap<String, HashMap<u8, Vec<String>>>, v: &Vec<char>) {
	let mut tmp_map: HashMap<u8, Vec<String>> = HashMap::new();
	for i in 0..255 {
		let x = &possible_x[i];
		// operation 1
		let r: usize = 2;
		let w =  {
			let mut word: Vec<char> = vec![' '; 8];
			for i in 0..8 {
				let w_idx = (i + r) % 8;
				word[w_idx] = v[i];
			}
			word
		};

		// operation 2
		let w = {
			let mut word: Vec<char> = vec![' '; 8];
			for i in 0..8 {
				word[i] = if x[i] != w[i] { '1' } else { '0' };
			}
			word
		};

		// operation 3
		let num_bits = {
			let mut tmp = 0;
			for i in 0..8 {
				if w[i] == '1' { tmp += 1; }
			}
			tmp
		};

		let candidate: String = w.iter().collect();
		match tmp_map.get_mut(&num_bits) {
			Some(vec) => vec.push(candidate),
			None => {
				tmp_map.insert(num_bits, vec![candidate]);
			},
		}
	}
	map.insert(v.iter().collect(), tmp_map);
}

fn main() {
	let POSSIBLE_X: [Vec<char>; 255] = ["00000001".chars().collect(), "00000010".chars().collect(), "00000011".chars().collect(), "00000100".chars().collect(), "00000101".chars().collect(), "00000110".chars().collect(), "00000111".chars().collect(), "00001000".chars().collect(), "00001001".chars().collect(), "00001010".chars().collect(), "00001011".chars().collect(), "00001100".chars().collect(), "00001101".chars().collect(), "00001110".chars().collect(), "00001111".chars().collect(), "00010000".chars().collect(), "00010001".chars().collect(), "00010010".chars().collect(), "00010011".chars().collect(), "00010100".chars().collect(), "00010101".chars().collect(), "00010110".chars().collect(), "00010111".chars().collect(), "00011000".chars().collect(), "00011001".chars().collect(), "00011010".chars().collect(), "00011011".chars().collect(), "00011100".chars().collect(), "00011101".chars().collect(), "00011110".chars().collect(), "00011111".chars().collect(), "00100000".chars().collect(), "00100001".chars().collect(), "00100010".chars().collect(), "00100011".chars().collect(), "00100100".chars().collect(), "00100101".chars().collect(), "00100110".chars().collect(), "00100111".chars().collect(), "00101000".chars().collect(), "00101001".chars().collect(), "00101010".chars().collect(), "00101011".chars().collect(), "00101100".chars().collect(), "00101101".chars().collect(), "00101110".chars().collect(), "00101111".chars().collect(), "00110000".chars().collect(), "00110001".chars().collect(), "00110010".chars().collect(), "00110011".chars().collect(), "00110100".chars().collect(), "00110101".chars().collect(), "00110110".chars().collect(), "00110111".chars().collect(), "00111000".chars().collect(), "00111001".chars().collect(), "00111010".chars().collect(), "00111011".chars().collect(), "00111100".chars().collect(), "00111101".chars().collect(), "00111110".chars().collect(), "00111111".chars().collect(), "01000000".chars().collect(), "01000001".chars().collect(), "01000010".chars().collect(), "01000011".chars().collect(), "01000100".chars().collect(), "01000101".chars().collect(), "01000110".chars().collect(), "01000111".chars().collect(), "01001000".chars().collect(), "01001001".chars().collect(), "01001010".chars().collect(), "01001011".chars().collect(), "01001100".chars().collect(), "01001101".chars().collect(), "01001110".chars().collect(), "01001111".chars().collect(), "01010000".chars().collect(), "01010001".chars().collect(), "01010010".chars().collect(), "01010011".chars().collect(), "01010100".chars().collect(), "01010101".chars().collect(), "01010110".chars().collect(), "01010111".chars().collect(), "01011000".chars().collect(), "01011001".chars().collect(), "01011010".chars().collect(), "01011011".chars().collect(), "01011100".chars().collect(), "01011101".chars().collect(), "01011110".chars().collect(), "01011111".chars().collect(), "01100000".chars().collect(), "01100001".chars().collect(), "01100010".chars().collect(), "01100011".chars().collect(), "01100100".chars().collect(), "01100101".chars().collect(), "01100110".chars().collect(), "01100111".chars().collect(), "01101000".chars().collect(), "01101001".chars().collect(), "01101010".chars().collect(), "01101011".chars().collect(), "01101100".chars().collect(), "01101101".chars().collect(), "01101110".chars().collect(), "01101111".chars().collect(), "01110000".chars().collect(), "01110001".chars().collect(), "01110010".chars().collect(), "01110011".chars().collect(), "01110100".chars().collect(), "01110101".chars().collect(), "01110110".chars().collect(), "01110111".chars().collect(), "01111000".chars().collect(), "01111001".chars().collect(), "01111010".chars().collect(), "01111011".chars().collect(), "01111100".chars().collect(), "01111101".chars().collect(), "01111110".chars().collect(), "01111111".chars().collect(), "10000000".chars().collect(), "10000001".chars().collect(), "10000010".chars().collect(), "10000011".chars().collect(), "10000100".chars().collect(), "10000101".chars().collect(), "10000110".chars().collect(), "10000111".chars().collect(), "10001000".chars().collect(), "10001001".chars().collect(), "10001010".chars().collect(), "10001011".chars().collect(), "10001100".chars().collect(), "10001101".chars().collect(), "10001110".chars().collect(), "10001111".chars().collect(), "10010000".chars().collect(), "10010001".chars().collect(), "10010010".chars().collect(), "10010011".chars().collect(), "10010100".chars().collect(), "10010101".chars().collect(), "10010110".chars().collect(), "10010111".chars().collect(), "10011000".chars().collect(), "10011001".chars().collect(), "10011010".chars().collect(), "10011011".chars().collect(), "10011100".chars().collect(), "10011101".chars().collect(), "10011110".chars().collect(), "10011111".chars().collect(), "10100000".chars().collect(), "10100001".chars().collect(), "10100010".chars().collect(), "10100011".chars().collect(), "10100100".chars().collect(), "10100101".chars().collect(), "10100110".chars().collect(), "10100111".chars().collect(), "10101000".chars().collect(), "10101001".chars().collect(), "10101010".chars().collect(), "10101011".chars().collect(), "10101100".chars().collect(), "10101101".chars().collect(), "10101110".chars().collect(), "10101111".chars().collect(), "10110000".chars().collect(), "10110001".chars().collect(), "10110010".chars().collect(), "10110011".chars().collect(), "10110100".chars().collect(), "10110101".chars().collect(), "10110110".chars().collect(), "10110111".chars().collect(), "10111000".chars().collect(), "10111001".chars().collect(), "10111010".chars().collect(), "10111011".chars().collect(), "10111100".chars().collect(), "10111101".chars().collect(), "10111110".chars().collect(), "10111111".chars().collect(), "11000000".chars().collect(), "11000001".chars().collect(), "11000010".chars().collect(), "11000011".chars().collect(), "11000100".chars().collect(), "11000101".chars().collect(), "11000110".chars().collect(), "11000111".chars().collect(), "11001000".chars().collect(), "11001001".chars().collect(), "11001010".chars().collect(), "11001011".chars().collect(), "11001100".chars().collect(), "11001101".chars().collect(), "11001110".chars().collect(), "11001111".chars().collect(), "11010000".chars().collect(), "11010001".chars().collect(), "11010010".chars().collect(), "11010011".chars().collect(), "11010100".chars().collect(), "11010101".chars().collect(), "11010110".chars().collect(), "11010111".chars().collect(), "11011000".chars().collect(), "11011001".chars().collect(), "11011010".chars().collect(), "11011011".chars().collect(), "11011100".chars().collect(), "11011101".chars().collect(), "11011110".chars().collect(), "11011111".chars().collect(), "11100000".chars().collect(), "11100001".chars().collect(), "11100010".chars().collect(), "11100011".chars().collect(), "11100100".chars().collect(), "11100101".chars().collect(), "11100110".chars().collect(), "11100111".chars().collect(), "11101000".chars().collect(), "11101001".chars().collect(), "11101010".chars().collect(), "11101011".chars().collect(), "11101100".chars().collect(), "11101101".chars().collect(), "11101110".chars().collect(), "11101111".chars().collect(), "11110000".chars().collect(), "11110001".chars().collect(), "11110010".chars().collect(), "11110011".chars().collect(), "11110100".chars().collect(), "11110101".chars().collect(), "11110110".chars().collect(), "11110111".chars().collect(), "11111000".chars().collect(), "11111001".chars().collect(), "11111010".chars().collect(), "11111011".chars().collect(), "11111100".chars().collect(), "11111101".chars().collect(), "11111110".chars().collect(), "11111111".chars().collect()];

	let mut map: HashMap<String, HashMap<u8, Vec<String>>> = HashMap::new();
	let v: Vec<char> = "00110011".chars().collect();
	fill_map(&POSSIBLE_X, &mut map, &v);
	// eprintln!("{:?}", map);


	let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());
	let T = get_int::<usize>();
	for t in 1..=T {
		solve(t, &mut out, &POSSIBLE_X, &mut map);
	}
}

fn test_operations() {
	let v: Vec<char> = "00110011".chars().collect();
	let x: Vec<char> = "01110010".chars().collect();

	// operation 1
	let r: usize = 2;
	let w =  {
		let mut word: Vec<char> = vec![' '; 8];
		for i in 0..8 {
			let w_idx = (i + r) % 8;
			word[w_idx] = v[i];
		}
		word
	};
	eprintln!("After operation 1: word = {:?}", w);

	// operation 2
	let w = {
		let mut word: Vec<char> = vec![' '; 8];
		for i in 0..8 {
			word[i] = if x[i] != w[i] { '1' } else { '0' };
		}
		word
	};
	eprintln!("After operation 2: word = {:?}", w);

	// operation 3
	let num_bits = {
		let mut tmp = 0;
		for i in 0..8 {
			if w[i] == '1' { tmp += 1; }
		}
		tmp
	};
	eprintln!("After operation 3: number of bits 1 = {}", num_bits);
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

