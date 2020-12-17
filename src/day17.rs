use anyhow::Result;
use aoc::soln;
use std::collections::{HashMap, HashSet};
use std::fs;

const A: char = '#';

#[soln]
pub fn main() -> Result<()> {
    let contents = fs::read_to_string("input_17.txt")?;
    part1(&contents)?;
    part2(&contents)?;
    Ok(())
}

fn part1(contents: &str) -> Result<i64> {
    let mut actives: HashSet<(i32, i32, i32)> = contents
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars().enumerate().filter_map(move |(j, c)| {
                if c == A {
                    Some((i as i32, j as i32, 0))
                } else {
                    None
                }
            })
        })
        .collect();
    for _i in 0..6 {
        let mut active_neighbors: HashMap<(i32, i32, i32), i32> = HashMap::new();
        for a in actives.iter() {
            for j in -1..2 {
                for k in -1..2 {
                    for l in -1..2 {
                        if j == 0 && k == 0 && l == 0 {
                            continue;
                        }
                        let c = (a.0 + j, a.1 + k, a.2 + l);
                        active_neighbors.insert(c, *active_neighbors.get(&c).unwrap_or(&0) + 1);
                    }
                }
            }
        }
        let mut new_actives: HashSet<(i32, i32, i32)> = HashSet::new();
        for c in actives.iter() {
            let count = active_neighbors.get(&c).unwrap_or(&0);
            if *count == 2 || *count == 3 {
                new_actives.insert(*c);
            }
        }
        for (c, count) in active_neighbors.iter() {
            if *count == 3 && actives.get(&c).is_none() {
                new_actives.insert(*c);
            }
        }
        actives = new_actives.clone();
    }
    println!("{}", actives.len());
    Ok(actives.len() as i64)
}

fn part2(contents: &str) -> Result<i64> {
    let mut actives: HashSet<(i32, i32, i32, i32)> = contents
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars().enumerate().filter_map(move |(j, c)| {
                if c == A {
                    Some((i as i32, j as i32, 0, 0))
                } else {
                    None
                }
            })
        })
        .collect();
    for _i in 0..6 {
        let mut active_neighbors: HashMap<(i32, i32, i32, i32), i32> = HashMap::new();
        for a in actives.iter() {
            for j in -1..2 {
                for k in -1..2 {
                    for l in -1..2 {
                        for m in -1..2 {
                            if j == 0 && k == 0 && l == 0 && m == 0 {
                                continue;
                            }
                            let c = (a.0 + j, a.1 + k, a.2 + l, a.3 + m);
                            active_neighbors.insert(c, *active_neighbors.get(&c).unwrap_or(&0) + 1);
                        }
                    }
                }
            }
        }
        let mut new_actives: HashSet<(i32, i32, i32, i32)> = HashSet::new();
        for c in actives.iter() {
            let count = active_neighbors.get(&c).unwrap_or(&0);
            if *count == 2 || *count == 3 {
                new_actives.insert(*c);
            }
        }
        for (c, count) in active_neighbors.iter() {
            if *count == 3 && actives.get(&c).is_none() {
                new_actives.insert(*c);
            }
        }
        actives = new_actives.clone();
    }
    println!("{}", actives.len());
    Ok(actives.len() as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            112,
            part1(
                ".#.
..#
###"
            )
            .unwrap()
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            848,
            part2(
                ".#.
..#
###"
            )
            .unwrap()
        );
    }
}
