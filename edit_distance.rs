
const UNIT:  i32 = 1;
const BEST:  i32 = 0;
const WORST: i32 = i32::MAX / 2;

struct EditDistance {
	allow_transpose: bool,
	score: Vec<Vec<i32>>,
}

impl EditDistance {

	fn new() -> Self {
		let ed = EditDistance {
			allow_transpose: true,
			score: vec![],
		};

		ed
	}

	fn clear(&mut self, source_length: usize, target_length: usize) {
		if self.score.is_empty() || self.score.len() < source_length + 1 ||
				self.score[0].len() < target_length + 1 {
			self.score = vec![vec![WORST; target_length + 1]; source_length + 1];
		}
	}

	fn better(x: i32, y: i32) -> i32 {
		if x < y { x } else { y }
	}

	fn combine(x: i32, y: i32) -> i32 {
		x + y
	}

	fn insert_cost() -> i32 {
		UNIT
	}

	fn delete_cost() -> i32 {
		UNIT
	}

	fn substitute_cost(source: char, target: char) -> i32 {
		if source == target { BEST } else { UNIT }
	}

	fn tranpose_cost(&self, s1: char, s2: char, t1: char, t2: char) -> i32 {
		if s1 == t2 && s2 == t1 {
			if self.allow_transpose {
				return UNIT;
			}
			return 2 * UNIT;
		}
		WORST
	}

	fn score1(&mut self, source: &Vec<char>, s_pos: usize, target: &Vec<char>, t_pos: usize) -> i32 {
		for i in 0..=s_pos {
			for j in 0..=t_pos {
				let mut min_score = self.score[i][j];
				if min_score != WORST {
					continue;
				}
				if i == 0 && j == 0 {
					min_score = BEST;
				} else {
					if i > 0 {
						min_score = Self::better(min_score,
							Self::combine(self.score[i - 1][j],	Self::delete_cost()));
					}
					if j > 0 {
						min_score = Self::better(min_score,
							Self::combine(self.score[i][j - 1],	Self::insert_cost()));
					}
					if i > 0 && j > 0 {
						min_score = Self::better(min_score,
							Self::combine(self.score[i - 1][j - 1],
								Self::substitute_cost(source[i - 1], target[j - 1])));
					}
					if i > 1 && j > 1 {
						min_score = Self::better(min_score,
							Self::combine(self.score[i - 2][j - 2],
								self.tranpose_cost(source[i - 2], source[i - 1],
									target[j - 2], target[j - 1])));
					}
				}
				self.score[i][j] = min_score;
			}
		}
		self.score[s_pos][t_pos]
	}

	fn score_chars(&mut self, source: &Vec<char>, target: &Vec<char>) -> i32 {
		self.clear(source.len(), target.len());
		self.score1(source, source.len(), target, target.len())
	}

	fn score(&mut self, source: &str, target: &str) -> i32 {
		if source == target {
			return 0;
		}
		let source: Vec<char> = source.chars().collect();
		let target: Vec<char> = target.chars().collect();
		self.score_chars(&source, &target)
	}
}

fn main() {
}

#[test]
fn test_edit_distance() {
	let mut ed = EditDistance::new();
	assert_eq!(7, ed.score("auiabbbaiii", "abba"));
}

