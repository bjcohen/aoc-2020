use anyhow::Result;
use aoc::soln;
use std::cmp::min;
use std::fs;

const F: char = '.';
const E: char = 'L';
const O: char = '#';

#[soln]
pub fn main() -> Result<()> {
    let contents = fs::read_to_string("input_11.txt")?;
    let seats: Vec<&str> = contents.lines().collect();
    part1(&seats)?;
    part2(&seats)?;
    Ok(())
}

fn part1(seats: &Vec<&str>) -> Result<i64> {
    run_sim(seats, 4, &num_adjacent_occupied)
}

fn run_sim(
    seats: &Vec<&str>,
    o_e_threshold: i32,
    occupied_fn: &dyn Fn(&Vec<String>, usize, usize) -> i32,
) -> Result<i64> {
    let mut curr: Vec<String> = seats.iter().map(|s| s.to_string()).collect();
    loop {
        let mut next: Vec<String> = vec![];
        for i in 0..curr.len() {
            next.push(String::new());
            for j in 0..curr[i].len() {
                match curr[i].chars().nth(j).unwrap() {
                    E => {
                        if occupied_fn(&curr, i, j) == 0 {
                            next[i].push(O);
                        } else {
                            next[i].push(E);
                        }
                    }
                    O => {
                        if occupied_fn(&curr, i, j) >= o_e_threshold {
                            next[i].push(E);
                        } else {
                            next[i].push(O);
                        }
                    }
                    F => next[i].push(F),
                    _ => unreachable!(),
                }
            }
        }
        if curr == next {
            let num_occupied = curr
                .iter()
                .flat_map(|s| s.chars())
                .filter(|c| *c == O)
                .count();
            println!("Num occupied seats: {}", num_occupied);
            return Ok(num_occupied as i64);
        }
        curr = next;
    }
}

fn num_adjacent_occupied(seats: &Vec<String>, i: usize, j: usize) -> i32 {
    let si = if i == 0 { 0 } else { i - 1 };
    let ei = min(i + 1, seats.len() - 1);
    let sj = if j == 0 { 0 } else { j - 1 };
    let ej = min(j + 1, seats[0].len() - 1);
    let mut n = 0;
    for ii in si..ei + 1 {
        for jj in sj..ej + 1 {
            if i == ii && j == jj {
                continue;
            }
            if seats[ii].chars().nth(jj).unwrap() == O {
                n += 1;
            }
        }
    }
    n
}

fn part2(seats: &Vec<&str>) -> Result<i64> {
    run_sim(seats, 5, &num_seen_occupied)
}

fn num_seen_occupied(seats: &Vec<String>, i: usize, j: usize) -> i32 {
    let directions: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut seen_occupied = 0;
    for (dx, dy) in directions {
        let mut c = 1;
        loop {
            let ii = (i as i32) + dx * c;
            let jj = (j as i32) + dy * c;
            if ii >= seats.len() as i32 || ii < 0 || jj >= seats[0].len() as i32 || jj < 0 {
                break;
            } else {
                match seats[ii as usize].chars().nth(jj as usize).unwrap() {
                    E => break,
                    O => {
                        seen_occupied += 1;
                        break;
                    }
                    F => {}
                    _ => unreachable!(),
                }
            }
            c += 1;
        }
    }
    seen_occupied
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            37,
            part1(&vec![
                "L.LL.LL.LL",
                "LLLLLLL.LL",
                "L.L.L..L..",
                "LLLL.LL.LL",
                "L.LL.LL.LL",
                "L.LLLLL.LL",
                "..L.L.....",
                "LLLLLLLLLL",
                "L.LLLLLL.L",
                "L.LLLLL.LL",
            ])
            .unwrap()
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            26,
            part2(&vec![
                "L.LL.LL.LL",
                "LLLLLLL.LL",
                "L.L.L..L..",
                "LLLL.LL.LL",
                "L.LL.LL.LL",
                "L.LLLLL.LL",
                "..L.L.....",
                "LLLLLLLLLL",
                "L.LLLLLL.L",
                "L.LLLLL.LL",
            ])
            .unwrap()
        );
    }
}
