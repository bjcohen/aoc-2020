use anyhow::Result;
use aoc::soln;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[soln]
pub fn main() -> Result<()> {
    let contents = fs::read_to_string("input_14.txt")?;
    let lines: Vec<&str> = contents.lines().collect();
    part1(&lines)?;
    part2(&lines)?;
    Ok(())
}

fn part1(lines: &Vec<&str>) -> Result<i64> {
    let mut m0: Option<i64> = None;
    let mut m1: Option<i64> = None;
    let mut mem: HashMap<i64, i64> = HashMap::new();
    for line in lines {
        if line[..3] == *"mas" {
            m0 = Some((1 << 36) - 1);
            m1 = Some(0);
            for (i, c) in line[7..].chars().rev().enumerate() {
                match c {
                    '0' => m0 = Some(m0.unwrap() ^ (1 << i)),
                    '1' => m1 = Some(m1.unwrap() ^ (1 << i)),
                    'X' => {}
                    _ => unreachable!(),
                }
            }
        } else {
            let re = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
            let cap = re.captures(line).unwrap();
            let addr: i64 = cap[1].parse().unwrap();
            let val: i64 = cap[2].parse().unwrap();
            mem.insert(addr, (val | m1.unwrap()) & m0.unwrap());
        }
    }
    let sum = mem.values().sum();
    println!("sum = {}", sum);
    Ok(sum)
}

fn part2(lines: &Vec<&str>) -> Result<i64> {
    let mut m1: i64 = 0;
    let mut floating_bits: Vec<usize> = vec![];
    let mut mem: HashMap<i64, i64> = HashMap::new();
    for line in lines {
        if line[..3] == *"mas" {
            m1 = 0;
            floating_bits = vec![];
            for (i, c) in line[7..].chars().rev().enumerate() {
                match c {
                    '0' => {}
                    '1' => m1 |= 1 << i,
                    'X' => floating_bits.push(i),
                    _ => unreachable!(),
                }
            }
        } else {
            let re = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
            let cap = re.captures(line).unwrap();
            let addr: i64 = cap[1].parse().unwrap();
            let val: i64 = cap[2].parse().unwrap();
            for i in 0..((1 as i64) << floating_bits.len()) {
                let m =
                    format!("{:#b}", i)[2..]
                        .chars()
                        .rev()
                        .enumerate()
                        .fold(0, |acc, (j, b)| {
                            if b == '1' {
                                acc | 1 << floating_bits[j]
                            } else {
                                acc
                            }
                        });
                mem.insert((addr | m1) ^ m, val);
            }
        }
    }
    let sum = mem.values().sum();
    println!("sum = {}", sum);
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines: Vec<&str> = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"
            .lines()
            .collect();
        assert_eq!(165, part1(&lines).unwrap());
    }

    #[test]
    fn test_part2() {
        let lines: Vec<&str> = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"
            .lines()
            .collect();
        assert_eq!(208, part2(&lines).unwrap());
    }
}
