use std::collections::HashSet;
use std::collections::VecDeque;

fn min_distance(mut s1: &str, mut s2: &str) -> i32 {
	if s1 == s2 { return 0; }

	while s1.len() > 0 && s2.len() > 0 && is_last_char_equals(s1, s2) {
		s1 = slice_minus_last_char(s1);
		s2 = slice_minus_last_char(s2);
	}

	let mut q_visited = VecDeque::new();
	q_visited.push_back((s1, s2, 0));

	let mut s_visited = HashSet::new();
	s_visited.insert((s1, s2));

	loop {
		let (mut s1, mut s2, depth) = q_visited.pop_front().unwrap();

		if s1.len() == 0 {
			s2 = slice_minus_last_char(s2);
			if s1 == s2 { return depth + 1; }
			if !s_visited.contains(&(s1, s2)) {
				s_visited.insert((s1, s2));
				q_visited.push_back((s1, s2, depth + 1));
			}
			continue;
		}
		if s2.len() == 0 {
			s1 = slice_minus_last_char(s1);
			if s1 == s2 { return depth + 1; }
			if !s_visited.contains(&(s1, s2)) {
				s_visited.insert((s1, s2));
				q_visited.push_back((s1, s2, depth + 1));
			}
			continue;
		}

		let cases = vec![
			(slice_minus_last_char(s1), s2),
			(s1, slice_minus_last_char(s2)),
			(slice_minus_last_char(s1), slice_minus_last_char(s2)),
		];

		for (mut w1, mut w2) in cases {
			if w1 == w2 { return depth + 1; }
			while w1.len() > 0 && w2.len() > 0 && is_last_char_equals(w1, w2) {
				w1 = slice_minus_last_char(w1);
				w2 = slice_minus_last_char(w2);
			}

			if !s_visited.contains(&(w1, w2)) {
				s_visited.insert((w1, w2));
				q_visited.push_back((w1, w2, depth + 1));
			}
		}
	}
}

fn is_last_char_equals(s1: &str, s2: &str) -> bool {
	&s1[s1.len() - 1..] == &s2[s2.len() - 1..]
}

fn slice_minus_last_char(s: &str) -> &str {
	&s[..s.len() - 1]
}

fn main() {
}


#[test]
fn test_slice_minus_last_char() {
	let s = "auiabbbaiii";
	let mut tmp = slice_minus_last_char(s);
	assert_eq!("auiabbbaii", tmp);
	tmp = slice_minus_last_char(tmp);
	assert_eq!("auiabbbai", tmp);
	tmp = slice_minus_last_char(tmp);
	assert_eq!("auiabbba", tmp);
	assert_eq!("", slice_minus_last_char("a"));
}

#[test]
fn test_edit_distance() {
	assert_eq!(7, min_distance("auiabbbaiii", "abba"));
}

