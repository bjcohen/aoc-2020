use anyhow::{anyhow, Result};
use aoc::soln;
use std::collections::{HashSet, VecDeque};
use std::fs;
use std::num::ParseIntError;

#[soln]
pub fn main() -> Result<()> {
    let contents = fs::read_to_string("input_22.txt")?;
    let (p1, p2) = parse(&contents)?;
    part1(&p1, &p2)?;
    part2(&p1, &p2)?;
    Ok(())
}

fn parse(contents: &str) -> Result<(VecDeque<i64>, VecDeque<i64>)> {
    let split: Vec<&str> = contents.split("\n\n").collect();
    let p1 = split[0]
        .lines()
        .skip(1)
        .map(|c| c.parse())
        .collect::<Result<VecDeque<i64>, ParseIntError>>()?;
    let p2 = split[1]
        .lines()
        .skip(1)
        .map(|c| c.parse())
        .collect::<Result<VecDeque<i64>, ParseIntError>>()?;
    Ok((p1, p2))
}

fn part1(p1: &VecDeque<i64>, p2: &VecDeque<i64>) -> Result<i64> {
    let mut p1 = p1.clone();
    let mut p2 = p2.clone();
    while p1.len() > 0 && p2.len() > 0 {
        let n1 = p1.pop_front().ok_or(anyhow!("no element"))?;
        let n2 = p2.pop_front().ok_or(anyhow!("no element"))?;
        if n1 > n2 {
            p1.push_back(n1);
            p1.push_back(n2);
        } else {
            p2.push_back(n2);
            p2.push_back(n1);
        }
    }
    let p_winning = if p1.is_empty() { p2 } else { p1 };
    let score = p_winning
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (i as i64 + 1) * c)
        .sum();
    println!("score={}", score);
    Ok(score)
}

fn part2(p1: &VecDeque<i64>, p2: &VecDeque<i64>) -> Result<i64> {
    let score = recursive_combat(p1, p2)?
        .1
        .ok_or(anyhow!("no score in top level game"))?;
    println!("score={}", score);
    Ok(score)
}

fn recursive_combat(p1: &VecDeque<i64>, p2: &VecDeque<i64>) -> Result<(i64, Option<i64>)> {
    let mut p1 = p1.clone();
    let mut p2 = p2.clone();
    let mut seen_decks = HashSet::new();
    while p1.len() > 0 && p2.len() > 0 {
        if !seen_decks.insert((p1.clone(), p2.clone())) {
            return Ok((1, None));
        }
        let n1 = p1.pop_front().ok_or(anyhow!("no element"))?;
        let n2 = p2.pop_front().ok_or(anyhow!("no element"))?;
        let winner = if p1.len() as i64 >= n1 && p2.len() as i64 >= n2 {
            recursive_combat(
                &p1.iter().take(n1 as usize).copied().collect(),
                &p2.iter().take(n2 as usize).copied().collect(),
            )?
            .0
        } else if n1 > n2 {
            1
        } else {
            2
        };
        match winner {
            1 => {
                p1.push_back(n1);
                p1.push_back(n2);
            }
            2 => {
                p2.push_back(n2);
                p2.push_back(n1);
            }
            _ => unreachable!(),
        }
    }
    let winning_player = if p1.is_empty() { 2 } else { 1 };
    let p_winning = if p1.is_empty() { p2 } else { p1 };
    let score = p_winning
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (i as i64 + 1) * c)
        .sum();
    Ok((winning_player, Some(score)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let (p1, p2) = parse(
            "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10",
        )?;
        assert_eq!(306, part1(&p1, &p2)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let (p1, p2) = parse(
            "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10",
        )?;

        assert_eq!(291, part2(&p1, &p2)?);
        Ok(())
    }
}
