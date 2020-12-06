use aoc::soln;
use std::collections::HashSet;
use std::fs;
use std::io;

#[soln]
pub fn day6() -> io::Result<()> {
    let contents = fs::read_to_string("input_6.txt").unwrap();
    let contents = contents.trim();
    let sum_of_counts_any = sum_of_counts_any(contents);
    println!("Sum of counts is ({})", sum_of_counts_any);
    let sum_of_counts_all = sum_of_counts_all(contents);
    println!("Sum of counts is ({})", sum_of_counts_all);
    Ok(())
}

fn sum_of_counts_any(s: &str) -> u32 {
    s.split("\n\n")
        .map(|g| {
            g.chars()
                .filter(|c| *c != '\n')
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

fn sum_of_counts_all(s: &str) -> u32 {
    s.split("\n\n")
        .map(|g| {
            g.lines()
                .map(|l| l.chars().collect::<HashSet<char>>())
                .fold_first(|a, b| a.intersection(&b).cloned().collect())
                .unwrap()
                .len()
        })
        .map(|g| g as u32)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_any() {
        assert_eq!(
            sum_of_counts_any(
                "abc

a
b
c

ab
ac

a
a
a
a

b"
            ),
            11
        );
    }

    #[test]
    fn test_all() {
        assert_eq!(
            sum_of_counts_all(
                "abc

a
b
c

ab
ac

a
a
a
a

b"
            ),
            6
        );
    }

    #[test]
    fn test_all_2() {
        assert_eq!(
            sum_of_counts_all(
                "nxhayv
npqfohbrl
kegchuidstwnm
nzhlj"
            ),
            2
        );
    }
}
