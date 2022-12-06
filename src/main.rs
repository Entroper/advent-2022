use std::error::Error;

mod day1;

fn main() -> Result<(), Box<dyn Error>> {
	day1::part1()?;
	day1::part2()?;

    Ok(())
}
