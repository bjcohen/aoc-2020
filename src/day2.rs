use aoc::soln;
use regex::Regex;
use std::fs;
use std::io::{self, BufRead};

#[soln]
pub fn day2() {
    if let Ok(file) = fs::File::open("input_2.txt") {
        let lines = io::BufReader::new(file).lines();
        let re = Regex::new(r"(\d+)\-(\d+) (\w): (\w+)").unwrap();
        let mut good_pws = 0;
        for line in lines {
            if let Ok(line) = line {
                let cap = re.captures(&line).unwrap();
                let lb = cap[1].parse::<usize>().unwrap();
                let ub = cap[2].parse::<usize>().unwrap();
                let c = &cap[3].chars().nth(0).unwrap();
                let pw = &cap[4];
                let count = pw.chars().filter(|x| x == c).count();
                if count >= lb && count <= ub {
                    good_pws += 1;
                }
            }
        }
        println!("({}) good passwords", good_pws);
    }
    if let Ok(file) = fs::File::open("input_2.txt") {
        let lines = io::BufReader::new(file).lines();
        let re = Regex::new(r"(\d+)\-(\d+) (\w): (\w+)").unwrap();
        let mut good_pws = 0;
        for line in lines {
            if let Ok(line) = line {
                let cap = re.captures(&line).unwrap();
                let pos1 = cap[1].parse::<usize>().unwrap();
                let pos2 = cap[2].parse::<usize>().unwrap();
                let c = &cap[3].chars().nth(0).unwrap();
                let pw = &cap[4];
                if (&pw.chars().nth(pos1 - 1).unwrap() == c)
                    ^ (&pw.chars().nth(pos2 - 1).unwrap() == c)
                {
                    good_pws += 1;
                }
            }
        }
        println!("({}) good passwords", good_pws);
    }
}
