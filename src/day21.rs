use anyhow::Result;
use aoc::soln;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

#[soln]
pub fn main() -> Result<()> {
    let contents = fs::read_to_string("input_21.txt")?;
    part1(&contents)?;
    part2(&contents)?;
    Ok(())
}

fn parse(
    contents: &str,
) -> Result<(
    Vec<(Vec<String>, Vec<String>)>,
    HashMap<String, HashSet<String>>,
)> {
    let re = Regex::new(r"(.*)*\(contains (.*)\)").unwrap();
    let ingredients: Vec<(Vec<String>, Vec<String>)> = contents
        .lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            (
                caps[1]
                    .trim()
                    .split(' ')
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>(),
                caps[2]
                    .split(", ")
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>(),
            )
        })
        .collect();
    let possible_allergen_ingredients: HashMap<String, HashSet<String>> =
        ingredients
            .iter()
            .fold(HashMap::new(), |mut acc, (ingredients, allergens)| {
                for a in allergens {
                    let set = ingredients
                        .iter()
                        .map(|s| s.clone())
                        .collect::<HashSet<String>>();
                    let e = acc.entry(a.clone()).or_insert(set.clone());
                    *e = e
                        .intersection(&set)
                        .map(|s| s.clone())
                        .collect::<HashSet<String>>();
                }
                acc
            });
    Ok((ingredients, possible_allergen_ingredients))
}

fn part1(contents: &str) -> Result<i64> {
    let (ingredients, possible_allergen_ingredients) = parse(contents)?;
    let possible_ingredients: HashSet<String> = possible_allergen_ingredients
        .values()
        .flat_map(|is| is.iter().map(|i| i.clone()))
        .collect();
    let excluded_ingredients: Vec<String> = ingredients
        .iter()
        .flat_map(|(is, _as)| is.iter())
        .filter_map(|i| {
            if possible_ingredients.contains(i) {
                None
            } else {
                Some(i.clone())
            }
        })
        .collect();
    let num_excluded_ingredients = excluded_ingredients.len() as i64;
    println!("num_excluded_ingredients={}", num_excluded_ingredients);
    Ok(num_excluded_ingredients)
}

fn part2(contents: &str) -> Result<String> {
    let (_ingredients, possible_allergen_ingredients) = parse(contents)?;
    let mut i = possible_allergen_ingredients.clone();
    let mut pairs: Vec<(String, String)> = vec![];
    while i.len() > 0 {
        let pairs_: Vec<(String, String)> = i
            .iter()
            .filter(|(_ing, alls)| alls.len() == 1)
            .map(|(ing, alls)| (ing.clone(), alls.iter().next().unwrap().clone()))
            .collect();
        pairs.extend(pairs_.clone());
        for (ing, all) in pairs_ {
            i.remove(&ing);
            for (_, v) in i.iter_mut() {
                v.remove(&all);
            }
        }
    }
    pairs.sort_by(|(a1, _), (a2, _)| a1.cmp(a2));
    let ings_sorted = pairs
        .into_iter()
        .map(|(_, i)| i)
        .collect::<Vec<String>>()
        .join(",");
    println!("{}", ings_sorted);
    Ok(ings_sorted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(
            5,
            part1(
                "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"
            )?
        );
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(
            "mxmxvkd,sqjhc,fvjkl",
            part2(
                "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"
            )?
        );
        Ok(())
    }
}
