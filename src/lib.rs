//! English-like word generator ported from Ryo Ota's fakelish packages.

extern crate serde;
extern crate rand;

use serde::{Deserialize, Serialize};
use std::{
	collections::HashMap,
	sync::OnceLock
};
use rand::Rng;

pub struct WordGenerator {
	min: u8, max: u8, seq: u8
}

impl WordGenerator {
	/// Configure a WordGenerator with min/max word length
	/// and maximum sequence [1—4] of related characters
	/// (higher `seq` value => less entropic);
	/// 
	/// # Example:
	/// ```
	/// use fakelish::WordGenerator;
	/// fn main() {
	///		let mut generate = WordGenerator::new(6, 9, 4);
	/// 	let word: String = generate.word();
	/// 	let words: Vec<String> = generate.phrase(6);
	/// }
	/// ```
	pub fn new(min: u8, max: u8, seq: u8) -> Self {
		let min = min.max(1);
		let seq = seq.clamp(1, 4);
		WordGenerator { min, max, seq }
	}

	/// Generate a word.
	pub fn word(&mut self) -> String {
		let mut word = String::from("");
		let mut chars: Vec<String> = Vec::new();
		let mut ch = String::from("^");

		let min: usize = rand::thread_rng()
			.gen_range(self.min..=self.max).into();
		let max: usize = self.max.into();
		let seq: usize = self.seq.into();

		while word.len() < min && ch != "END" {
			chars.push(ch.to_string());
			if chars.len() > seq {
				chars.remove(0);
			}
			let mut c = 0;

			let map = get_probabilities();
			let mut probabilities = &Vec::<Probability>::new();
			while probabilities.len() == 0 && c < chars.len() {
				let key = chars[c..].join("");
				let record = map.get(&key);
				if !record.is_none() {
					probabilities = record.unwrap();
				}
				c += 1;
			}
			let mut next_ch: String = String::from("");
			let mut rng = rand::thread_rng();
			for p in probabilities {
				let candidate_ch = p.0.clone();
				let probability  = p.1.clone();
				if rng.gen_range(0.0..1.0) <= probability {
					next_ch = candidate_ch;
					break;
				}
			}

			if next_ch != "END" {
				word.push_str(&next_ch);
			}
			ch = next_ch;
		}

		if word.len() < min || word.len() > max {
			self.word()
		} else {
			word
		}
	}

	/// Generate multiple `num` of words.
	pub fn phrase(&mut self, num: u16) -> Vec<String> {
		let num: usize = num.into();
		let mut words: Vec<String> = Vec::with_capacity(num);
		while words.len() < num {
			words.push(self.word().to_string());
		}
		words
	}
}


#[derive(Deserialize, Serialize)]
struct Probability(String, f32);
type Probabilities = HashMap<String, Vec<Probability>>;
const JSON: &str = include_str!("word-probability.json");
static PROBABILITIES: OnceLock<Probabilities> = OnceLock::new();
fn get_probabilities() -> &'static Probabilities {
	PROBABILITIES.get_or_init(|| {
		serde_json::from_str::<Probabilities>(&JSON)
			.expect("Failed to parse probabilities.")
	})
}