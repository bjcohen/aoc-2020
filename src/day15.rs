use anyhow::{anyhow, Result};
use aoc::soln;
use std::collections::HashMap;
use std::fs;

#[soln]
pub fn main() -> Result<()> {
    let contents = fs::read_to_string("input_15.txt")?;
    let lines: Vec<i64> = contents
        .trim()
        .split(',')
        .map(|l| l.parse())
        .collect::<Result<Vec<i64>, _>>()?;
    part1(&lines, 2020)?;
    part2(&lines, 30000000)?;
    Ok(())
}

fn part1(starting_nums: &Vec<i64>, n: i64) -> Result<i64> {
    let mut last_seen: HashMap<i64, i64> = HashMap::new();
    for (i, n) in starting_nums
        .iter()
        .take(starting_nums.len() - 1)
        .enumerate()
    {
        last_seen.insert(*n, i as i64);
    }
    let mut last_num = *starting_nums.last().ok_or(anyhow!("no last number"))?;
    for i in last_seen.len() as i64..n - 1 {
        let curr_num = if let Some(last_idx) = last_seen.get(&last_num) {
            i - last_idx
        } else {
            0
        };
        last_seen.insert(last_num, i);
        last_num = curr_num;
    }
    println!("{}", last_num);
    Ok(last_num)
}

fn part2(starting_nums: &Vec<i64>, n: i64) -> Result<i64> {
    part1(starting_nums, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let starting_nums = vec![0, 3, 6];
        assert_eq!(0, part1(&starting_nums, 10).unwrap());
        assert_eq!(436, part1(&starting_nums, 2020).unwrap());
    }
}
