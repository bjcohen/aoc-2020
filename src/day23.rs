use anyhow::{anyhow, Result};
use aoc::soln;
use std::cell::RefCell;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::rc::Rc;

#[soln]
pub fn main() -> Result<()> {
    let cups = "398254716"
        .chars()
        .map(|c| c.to_string().parse())
        .collect::<Result<Vec<i64>, ParseIntError>>()?;
    part1(&cups)?;
    part2(&cups)?;
    Ok(())
}

#[allow(dead_code)]
fn run_cups_array(cups: &Vec<i64>, n_moves: usize) -> Result<Vec<i64>> {
    let mut cups = cups.clone();
    let mut c = *cups.iter().next().ok_or(anyhow!("no elements in c"))?;
    let min_c = *cups.iter().min().ok_or(anyhow!("no min"))?;
    let max_c = *cups.iter().max().ok_or(anyhow!("no max"))?;
    let n_cups = cups.len();
    for iter in 0..n_moves {
        if iter % 10 == 0 {
            println!("iteration {}", iter);
        }
        let i = cups
            .iter()
            .position(|&c_| c == c_)
            .ok_or(anyhow!("couldn't find c"))?;
        let mut remove_i = if i == n_cups - 1 { 0 } else { i + 1 };
        let mut dest_c = c - 1;
        let mut c0 = vec![];
        for k in 0..3 {
            c0.push(cups.remove(remove_i));
            if remove_i >= n_cups - k - 1 {
                remove_i = 0;
            }
        }
        if dest_c < min_c {
            dest_c = max_c;
        }
        while c0.contains(&dest_c) {
            dest_c -= 1;
            if dest_c < min_c {
                dest_c = max_c;
            }
        }
        let dest_i = cups.iter().position(|&c| c == dest_c).ok_or(anyhow!(
            "couldn't find dest {} in {:?}, c0={:?}",
            dest_c,
            cups,
            c0
        ))?;
        for c in c0.iter().rev() {
            cups.insert(dest_i + 1, *c);
        }
        let mut i = cups
            .iter()
            .position(|&c_| c == c_)
            .ok_or(anyhow!("couldn't find c"))?;
        i += 1;
        if i == n_cups {
            i = 0;
        }
        c = cups[i];
    }
    Ok(cups)
}

#[derive(Debug)]
struct Node {
    v: i64,
    next: Option<Rc<RefCell<Node>>>,
}

fn ll_to_vec(head: Rc<RefCell<Node>>) -> Vec<i64> {
    let mut iter = head.clone();
    let mut result = vec![head.borrow().v];
    let tmp = iter.borrow().next.clone().unwrap();
    iter = tmp;
    while iter.borrow().v != head.borrow().v {
        result.push(iter.borrow().v);
        let tmp = iter.borrow().next.clone().unwrap();
        iter = tmp;
    }
    result
}

fn run_cups_ll(cups: &Vec<i64>, n_moves: usize) -> Result<Vec<i64>> {
    let min_c = *cups.iter().min().ok_or(anyhow!("no min"))?;
    let max_c = *cups.iter().max().ok_or(anyhow!("no max"))?;
    let mut map: HashMap<i64, Rc<RefCell<Node>>> = HashMap::new();
    let mut c = Rc::new(RefCell::new(Node {
        v: *cups.iter().next().ok_or(anyhow!("no elements in c"))?,
        next: None,
    }));
    map.insert(c.borrow().v, c.clone());
    let mut last = c.clone();
    for cup in cups.iter().skip(1) {
        let new_node = Rc::new(RefCell::new(Node {
            v: *cup,
            next: None,
        }));
        map.insert(*cup, new_node.clone());
        last.clone().borrow_mut().next = Some(new_node.clone());
        last = new_node;
    }
    last.borrow_mut().next = Some(c.clone());
    for iter in 0..n_moves {
        if iter % 1_000_000 == 0 {
            println!("move {}", iter);
        }
        let mut c0 = vec![];
        for _ in 0..3 {
            c0.push(c.borrow().next.as_ref().unwrap().borrow().v);
            let n = c.borrow().next.clone().unwrap();
            c.borrow_mut().next = n.borrow().next.clone();
        }
        let mut dest_c = c.borrow().v - 1;
        if dest_c < min_c {
            dest_c = max_c;
        }
        while c0.contains(&dest_c) {
            dest_c -= 1;
            if dest_c < min_c {
                dest_c = max_c;
            }
        }
        let dest = map
            .get(&dest_c)
            .ok_or(anyhow!("couldn't find dest in map"))?;
        let mut tmp = dest.clone();
        for cup in c0 {
            let new_node = Rc::new(RefCell::new(Node {
                v: cup,
                next: Some(tmp.borrow().next.as_ref().unwrap().clone()),
            }));
            map.insert(cup, new_node.clone());
            tmp.clone().borrow_mut().next = Some(new_node.clone());
            tmp = new_node;
        }
        let tmp = c.borrow().next.as_ref().unwrap().clone();
        c = tmp;
    }
    Ok(ll_to_vec(c))
}

fn part1(cups: &Vec<i64>) -> Result<String> {
    let cups = run_cups_ll(&cups, 100)?;
    let one_i = cups
        .iter()
        .position(|&c| c == 1)
        .ok_or(anyhow!("couldn't find 1 in {:?}", cups))?;
    let result =
        cups[(one_i + 1)..]
            .iter()
            .map(|c| c.to_string())
            .fold(String::new(), |mut acc, cs| {
                acc.push_str(&cs);
                acc
            })
            + &cups[..one_i]
                .iter()
                .map(|c| c.to_string())
                .fold(String::new(), |mut acc, cs| {
                    acc.push_str(&cs);
                    acc
                });
    println!("{}", result);
    Ok(result)
}

fn part2(cups: &Vec<i64>) -> Result<i64> {
    let mut cups = cups.clone();
    cups.extend((cups.len() as i64 + 1)..1_000_001);
    let result_cups = run_cups_ll(&cups, 10_000_000)?;
    let one_i = result_cups
        .iter()
        .position(|&c| c == 1)
        .ok_or(anyhow!("couldn't find 1 in {:?}", cups))?;
    let result = result_cups[one_i + 1] * result_cups[one_i + 2];
    println!("{}", result);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
        assert_eq!("67384529", part1(&input)?);
        Ok(())
    }

    #[test]
    fn test_array_ll_equal() -> Result<()> {
        let input = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
        let mut array_result = run_cups_array(&input, 100)?;
        let ll_result = run_cups_ll(&input, 100)?;
        assert_eq!(array_result.len(), ll_result.len());
        assert!((0..array_result.len()).any(|_| {
            array_result.rotate_left(1);
            array_result == ll_result
        }));
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(149245887792, part2(&vec![3, 8, 9, 1, 2, 5, 4, 6, 7])?);
        Ok(())
    }
}
