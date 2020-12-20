use anyhow::Result;
use aoc::soln;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

#[soln]
pub fn main() -> Result<()> {
    let contents = fs::read_to_string("input_20.txt")?;
    part1_2(&contents)?;
    Ok(())
}
fn part1_2(contents: &str) -> Result<i64> {
    let tiles: HashMap<i64, Vec<&str>> =
        contents.split("\n\n").fold(HashMap::new(), |mut acc, t| {
            let ls: Vec<&str> = t.lines().collect();
            let tn: i64 = ls[0][5..9].parse().unwrap();
            acc.insert(tn, ls.into_iter().skip(1).collect());
            acc
        });
    let adjacencies: HashMap<i64, Vec<(i64, &str)>> = tiles
        .iter()
        .map(|(tn, ls)| {
            let mut t = 0;
            let mut t_r = 0;
            let mut b = 0;
            let mut b_r = 0;
            let mut l = 0;
            let mut l_r = 0;
            let mut r = 0;
            let mut r_r = 0;
            for i in 0..10 {
                if ls[0].chars().nth(i).unwrap() == '#' {
                    t |= 1 << i;
                    t_r |= 1 << (9 - i);
                }
                if ls[9].chars().nth(i).unwrap() == '#' {
                    b |= 1 << i;
                    b_r |= 1 << (9 - i);
                }
                if ls[i].chars().nth(0).unwrap() == '#' {
                    l |= 1 << i;
                    l_r |= 1 << (9 - i);
                }
                if ls[i].chars().nth(9).unwrap() == '#' {
                    r |= 1 << i;
                    r_r |= 1 << (9 - i);
                }
            }
            vec![
                (t, tn, "0"),
                (t_r, tn, "0f"),
                (b, tn, "2"),
                (b_r, tn, "2f"),
                (l, tn, "3"),
                (l_r, tn, "3f"),
                (r, tn, "1"),
                (r_r, tn, "1f"),
            ]
        })
        .fold(HashMap::new(), |mut acc, ps| {
            for (e, tn, o) in ps {
                acc.entry(e).or_insert(vec![]).push((*tn, o));
            }
            acc
        });
    let num_neighbors: HashMap<i64, i64> = adjacencies.iter().filter(|(_k, v)| v.len() > 1).fold(
        HashMap::new(),
        |mut acc, (_e, ps)| {
            for (tn, _o) in ps {
                *acc.entry(*tn).or_insert(0) += 1;
            }
            acc
        },
    );
    let num_neighbors_is_four: Vec<i64> = num_neighbors
        .iter()
        .filter_map(|(k, v)| if *v == 4 { Some(*k) } else { None })
        .collect();
    let prod = num_neighbors_is_four.iter().product();
    println!("{:?} {}", num_neighbors_is_four, prod);

    let adjacent_tiles: HashMap<i64, Vec<(&str, &str, i64)>> = adjacencies
        .iter()
        .filter(|(_k, v)| v.len() > 1)
        .fold(HashMap::new(), |mut acc, (_e, ps)| {
            let (tn1, o1) = ps[0];
            let (tn2, o2) = ps[1];
            acc.entry(tn1).or_insert(vec![]).push((o1, o2, tn2));
            acc.entry(tn2).or_insert(vec![]).push((o2, o1, tn1));
            acc
        });
    // 0 -> 0f, 1f, 2, 3
    // 1 -> 0f, 1f, 2, 3
    // 2 -> 0, 1, 2f, 3f
    // 3 -> 0, 1, 2f, 3f
    let mut orientations: Vec<Vec<(i64, &str)>> = Vec::with_capacity(12);
    for i in 0..12 {
        for j in 0..12 {
            if (i, j) == (0, 0) {
                let start_tile = num_neighbors_is_four.iter().min().unwrap();
                let start_tile_adjacencies: Vec<&str> = adjacent_tiles
                    .get(start_tile)
                    .unwrap()
                    .iter()
                    .map(|adj| adj.0)
                    .filter(|a| a.len() == 1)
                    .sorted()
                    .collect();
                orientations.push(
                    match (start_tile_adjacencies[0], start_tile_adjacencies[1]) {
                        ("0", "3") => vec![(*start_tile, "2")],
                        ("0", "1") => vec![(*start_tile, "1")],
                        ("1", "2") => vec![(*start_tile, "0")],
                        ("2", "3") => vec![(*start_tile, "3")],
                        _ => unreachable!(),
                    },
                );
            } else if j > 0 {
                let (last_tile, last_orientation) = orientations[i][j - 1];
                // println!("{:?}", adjacent_tiles.get_key_value(&last_tile).unwrap());
                let (_, e2, t2) = adjacent_tiles
                    .get(&last_tile)
                    .unwrap()
                    .iter()
                    .find(|(e1, _e2, _t2)| match last_orientation {
                        "0" => *e1 == "1",
                        "0f" => *e1 == "1f",
                        "1" => *e1 == "0",
                        "1f" => *e1 == "0f",
                        "2" => *e1 == "3f",
                        "2f" => *e1 == "3",
                        "3" => *e1 == "2f",
                        "3f" => *e1 == "2",
                        _ => unreachable!(),
                    })
                    .unwrap();
                orientations[i].push((
                    *t2,
                    match *e2 {
                        "0" => "3f",
                        "0f" => "3",
                        "1" => "2f",
                        "1f" => "2",
                        "2" => "1",
                        "2f" => "1f",
                        "3" => "0",
                        "3f" => "0f",
                        _ => unreachable!(),
                    },
                ));
            } else {
                let (last_tile, last_orientation) = orientations[i - 1][j];
                // println!("{:?}", adjacent_tiles.get_key_value(&last_tile).unwrap());
                let (_, e2, t2) = adjacent_tiles
                    .get(&last_tile)
                    .unwrap()
                    .iter()
                    .find(|(e1, _e2, _t2)| match last_orientation {
                        "0" => *e1 == "2",
                        "0f" => *e1 == "0",
                        "1" => *e1 == "1f",
                        "1f" => *e1 == "3f",
                        "2" => *e1 == "0f",
                        "2f" => *e1 == "2f",
                        "3" => *e1 == "3",
                        "3f" => *e1 == "1",
                        _ => unreachable!(),
                    })
                    .unwrap();
                orientations.push(vec![(
                    *t2,
                    match *e2 {
                        "0" => "0",
                        "0f" => "2f",
                        "1" => "3",
                        "1f" => "1f",
                        "2" => "0f",
                        "2f" => "2",
                        "3" => "3f",
                        "3f" => "1",
                        _ => unreachable!(),
                    },
                )]);
            }
            // println!("{:?}", orientations);
        }
    }
    let mut full_composite_chars: Vec<Vec<char>> = vec![vec![' '; 12 * 11]; 12 * 11];
    for (i, row_orientations) in orientations.iter().enumerate() {
        for (j, (tn, o)) in row_orientations.iter().enumerate() {
            let tile = tiles.get(&tn).unwrap();
            let tile_transformed = transform(tile, o);
            for k in 0..10 {
                for l in 0..10 {
                    full_composite_chars[i * 11 + k][j * 11 + l] =
                        tile_transformed[k].chars().nth(l).unwrap();
                }
            }
        }
    }
    let _full_composite: Vec<String> = full_composite_chars
        .into_iter()
        .map(|r| r.into_iter().collect::<String>())
        .collect();
    let mut composite_chars: Vec<Vec<char>> = vec![vec![' '; 12 * 8]; 12 * 8];
    for (i, row_orientations) in orientations.iter().enumerate() {
        for (j, (tn, o)) in row_orientations.iter().enumerate() {
            let tile = tiles.get(&tn).unwrap();
            let tile_transformed = transform(tile, o);
            for k in 0..8 {
                for l in 0..8 {
                    composite_chars[i * 8 + k][j * 8 + l] =
                        tile_transformed[k + 1].chars().nth(l + 1).unwrap();
                }
            }
        }
    }
    let composite: Vec<String> = composite_chars
        .into_iter()
        .map(|r| r.into_iter().collect::<String>())
        .collect();
    let monster_pat: Vec<&str> = "                  # 
#    ##    ##    ###
 #  #  #  #  #  #   "
        .lines()
        .collect();
    for o in &["0", "0f", "1", "1f", "2", "2f", "3", "3f"] {
        let marked_composite = mark_monsters(&composite, &transform(&monster_pat, o));
        println!(
            "o={}, num_monsters={} num_hashes={}",
            o,
            marked_composite
                .iter()
                .flat_map(|l| l.chars())
                .filter(|c| *c == 'O')
                .count(),
            marked_composite
                .iter()
                .flat_map(|l| l.chars())
                .filter(|c| *c == '#')
                .count()
        );
    }
    Ok(prod)
}

fn mark_monsters(composite: &Vec<String>, monster_pat: &Vec<String>) -> Vec<String> {
    let mut composite_chars: Vec<Vec<char>> =
        composite.iter().map(|l| l.chars().collect()).collect();
    for i in 0..composite_chars.len() - monster_pat.len() {
        for j in 0..composite_chars[0].len() - monster_pat[0].len() {
            let mut all_match = true;
            for k in 0..monster_pat.len() {
                for l in 0..monster_pat[0].len() {
                    if monster_pat[k].chars().nth(l).unwrap() == '#'
                        && composite[i + k].chars().nth(j + l).unwrap() != '#'
                    {
                        all_match = false;
                    }
                }
            }
            if all_match {
                for k in 0..monster_pat.len() {
                    for l in 0..monster_pat[0].len() {
                        if monster_pat[k].chars().nth(l).unwrap() == '#' {
                            composite_chars[i + k][j + l] = 'O';
                        }
                    }
                }
            }
        }
    }
    composite_chars.iter().map(|r| r.iter().collect::<String>()).collect()
}

fn transform(tile: &Vec<&str>, o: &str) -> Vec<String> {
    let h = tile.len();
    let w = tile[0].len();
    let tile_chars: Vec<Vec<char>> = tile.iter().map(|s| s.chars().collect()).collect();
    let mut result_chars: Vec<Vec<char>> = match o {
        "0" | "0f" | "2" | "2f" => (0..h).map(|_| vec![' '; w]).collect(),
        "1" | "1f" | "3" | "3f" => (0..w).map(|_| vec![' '; h]).collect(),
        _ => unreachable!(),
    };
    for i in 0..h {
        for j in 0..w {
            match o {
                "0" => result_chars[i][j] = tile_chars[i][j],
                "0f" => result_chars[h - 1 - i][j] = tile_chars[i][j],
                "1" => result_chars[j][h - 1 - i] = tile_chars[i][j],
                "1f" => result_chars[w - 1 - j][h - 1 - i] = tile_chars[i][j],
                "2" => result_chars[h - 1 - i][w - 1 - j] = tile_chars[i][j],
                "2f" => result_chars[i][w - 1 - j] = tile_chars[i][j],
                "3" => result_chars[w - 1 - j][i] = tile_chars[i][j],
                "3f" => result_chars[j][i] = tile_chars[i][j],
                _ => unreachable!(),
            }
        }
    }
    result_chars
        .into_iter()
        .map(|r| r.into_iter().collect::<String>())
        .collect()
}
