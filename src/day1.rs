use anyhow;
use aoc::soln;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, BufRead};

#[soln]
pub fn main() -> anyhow::Result<()> {
    let file = fs::File::open("input_1.txt")?;
    let lines = io::BufReader::new(file).lines();
    let nums: Vec<i32> = lines.map(|l| l.unwrap().parse::<i32>().unwrap()).collect();
    let mut nums_set = HashSet::new();
    for num in nums.clone() {
        nums_set.insert(num);
    }
    let sum = 2020;
    for num in nums.clone() {
        if nums_set.contains(&(sum - num)) {
            println!(
                "Found ({}, {}), which multiply to ({}).",
                num,
                2020 - num,
                num * (2020 - num)
            );
            break;
        }
    }
    let mut num_sums_map: HashMap<i32, (i32, i32)> = HashMap::new();
    for num1 in nums.clone() {
        for num2 in nums.clone() {
            num_sums_map.insert(num1 + num2, (num1, num2));
        }
    }
    for num in nums.clone() {
        if let Some((num1, num2)) = num_sums_map.get(&(2020 - num)) {
            println!(
                "Found ({}, {}, {}), which multiply to ({}).",
                num,
                num1,
                num2,
                num * num1 * num2
            );
            break;
        }
    }
    Ok(())
}
