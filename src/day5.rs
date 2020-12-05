use aoc::soln;
use std::fs;
use std::io::{self, BufRead};

const N_ROW_DESC: usize = 7;
const N_COL_DESC: usize = 3;

#[soln]
pub fn day5() -> io::Result<()> {
    let file = fs::File::open("input_5.txt")?;
    let lines = io::BufReader::new(file).lines();
    let mut seat_ids : Vec<u32> = lines.map(|l| seat_id(&l.unwrap())).collect();
    seat_ids.sort();
    let min = seat_ids[0];
    let max = seat_ids[seat_ids.len() - 1];
    println!("The highest seat id is ({})", max);
    let mut l = min;
    for i in seat_ids.iter().skip(1) {
        if i-1 != l {
            println!("The missing seat id is ({})", i-1);
        }
        l = *i;
    }
    Ok(())
}

fn seat_id(bp: &str) -> u32 {
    let bp = bp.as_bytes();
    let mut r = 0;
    for i in 0..N_ROW_DESC {
        if bp[i] == b'B' {
            r |= 1 << N_ROW_DESC-1-i;
        }
    }
    let mut c = 0;
    for i in 0..N_COL_DESC {
        if bp[N_ROW_DESC+i] == b'R' {
            c |= 1 << N_COL_DESC-1-i;
        }
    }
    r * 8 + c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(seat_id("FBFBBFFRLR"), 357);
        assert_eq!(seat_id("BFFFBBFRRR"), 567);
        assert_eq!(seat_id("FFFBBBFRRR"), 119);
        assert_eq!(seat_id("BBFFBBFRLL"), 820);
    }
}
