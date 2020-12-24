use anyhow::Result;
use aoc::soln;
use std::collections::{HashMap, HashSet};
use std::fs;

#[soln]
pub fn main() -> Result<()> {
    let contents = fs::read_to_string("input_24.txt")?;
    part1(&contents)?;
    part2(&contents)?;
    Ok(())
}

fn parse(contents: &str) -> Result<HashMap<(i64, i64), i64>> {
    Ok(contents
        .lines()
        .map(|l| {
            let mut x = 0;
            let mut z = 0;
            let mut chars = l.chars();
            loop {
                match chars.next() {
                    Some('e') => x += 1,
                    Some('w') => x -= 1,
                    Some('s') => match chars.next() {
                        Some('e') => z += 1,
                        Some('w') => {
                            x -= 1;
                            z += 1;
                        }
                        Some(_) | None => unreachable!(),
                    },
                    Some('n') => match chars.next() {
                        Some('e') => {
                            x += 1;
                            z -= 1;
                        }
                        Some('w') => z -= 1,
                        Some(_) | None => unreachable!(),
                    },
                    None => return (x, z),
                    Some(_) => unreachable!(),
                }
            }
        })
        .fold(HashMap::new(), |mut acc, coords| {
            *acc.entry(coords).or_insert(0) += 1;
            acc
        }))
}

fn part1(contents: &str) -> Result<usize> {
    let counts = parse(contents)?;
    let num_black = counts.iter().filter(|(_, c)| *c % 2 == 1).count();
    println!("num_black={}", num_black);
    Ok(num_black)
}

fn get_neighbors<'a>(coords: &(i64, i64)) -> impl Iterator<Item = (i64, i64)> + 'a {
    let (x, z) = *coords;
    vec![
        (x + 1, z),
        (x, z + 1),
        (x - 1, z + 1),
        (x - 1, z),
        (x, z - 1),
        (x + 1, z - 1),
    ]
    .into_iter()
}

fn part2(contents: &str) -> Result<usize> {
    let counts = parse(contents)?;
    let mut is_black: HashSet<(i64, i64)> = counts
        .iter()
        .filter_map(|(&k, v)| if v % 2 == 1 { Some(k) } else { None })
        .collect();
    for _ in 0..100 {
        let num_adjacent: HashMap<(i64, i64), i64> =
            is_black
                .iter()
                .flat_map(&get_neighbors)
                .fold(HashMap::new(), |mut acc, coords| {
                    *acc.entry(coords).or_insert(0) += 1;
                    acc
                });
        let is_black_: HashSet<(i64, i64)> = num_adjacent
            .iter()
            .filter_map(|(coords, count)| {
                if is_black.get(coords).is_some() {
                    if *count == 0 || *count > 2 {
                        None
                    } else {
                        Some(*coords)
                    }
                } else {
                    if *count == 2 {
                        Some(*coords)
                    } else {
                        None
                    }
                }
            })
            .collect();
        is_black = is_black_;
    }
    let num_black = is_black.len();
    println!("num_black={}", num_black);
    Ok(num_black)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> &'static str {
        "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(10, part1(get_test_data())?);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(2208, part2(get_test_data())?);
        Ok(())
    }
}
