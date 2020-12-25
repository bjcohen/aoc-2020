use anyhow::Result;
use aoc::soln;

#[soln]
pub fn main() -> Result<()> {
    let key1 = 13316116;
    let key2 = 13651422;
    part1(key1, key2)?;
    Ok(())
}
fn part1(key1: u64, key2: u64) -> Result<u64> {
    let mut v = 1;
    let s = 7;
    let mut n1 = None;
    let mut n2 = None;
    let mut i = 0;
    while n1.is_none() || n2.is_none() {
        v = (v * s) % 20201227;
        if v == key1 {
            n1 = Some(i + 1);
        }
        if v == key2 {
            n2 = Some(i + 1);
        }
        i += 1;
    }
    println!("n1={}, n2={}", n1.unwrap(), n2.unwrap());
    let mut v2 = 1;
    for _ in 0..n1.unwrap() {
        v2 = (v2 * key2) % 20201227;
    }
    let mut v1 = 1;
    for _ in 0..n2.unwrap() {
        v1 = (v1 * key1) % 20201227;
    }
    println!("v1={} v2={}", v1, v2);
    Ok(v)
}
