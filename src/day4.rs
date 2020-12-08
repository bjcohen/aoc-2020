use anyhow;
use aoc::soln;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Passport {
    byr: Option<u32>,
    iyr: Option<u32>,
    eyr: Option<u32>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<u32>,
}

impl Passport {
    fn is_valid1(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid2(&self) -> bool {
        self.byr.is_some()
            && self.byr.unwrap() >= 1920
            && self.byr.unwrap() <= 2002
            && self.iyr.is_some()
            && self.iyr.unwrap() >= 2010
            && self.iyr.unwrap() <= 2020
            && self.eyr.is_some()
            && self.eyr.unwrap() >= 2020
            && self.eyr.unwrap() <= 2030
            && self.hgt.is_some()
            && self._check_hgt()
            && self.hcl.is_some()
            && Regex::new(r"^#[0-9a-f]{6}$")
                .unwrap()
                .is_match(self.hcl.as_ref().unwrap())
            && self.ecl.is_some()
            && Regex::new(r"(amb|blu|brn|gry|grn|hzl|oth)$")
                .unwrap()
                .is_match(self.ecl.as_ref().unwrap())
            && self.pid.is_some()
            && Regex::new(r"^[0-9]{9}$")
                .unwrap()
                .is_match(self.pid.as_ref().unwrap())
    }

    fn _check_hgt(&self) -> bool {
        let captures = Regex::new(r"^(\d+)(cm|in)$")
            .unwrap()
            .captures(self.hgt.as_ref().unwrap());
        if captures.is_none() {
            return false;
        }
        let captures = captures.unwrap();
        match &captures[2] {
            "cm" => {
                let n: u32 = captures[1].parse().unwrap();
                n >= 150 && n <= 193
            }
            "in" => {
                let n: u32 = captures[1].parse().unwrap();
                n >= 59 && n <= 76
            }
            _ => false,
        }
    }
}

impl FromStr for Passport {
    type Err = ();

    fn from_str(s: &str) -> Result<Passport, ()> {
        let mut parsed: HashMap<&str, &str> = HashMap::new();
        for pair in s.split_whitespace() {
            let pair_parsed = pair.split(":").collect::<Vec<_>>();
            parsed.insert(pair_parsed[0], pair_parsed[1]);
        }
        Ok(Passport {
            byr: parsed.get("byr").map(|s| s.parse().unwrap()),
            iyr: parsed.get("iyr").map(|s| s.parse().unwrap()),
            eyr: parsed.get("eyr").map(|s| s.parse().unwrap()),
            hgt: parsed.get("hgt").map(|s| s.to_string()),
            hcl: parsed.get("hcl").map(|s| s.to_string()),
            ecl: parsed.get("ecl").map(|s| s.to_string()),
            pid: parsed.get("pid").map(|s| s.to_string()),
            cid: parsed.get("cid").map(|s| s.parse().unwrap()),
        })
    }
}

#[soln]
pub fn main() -> anyhow::Result<()> {
    let file_contents = fs::read_to_string("input_4.txt")?;
    let passports: Result<Vec<Passport>, _> = file_contents
        .split("\n\n")
        .map(|p| p.parse::<Passport>())
        .collect();
    let passports = passports.unwrap();
    let num_valid1 = passports.iter().filter(|p| p.is_valid1()).count();
    println!("Found ({}) valid passports.", num_valid1);
    let num_valid2 = passports.iter().filter(|p| p.is_valid2()).count();
    println!("Found ({}) valid passports.", num_valid2);
    Ok(())
}
