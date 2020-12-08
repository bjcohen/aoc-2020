use anyhow;
use aoc::soln;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::Hash;
use std::iter::FromIterator;

#[derive(Debug)]
struct HashMultimap<K, V>(HashMap<K, Vec<V>>);

impl<K, V> FromIterator<(K, V)> for HashMultimap<K, V>
where
    K: Eq + Hash + Clone,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> HashMultimap<K, V> {
        let mut map: HashMap<K, Vec<V>> = HashMap::new();
        for (k, v) in iter {
            if !map.contains_key(&k) {
                map.insert(k.clone(), Vec::new());
            }
            map.get_mut(&k).unwrap().push(v);
        }
        HashMultimap(map)
    }
}

impl<K, V> FromIterator<(K, Vec<V>)> for HashMultimap<K, V>
where
    K: Eq + Hash + Clone,
{
    fn from_iter<I: IntoIterator<Item = (K, Vec<V>)>>(iter: I) -> HashMultimap<K, V> {
        HashMultimap(iter.into_iter().collect())
    }
}

#[soln]
pub fn main() -> anyhow::Result<()> {
    let contents = fs::read_to_string("input_7.txt").unwrap();
    let contents = contents.trim();
    println!("({}) bad colors", part1(&contents));
    println!(
        "Shiny gold bags must contain ({}) other bags",
        part2(&contents)
    );
    Ok(())
}

fn part1(contents: &str) -> u32 {
    let re_outer = Regex::new(r"(\w+ \w+) bags contain (.*)\.").unwrap();
    let re_inner = Regex::new(r"\d+ (\w+ \w+) bags?").unwrap();
    let can_be_in: HashMultimap<String, String> = contents
        .lines()
        .flat_map(|l| {
            let captures = re_outer.captures(l).expect(l);
            if captures[2].eq("no other bags") {
                return vec![];
            }
            captures[2]
                .split(',')
                .map(|x| {
                    (
                        re_inner.captures(x).expect(x)[1].to_string(),
                        captures[1].to_string(),
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let mut queue: Vec<String> = vec!["shiny gold".to_string()];
    let mut colors: HashSet<String> = HashSet::new();
    while !queue.is_empty() {
        let color = queue.pop().unwrap();
        if let Some(color_can_be_in) = can_be_in.0.get(&color) {
            for color_can_be_in_color in color_can_be_in {
                colors.insert(color_can_be_in_color.to_string());
            }
            queue.extend_from_slice(color_can_be_in);
        }
    }
    colors.len() as u32
}

fn part2(contents: &str) -> u32 {
    let re_outer = Regex::new(r"(\w+ \w+) bags contain (.*)\.").unwrap();
    let re_inner = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();
    let must_contain: HashMultimap<String, (u32, String)> = contents
        .lines()
        .map(|l| {
            let captures = re_outer.captures(l).expect(l);
            if captures[2].eq("no other bags") {
                return (captures[1].to_string(), vec![]);
            }
            (
                captures[1].to_string(),
                captures[2]
                    .split(',')
                    .map(|x| {
                        let captures_inner = re_inner.captures(x).expect(x);
                        (
                            captures_inner[1].parse().unwrap(),
                            captures_inner[2].to_string(),
                        )
                    })
                    .collect(),
            )
        })
        .collect();
    let mut queue: Vec<(u32, String)> = vec![(1, "shiny gold".to_string())];
    let mut bags = 0;
    while !queue.is_empty() {
        let (n, c) = queue.pop().unwrap();
        bags += n;
        if let Some(bag_must_contain) = must_contain.0.get(&c) {
            queue.extend(
                bag_must_contain
                    .into_iter()
                    .map(|(bn, bc)| (bn * n, bc.to_string())),
            );
        }
    }
    bags - 1
}
