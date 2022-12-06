use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};

use crate::util;

pub fn part1() -> Result<(), Box<dyn Error>> {
	let file = File::open("input03.txt")?;
	let reader = BufReader::new(file);

	let mut total = 0;
	for line in reader.lines() {
		let line = line?;
		let half_length = line.len()/2;
		let sack1 = &line[..half_length];
		let sack2 = &line[half_length..];
		total += get_matching_priority(sack1, sack2);
	}

	println!("{total}");

	Ok(())
}

pub fn part2() -> Result<(), Box<dyn Error>> {
	let file = File::open("input03.txt")?;
	let reader = BufReader::new(file);

	let mut total = 0;
	let mut lines = reader.lines();
	while let Some(line1) = lines.next() {
		let line1 = line1?;
		let line2 = lines.next().ok_or(util::InvalidInputError("Didn't read 3 lines"))??;
		let line3 = lines.next().ok_or(util::InvalidInputError("Didn't read 3 lines"))??;
		total += get_triple_priority(&line1, &line2, &line3);
	}

	println!("{total}");

	Ok(())
}

fn get_matching_priority(sack1: &str, sack2: &str) -> i32 {
	for char1 in sack1.chars() {
		for char2 in sack2.chars() {
			if char1 == char2 {
				return match char1 {
					'a'..='z' => char1 as i32 - 'a' as i32 + 1,
					'A'..='Z' => char1 as i32 - 'A' as i32 + 27,
					_ => 0
				}
			}
		}
	}

	0
}

fn get_triple_priority(sack1: &str, sack2: &str, sack3: &str) -> i32 {
	for char1 in sack1.chars() {
		for char2 in sack2.chars() {
			if char1 == char2 {
				for char3 in sack3.chars() {
					if char1 == char3 {
						return match char1 {
							'a'..='z' => char1 as i32 - 'a' as i32 + 1,
							'A'..='Z' => char1 as i32 - 'A' as i32 + 27,
							_ => 0
						}
					}
				}
			}
		}
	}

	0
}
