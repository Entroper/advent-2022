use std::cmp::Ordering;
use std::error::Error;
use std::fs::File;
use std::vec::Vec;
use std::io::{BufReader, BufRead};

#[derive(Eq)]
struct Elf {
	food: Vec<i32>
}

impl Elf {
	pub fn new() -> Elf {
		Elf {
			food: Vec::new()
		}
	}

	pub fn total(&self) -> i32 {
		self.food.iter().sum()
	}
}

impl PartialEq for Elf {
	fn eq(&self, other: &Self) -> bool {
		self.total() == other.total()
	}
}

impl Ord for Elf {
	fn cmp(&self, other: &Self) -> Ordering {
		self.total().cmp(&other.total())
	}
}

impl PartialOrd for Elf {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part1() -> Result<(), Box<dyn Error>> {
	let elves = read_file("input01.txt")?;
	let max = elves.iter().max().ok_or("No elves?")?.total();
	println!("{max}");

	Ok(())
}

pub fn part2() -> Result<(), Box<dyn Error>> {
	let mut elves = read_file("input01.txt")?;
	elves.sort();
	let top_three: i32 = elves[elves.len()-3..].iter().map(|elf| elf.total()).sum();
	println!("{top_three}");

	Ok(())
}

fn read_file(filename: &str) -> Result<Vec<Elf>, Box<dyn Error>>  {
	let file = File::open(filename)?;
	let reader = BufReader::new(file);
	let mut elves = Vec::new();
	let mut elf = Elf::new();
	for line in reader.lines() {
		let line = line?;
		if line == "" {
			elves.push(elf);
			elf = Elf::new();
		}
		else {
			elf.food.push(line.parse()?);
		}
	}
	elves.push(elf);

	Ok(elves)
}
