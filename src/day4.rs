use std::error::Error;
use std::cmp;
use std::ops::Range;
use std::fs::File;
use std::io::{BufReader, BufRead};

use crate::util;

pub fn part1() -> Result<(), Box<dyn Error>> {
	let file = File::open("input04.txt")?;
	let reader = BufReader::new(file);

	let mut total = 0;
	for line in reader.lines() {
		let line = line?;
		let mut pairs = line.split(',');
		let first = pairs.next().ok_or(util::InvalidInputError("No range"))?;
		let second = pairs.next().ok_or(util::InvalidInputError("No range"))?;
		let first = parse_range(first)?;
		let second = parse_range(second)?;
		let intersection = first.intersect(&second);
		if intersection == first || intersection == second {
			total += 1;
		}
	}

	println!("{total}");

	Ok(())
}

pub fn part2() -> Result<(), Box<dyn Error>> {
	let file = File::open("input04.txt")?;
	let reader = BufReader::new(file);

	let mut total = 0;
	for line in reader.lines() {
		let line = line?;
		let mut pairs = line.split(',');
		let first = pairs.next().ok_or(util::InvalidInputError("No range"))?;
		let second = pairs.next().ok_or(util::InvalidInputError("No range"))?;
		let first = parse_range(first)?;
		let second = parse_range(second)?;
		let intersection = first.intersect(&second);
		if !intersection.is_empty() {
			total += 1;
		}
	}

	println!("{total}");

	Ok(())
}

fn parse_range(input: &str) -> Result<Range<i32>, Box<dyn Error>> {
	let mut bounds = input.split('-');
	let first: i32 = bounds.next().ok_or(util::InvalidInputError("No range"))?.parse()?;
	let second: i32 = bounds.next().ok_or(util::InvalidInputError("No range"))?.parse()?;
	Ok(Range { start: first, end: second + 1})
}

trait Intersect {
	fn intersect(&self, other: &Self) -> Self;
}

impl Intersect for Range<i32> {
	fn intersect(&self, other: &Self) -> Range<i32> {
		Range { start: cmp::max(self.start, other.start), end: cmp::min(self.end, other.end) }
	}
}
