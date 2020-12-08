#![feature(iterator_fold_self)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

use anyhow;

fn main() -> anyhow::Result<()> {
    day1::day1()?;
    day2::day2()?;
    day3::day3()?;
    day4::day4()?;
    day5::day5()?;
    day6::day6()?;
    day7::day7()?;
    day8::main()?;
    Ok(())
}
