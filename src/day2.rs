use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;
use std::str::FromStr;

use crate::util;

#[derive(Debug)]
enum RPS {
	Rock,
	Paper,
	Scissors
}

impl RPS {
	pub fn value(&self) -> i32 {
		match self {
			RPS::Rock => 1,
			RPS::Paper => 2,
			RPS::Scissors => 3
		}
	}

	pub fn score_vs(&self, other: &Self) -> i32 {
		let winnings = match self {
			RPS::Rock => match other {
				RPS::Rock => 3,
				RPS::Paper => 0,
				RPS::Scissors => 6
			},
			RPS::Paper => match other {
				RPS::Rock => 6,
				RPS::Paper => 3,
				RPS::Scissors => 0
			},
			RPS::Scissors => match other {
				RPS::Rock => 0,
				RPS::Paper => 6,
				RPS::Scissors => 3
			}
		};

		winnings + self.value()
	}

	pub fn from_opponent(value: &str, other: &Self) -> Result<RPS, std::io::Error> {
		match value {
			"X" => match other {
				RPS::Rock => Ok(RPS::Scissors),
				RPS::Paper => Ok(RPS::Rock),
				RPS::Scissors => Ok(RPS::Paper)
			},
			"Y" => match other {
				RPS::Rock => Ok(RPS::Rock),
				RPS::Paper => Ok(RPS::Paper),
				RPS::Scissors => Ok(RPS::Scissors)
			},
			"Z" => match other {
				RPS::Rock => Ok(RPS::Paper),
				RPS::Paper => Ok(RPS::Scissors),
				RPS::Scissors => Ok(RPS::Rock)
			},
			_ => Err(util::InvalidInputError("Bad opponent"))
		}
	}
}

impl FromStr for RPS {
	type Err = std::io::Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"A" | "X" => Ok(RPS::Rock),
			"B" | "Y" => Ok(RPS::Paper),
			"C" | "Z" => Ok(RPS::Scissors),
			_ => Err(util::InvalidInputError("Bad RPS"))
		}
	}
}

pub fn part1() -> Result<(), Box<dyn Error>> {
	let file = File::open("input02.txt")?;
	let reader = BufReader::new(file);
	let mut score: i32 = 0;
	for line in reader.lines() {
		let line = line?;
		let mut tokens = line.split_ascii_whitespace();
		let first: RPS = tokens.next().ok_or("Bad line")?.parse()?;
		let second: RPS = tokens.next().ok_or("Bad line")?.parse()?;
		let round = second.score_vs(&first);
		score += round;
	}

	println!("{score}");

	Ok(())
}

pub fn part2() -> Result<(), Box<dyn Error>> {
	let file = File::open("input02.txt")?;
	let reader = BufReader::new(file);
	let mut score: i32 = 0;
	for line in reader.lines() {
		let line = line?;
		let mut tokens = line.split_ascii_whitespace();
		let first: RPS = tokens.next().ok_or("Bad line")?.parse()?;
		let second: RPS = RPS::from_opponent(tokens.next().ok_or("Bad line")?, &first)?;
		let round = second.score_vs(&first);
		score += round;
	}

	println!("{score}");

	Ok(())
}
