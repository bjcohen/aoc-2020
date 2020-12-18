use anyhow::{ensure, Result};
use aoc::soln;
use std::fs;

#[soln]
pub fn main() -> Result<()> {
    let contents = fs::read_to_string("input_18.txt")?;
    part1(&contents)?;
    part2(&contents)?;
    Ok(())
}
fn part1(contents: &str) -> Result<i64> {
    let sum = contents.lines().map(|l| eval1(l).unwrap()).sum();
    println!("sum={}", sum);
    Ok(sum)
}

fn eval1(e: &str) -> Result<i64> {
    let mut args: Vec<Option<i64>> = vec![];
    let mut ops: Vec<char> = vec![];
    for c in e.chars() {
        match c {
            ' ' => continue,
            '*' | '+' => ops.push(c),
            '(' => args.push(None),
            ')' => {
                let a = args.pop().unwrap();
                let n = args.pop().unwrap();
                let b = args.pop();
                ensure!(n.is_none(), "n must be none but was {:?}", n);
                match b {
                    Some(Some(b)) => match ops.pop().unwrap() {
                        '*' => args.push(Some(a.unwrap() * b)),
                        '+' => args.push(Some(a.unwrap() + b)),
                        _ => unreachable!(),
                    },
                    Some(None) => {
                        args.push(None);
                        args.push(a);
                    }
                    None => {
                        args.push(a);
                    }
                }
            }
            _ => {
                let b = c.to_digit(10).unwrap().into();
                if args.last().is_none() || args.last().unwrap().is_none() {
                    args.push(Some(b));
                } else {
                    let a = args.pop().unwrap().unwrap();
                    match ops.pop().unwrap() {
                        '*' => args.push(Some(a * b)),
                        '+' => args.push(Some(a + b)),
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
    while ops.len() > 0 {
        let b = args.pop().unwrap().unwrap();
        let a = args.pop().unwrap().unwrap();
        match ops.pop().unwrap() {
            '*' => args.push(Some(a * b)),
            '+' => args.push(Some(a + b)),
            _ => unreachable!(),
        }
    }
    Ok(args[0].unwrap())
}

fn part2(contents: &str) -> Result<i64> {
    let sum = contents
        .lines()
        .map(|l| eval2(&mut l.chars()).unwrap())
        .sum();
    println!("sum={}", sum);
    Ok(sum)
}

#[derive(Debug)]
enum E {
    Oper(char),
    Val(i64),
}

fn eval2(e: &mut dyn Iterator<Item = char>) -> Result<i64> {
    let mut expr: Vec<E> = vec![];
    loop {
        let c = e.next();
        if c.is_none() || c.unwrap() == ')' {
            break;
        } else if c.unwrap() == ' ' {
            continue;
        } else if c.unwrap() == '(' {
            expr.push(E::Val(eval2(e).unwrap()));
        } else if c.unwrap() == '+' || c.unwrap() == '*' {
            expr.push(E::Oper(c.unwrap()));
        } else {
            expr.push(E::Val(c.unwrap().to_digit(10).unwrap().into()));
        }
    }
    let mut mults: Vec<i64> = vec![];
    let mut running_sum = 0;
    for i in 0..expr.len() / 2 {
        match expr[2 * i + 1] {
            E::Oper(o) => {
                if let E::Val(v) = expr[2 * i] {
                    running_sum += v;
                } else {
                    unreachable!();
                }
                if o == '*' {
                    mults.push(running_sum);
                    running_sum = 0;
                }
            }
            E::Val(_v) => unreachable!(),
        }
    }
    if let E::Val(v) = expr.last().unwrap() {
        running_sum += v;
    } else {
        unreachable!();
    }
    mults.push(running_sum);
    Ok(mults.iter().product())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval1() {
        assert_eq!(71, eval1("1 + 2 * 3 + 4 * 5 + 6").unwrap());
        assert_eq!(26, eval1("2 * 3 + (4 * 5)").unwrap());
        assert_eq!(437, eval1("5 + (8 * 3 + 9 + 3 * 4 * 3)").unwrap());
        assert_eq!(
            12240,
            eval1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").unwrap()
        );
        assert_eq!(
            13632,
            eval1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap()
        );
    }

    #[test]
    fn test_eval2() {
        assert_eq!(231, eval2(&mut "1 + 2 * 3 + 4 * 5 + 6".chars()).unwrap());
        assert_eq!(
            51,
            eval2(&mut "1 + (2 * 3) + ((4 * (5 + 6))".chars()).unwrap()
        );
        assert_eq!(46, eval2(&mut "2 * 3 + (4 * 5)".chars()).unwrap());
        assert_eq!(
            1445,
            eval2(&mut "5 + (8 * 3 + 9 + 3 * 4 * 3)".chars()).unwrap()
        );
        assert_eq!(
            669060,
            eval2(&mut "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".chars()).unwrap()
        );
        assert_eq!(
            23340,
            eval2(&mut "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".chars()).unwrap()
        );
    }
}
