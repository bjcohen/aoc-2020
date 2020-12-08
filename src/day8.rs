use anyhow::{bail, ensure, Result};
use aoc::soln;
use std::fs;

#[soln]
pub fn main() -> Result<()> {
    let contents = fs::read_to_string("input_8.txt").unwrap();
    let mut lines: Vec<(&str, &str)> = contents
        .trim()
        .lines()
        .map(|l| {
            let mut s = l.split(' ');
            (s.next().unwrap(), s.next().unwrap())
        })
        .collect();
    part1(&lines)?;
    part2(&mut lines)?;
    Ok(())
}

enum RunResult {
    Loop(i32),
    Term(i32),
}

fn run(lines: &Vec<(&str, &str)>) -> Result<RunResult> {
    let mut acc: i32 = 0;
    let mut pc: i32 = 0;
    let mut used: Vec<bool> = vec![false; lines.len()];
    while pc < lines.len() as i32 {
        if used[pc as usize] {
            return Ok(RunResult::Loop(acc));
        }
        used[pc as usize] = true;
        let instr = lines[pc as usize].0;
        let arg: i32 = lines[pc as usize].1.parse()?;
        match instr {
            "acc" => {
                acc += arg;
                pc += 1;
            }
            "jmp" => pc += arg,
            "nop" => pc += 1,
            _ => bail!("Unhandled instruction [{}]", instr),
        }
        ensure!(pc > 0, "PC must be greater than 0 but was {}", pc);
    }
    Ok(RunResult::Term(acc))
}

fn part1(lines: &Vec<(&str, &str)>) -> Result<()> {
    match run(&lines)? {
        RunResult::Loop(res) => println!("Acc value was ({})", res),
        _ => bail!("Unexpected Term result"),
    }
    Ok(())
}

fn part2(lines: &mut Vec<(&str, &str)>) -> Result<()> {
    let mut result = 0;
    for i in 0..lines.len() {
        let instr = lines[i].0;
        let arg = lines[i].1;
        match instr {
            "acc" => continue,
            "jmp" => {
                lines[i] = ("nop", arg);
                match run(lines)? {
                    RunResult::Loop(_) => (),
                    RunResult::Term(res) => {
                        result = res;
                        break;
                    }
                }
                lines[i] = ("jmp", arg);
            }
            "nop" => {
                lines[i] = ("jmp", arg);
                match run(lines)? {
                    RunResult::Loop(_) => (),
                    RunResult::Term(res) => {
                        result = res;
                        break;
                    }
                }
                lines[i] = ("nop", arg);
            }
            _ => bail!("Unhandled instruction [{}]", instr),
        }
    }
    println!("Acc value was ({})", result);
    Ok(())
}
