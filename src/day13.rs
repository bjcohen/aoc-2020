#![feature(destructuring_assignment)]

use anyhow::{anyhow, Result};
use aoc::soln;
use std::fs;
use std::option::Option;

#[soln]
pub fn main() -> Result<()> {
    let contents = fs::read_to_string("input_13.txt")?;
    let lines: Vec<&str> = contents.lines().collect();
    let earliest_time = lines[0].parse()?;
    let buses: Vec<i64> = lines[1]
        .split(',')
        .filter(|c| *c != "x")
        .map(|c| c.parse().unwrap())
        .collect();
    part1(&earliest_time, &buses)?;
    part2(&lines[1])?;
    Ok(())
}

fn part1(earliest_time: &i64, buses: &Vec<i64>) -> Result<i64> {
    let min = buses
        .iter()
        .map(|b| (earliest_time % b - b).abs())
        .min()
        .unwrap();
    let key = buses
        .iter()
        .min_by_key(|b| (earliest_time % *b - *b).abs())
        .unwrap();
    println!("{} {} {}", min, key, min * key);
    Ok(min * key)
}

fn part2(buses_str: &str) -> Result<i64> {
    let buses: Vec<Option<i64>> = buses_str.split(',').map(|b| b.parse().ok()).collect();
    // find x s.t. x = i mod b[i] for all i
    // CRT with a_i = i, n_i = b[i]
    let cap_n: i64 = buses.iter().filter_map(|o| *o).product();
    let y_is: Vec<Option<i64>> = buses.iter().map(|n_| n_.map(|n| cap_n / n)).collect();
    let z_is: Vec<Option<i64>> = y_is
        .iter()
        .zip(buses.iter())
        .map(|(y_i, n_i)| {
            assert!(y_i.is_some() == n_i.is_some());
            if y_i.is_some() && n_i.is_some() {
                Some(inverse(y_i.unwrap(), n_i.unwrap()).unwrap())
            } else {
                None
            }
        })
        .collect();
    assert!(y_is.len() == z_is.len());
    let x: i64 = y_is
        .iter()
        .enumerate()
        .zip(z_is.iter())
        .filter_map(|((a_i, y_i), z_i)| {
            assert!(y_i.is_some() == z_i.is_some());
            if y_i.is_some() && z_i.is_some() {
                Some((a_i as i64) * y_i.unwrap() * z_i.unwrap())
            } else {
                None
            }
        })
        .sum();
    println!("{} {} {}", x, cap_n, cap_n - x % cap_n);
    Ok(cap_n - x % cap_n)
}

fn inverse(a: i64, n: i64) -> Result<i64> {
    let mut t = 0;
    let mut r = n;
    let mut t_ = 1;
    let mut r_ = a;
    while r_ != 0 {
        let q = r / r_;
        (t, t_) = (t_, t - q * t_);
        (r, r_) = (r_, r - q * r_);
    }
    if r > 1 {
        return Err(anyhow!("a is not invertible"));
    }
    if t < 0 {
        t = t + n;
    }
    Ok(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(295, part1(&939, &vec![7, 13, 59, 31, 19]).unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(1068781, part2("7,13,x,x,59,x,31,19").unwrap());
    }
}
