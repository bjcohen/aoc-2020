use anyhow::{bail, Result};
use aoc::soln;
use std::collections::HashSet;
use std::fs;
use std::num::ParseIntError;

#[soln]
pub fn main() -> Result<()> {
    let contents = fs::read_to_string("input_9.txt")?;
    let numbers: Vec<u64> = contents
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<u64>, ParseIntError>>()?;
    let num = part1(&numbers)?;
    part2(num, &numbers)?;
    Ok(())
}

fn part1(numbers: &Vec<u64>) -> Result<u64> {
    let mut set = HashSet::new();
    for (i, n) in numbers.iter().enumerate() {
        set.insert(n);
        if set.len() > 25 {
            let mut found = false;
            for m in set.iter() {
                if *n > **m && set.contains(&(*n - *m)) {
                    set.remove(&numbers[i - 25]);
                    found = true;
                    break;
                }
            }
            if !found {
                println!("First number is ({})", *n);
                return Ok(*n);
            }
        }
    }
    bail!("Couldn't find non-summing number")
}

fn part2(num: u64, numbers: &Vec<u64>) -> Result<u64> {
    let prefix_sum: Vec<u64> = numbers.iter().fold(Vec::new(), |mut acc, n| {
        acc.push(n + acc.last().unwrap_or(&0));
        acc
    });
    for i in 0..numbers.len() {
        for j in 0..i {
            if prefix_sum[i] - prefix_sum[j] == num {
                let min = numbers[j+1..i+1].iter().min().unwrap();
                let max = numbers[j+1..i+1].iter().max().unwrap();
                println!(
                    "Found i,j={},{} {}..{}={}, {}+{}={}",
                    i,
                    j,
                    numbers[i],
                    numbers[j],
                    prefix_sum[i] - prefix_sum[j],
                    min,
                    max,
                    min + max
                );
                return Ok(min + max);
            }
        }
    }
    bail!("Didn't find a range")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_any() {
        assert_eq!(
            62,
            part2(
                127,
                &vec![
                    35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                    309, 576,
                ]
            )
            .unwrap(),
        );
    }
}
