struct Solution;

impl Solution {
	pub fn min_distance(word1: String, word2: String) -> i32 {
		if word1.is_empty() {
			return word2.len() as i32;
		} else if word2.is_empty() {
			return word1.len() as i32;
		}

		let len1 = word1.len();
		let len2 = word2.len();

		let mut dp = vec![vec![0i32; len2+1]; len1+1];

		for i in 1..=len1 {
			dp[i][0] = i as i32;
		}
		for j in 1..=len2 {
			dp[0][j] = j as i32;
		}

		for (i, c1) in word1.chars().enumerate() {
			for (j, c2) in word2.chars().enumerate() {
				let ins = dp[i+1][j] + 1;
				let del = dp[i][j+1] + 1;
				let sub = dp[i][j] + (c1 != c2) as i32;
				dp[i+1][j+1] = ins.min(del).min(sub);
			}
		}

		*dp.last().and_then(|s| s.last()).unwrap()
	}
}

fn main() {
}

#[test]
fn test_edit_distance() {
	assert_eq!(7, Solution::min_distance("auiabbbaiii".to_string(), "abba".to_string()));
}

