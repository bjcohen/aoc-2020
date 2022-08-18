use anyhow::{Error, Result};
use aoc::soln;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

#[soln]
pub fn main() -> Result<()> {
    let file = fs::File::open("input_1.txt")?;
    let lines = BufReader::new(file).lines();
    let nums = lines
        .map(|l| {
            l.map_err(Error::new)
                .and_then(|l| l.parse::<i32>().map_err(Error::new))
        })
        .collect::<Result<Vec<i32>, _>>()?;
    let nums_set: HashSet<&i32> = HashSet::from_iter(&nums);
    const SUM: i32 = 2020;
    let num = nums
        .iter()
        .filter(|n| nums_set.contains(&(SUM - *n)))
        .next()
        .ok_or(Error::msg("Didn't find matching num"))?;
    println!(
        "Found ({}, {}), which multiply to ({}).",
        num,
        SUM - num,
        num * (SUM - num)
    );
    let num_sums_map: HashMap<i32, (i32, i32)> = HashMap::from_iter(
        nums.iter()
            .cartesian_product(&nums)
            .map(|(n1, n2)| (n1 + n2, (*n1, *n2))),
    );
    let (num, num1, num2) = nums
        .iter()
        .find_map(|n| num_sums_map.get(&(SUM - *n)).map(|(n1, n2)| (*n, *n1, *n2)))
        .ok_or(Error::msg("Didn't find matching num"))?;
    println!(
        "Found ({}, {}, {}), which multiply to ({}).",
        num,
        num1,
        num2,
        num * num1 * num2
    );
    Ok(())
}
