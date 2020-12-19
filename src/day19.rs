use anyhow::Result;
use aoc::soln;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum R {
    Or(Vec<i64>, Vec<i64>),
    Concat(Vec<i64>),
    Term(char),
}

#[soln]
pub fn main() -> Result<()> {
    let contents = fs::read_to_string("input_19.txt")?;
    part1(&contents)?;
    part2(&contents)?;
    Ok(())
}

fn parse(rules_str: &str) -> HashMap<i64, R> {
    let mut rules: HashMap<i64, R> = HashMap::new();
    for line in rules_str.lines() {
        let s: Vec<&str> = line.split(": ").collect();
        let n: i64 = s[0].parse().unwrap();
        if s[1] == "\"a\"" {
            rules.insert(n, R::Term('a'));
        } else if s[1] == "\"b\"" {
            rules.insert(n, R::Term('b'));
        } else if s[1].contains('|') {
            let ps: Vec<&str> = s[1].split(" | ").collect();
            let l = ps[0].split(' ').map(|i| i.parse().unwrap()).collect();
            let r = ps[1].split(' ').map(|i| i.parse().unwrap()).collect();
            rules.insert(n, R::Or(l, r));
        } else {
            let rs = s[1].split(' ').map(|i| i.parse().unwrap()).collect();
            rules.insert(n, R::Concat(rs));
        }
    }
    rules
}

fn part1(contents: &str) -> Result<i64> {
    let split: Vec<&str> = contents.split("\n\n").collect();
    let rules = parse(split[0]);
    let n_valid = split[1]
        .lines()
        .filter(|l| eval1(l, &rules, 0).iter().any(|r| r.is_empty()))
        .count();
    println!("n_valid={}", n_valid);
    Ok(n_valid as i64)
}

fn eval1<'a>(line: &'a str, rules: &HashMap<i64, R>, rn: i64) -> Vec<&'a str> {
    let rule = rules.get(&rn).unwrap();
    match rule {
        R::Or(l, r) => {
            let mut l_ss = vec![line];
            for t in l {
                l_ss = l_ss.iter().flat_map(|s| eval1(s, rules, *t)).collect();
                if l_ss.is_empty() {
                    break;
                }
            }
            let mut r_ss = vec![line];
            for t in r {
                r_ss = r_ss.iter().flat_map(|s| eval1(s, rules, *t)).collect();
                if r_ss.is_empty() {
                    break;
                }
            }
            l_ss.append(&mut r_ss);
            l_ss
        }
        R::Concat(ts) => {
            let mut ss = vec![line];
            for t in ts {
                ss = ss.iter().flat_map(|s| eval1(s, rules, *t)).collect();
                if ss.is_empty() {
                    break;
                }
            }
            ss
        }
        R::Term(c) => {
            let ss = if line.len() > 0 && line.chars().next().unwrap() == *c {
                vec![&line[1..]]
            } else {
                vec![]
            };
            ss
        }
    }
}

fn part2(contents: &str) -> Result<i64> {
    let split: Vec<&str> = contents.split("\n\n").collect();
    let mut rules = parse(split[0]);
    rules.insert(8, R::Or(vec![42], vec![42, 8]));
    rules.insert(11, R::Or(vec![42, 31], vec![42, 11, 31]));
    let n_valid = split[1]
        .lines()
        .filter(|l| eval1(l, &rules, 0).iter().any(|r| r.is_empty()))
        .count();
    println!("n_valid={}", n_valid);
    Ok(n_valid as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            2,
            part1(
                r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#
            )
            .unwrap()
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            12,
            part2(
                r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#
            )
            .unwrap()
        );
    }
}
