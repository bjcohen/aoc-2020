use aoc::soln;
use std::fs;
use std::io::{self, BufRead};

#[soln]
pub fn day3() -> io::Result<()> {
    let file = fs::File::open("input_3.txt")?;
    let lines = io::BufReader::new(file).lines();
    let trees: Vec<Vec<bool>> = lines
        .map(|l| l.unwrap().chars().map(|c| c == '#').collect())
        .collect();
    println!("Hit ({}) trees", count_trees_hit(&trees, 3, 1));
    println!(
        "Product of all the trees hit is ({})",
        count_trees_hit(&trees, 1, 1)
            * count_trees_hit(&trees, 3, 1)
            * count_trees_hit(&trees, 5, 1)
            * count_trees_hit(&trees, 7, 1)
            * count_trees_hit(&trees, 1, 2)
    );
    Ok(())
}

fn count_trees_hit(trees: &Vec<Vec<bool>>, r: usize, d: usize) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut tree_count = 0;
    let width = trees[0].len();
    while y < trees.len() {
        if trees[y][x] {
            tree_count += 1;
        }
        x = (x + r) % width;
        y = y + d;
    }
    tree_count
}
