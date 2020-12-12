#![feature(destructuring_assignment)]

use anyhow::{anyhow, bail, Result};
use aoc::soln;
use std::fs;

#[soln]
pub fn main() -> Result<()> {
    let contents = fs::read_to_string("input_12.txt")?;
    let instructions: Vec<&str> = contents.lines().collect();
    part1(&instructions)?;
    part2(&instructions)?;
    Ok(())
}

fn part1(instructions: &Vec<&str>) -> Result<i64> {
    let mut x = 0;
    let mut y = 0;
    let mut dir = 0;
    for i in instructions {
        let a: char = i
            .chars()
            .nth(0)
            .ok_or(anyhow!("Couldn't get the first char"))?;
        let b: i64 = i[1..].parse()?;
        match a {
            'N' => y += b,
            'S' => y -= b,
            'E' => x += b,
            'W' => x -= b,
            'L' => dir = (dir + b) % 360,
            'R' => dir = (dir - b) % 360,
            'F' => {
                if dir == 0 {
                    x += b;
                } else if dir == 180 || dir == -180 {
                    x -= b;
                } else if dir == 90 || dir == -270 {
                    y += b;
                } else if dir == -90 || dir == 270 {
                    y -= b;
                } else {
                    bail!("Unhandled dir {}", dir);
                }
            }
            _ => unreachable!(),
        }
    }
    println!("x={} y={} dist={}", x, y, x.abs() + y.abs());
    Ok(x.abs() + y.abs())
}

fn part2(instructions: &Vec<&str>) -> Result<i64> {
    let mut x = 0;
    let mut y = 0;
    let mut wx = 10;
    let mut wy = 1;
    for i in instructions {
        let a: char = i
            .chars()
            .nth(0)
            .ok_or(anyhow!("Couldn't get the first char"))?;
        let b: i64 = i[1..].parse()?;
        match a {
            'N' => wy += b,
            'S' => wy -= b,
            'E' => wx += b,
            'W' => wx -= b,
            'L' => match b {
                90 => (wx, wy) = (-wy, wx),
                180 => (wx, wy) = (-wx, -wy),
                270 => (wx, wy) = (wy, -wx),
                _ => bail!("Unhandled angle {}", b),
            },
            'R' => match b {
                90 => (wx, wy) = (wy, -wx),
                180 => (wx, wy) = (-wx, -wy),
                270 => (wx, wy) = (-wy, wx),
                _ => bail!("Unhandled angle {}", b),
            },
            'F' => {
                x += wx * b;
                y += wy * b;
            }
            _ => unreachable!(),
        }
    }
    println!("x={} y={} dist={}", x, y, x.abs() + y.abs());
    Ok(x.abs() + y.abs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(25, part1(&vec!["F10", "N3", "F7", "R90", "F11"]).unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(286, part2(&vec!["F10", "N3", "F7", "R90", "F11"]).unwrap());
    }
}
