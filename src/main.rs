mod day1;
mod day2;

use std::io;

fn main() -> io::Result<()> {
    day1::day1()?;
    day2::day2()?;
    Ok(())
}
