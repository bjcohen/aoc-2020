mod day1;
mod day2;
mod day3;

use std::io;

fn main() -> io::Result<()> {
    day1::day1()?;
    day2::day2()?;
    day3::day3()?;
    Ok(())
}
