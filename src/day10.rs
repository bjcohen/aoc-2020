use anyhow::{anyhow, bail, Result};
use aoc::soln;
use std::fs;
use std::num::ParseIntError;

#[soln]
pub fn main() -> Result<()> {
    let contents = fs::read_to_string("input_10.txt")?;
    let mut ratings: Vec<i64> = contents
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<i64>, ParseIntError>>()?;
    ratings.sort();
    part1(&ratings)?;
    part2(&ratings)?;
    Ok(())
}

fn part1(ratings: &Vec<i64>) -> Result<i64> {
    let init = match ratings[0] {
        3 => (0, 2),
        1 => (1, 1),
        d => bail!("Unhandled diff amount {}", d),
    };
    let diff_counts = ratings.windows(2).fold(Ok(init), |acc, w| {
        if let Ok(acc) = acc {
            match w[1] - w[0] {
                3 => Ok((acc.0, acc.1 + 1)),
                1 => Ok((acc.0 + 1, acc.1)),
                d => bail!("Unhandled diff amount {}", d),
            }
        } else {
            acc
        }
    })?;
    println!("3-count * 1-count = {}", diff_counts.0 * diff_counts.1);
    Ok(diff_counts.0 * diff_counts.1)
}

fn part2(ratings: &Vec<i64>) -> Result<i64> {
    let num_ending_with = ratings
        .iter()
        .enumerate()
        .fold(vec![1], |mut acc: Vec<i64>, (i, r)| {
            let x = if r <= &3 { 1 } else { 0 };
            let y: i64 = (0..3)
                .filter(|j| *j < i)
                .filter(|j| r - ratings[i - 1 - j] <= 3)
                .map(|j| acc[i - j])
                .sum();
            acc.push(x + y);
            acc
        });
    let num_combinations = num_ending_with.last().ok_or(anyhow!("No last element"))?;
    println!("Number of combinations = {}", num_combinations);
    Ok(*num_combinations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_any() {
        let mut input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        input.sort();
        assert_eq!(35, part1(&input).unwrap());
    }

    #[test]
    fn test_part2_1() {
        let mut input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        input.sort();
        assert_eq!(8, part2(&input).unwrap());
    }

    #[test]
    fn test_part2_2() {
        let mut input = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        input.sort();
        assert_eq!(19208, part2(&input).unwrap());
    }
}
