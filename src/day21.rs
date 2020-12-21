use anyhow::{anyhow, Result};
use aoc::soln;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;

#[soln]
pub fn main() -> Result<()> {
    let contents = fs::read_to_string("input_21.txt")?;
    let (ingredients, possible_allergen_ingredients) = parse(&contents)?;
    part1(&ingredients, &possible_allergen_ingredients)?;
    part2(&possible_allergen_ingredients)?;
    Ok(())
}

fn parse(contents: &str) -> Result<(Vec<(Vec<&str>, Vec<&str>)>, HashMap<&str, HashSet<&str>>)> {
    let ingredients: Vec<(Vec<&str>, Vec<&str>)> = contents
        .lines()
        .map(|l| {
            let caps: Vec<&str> = l.trim_end_matches(')').split(" (contains ").collect();
            (
                caps[0].split(' ').collect::<Vec<&str>>(),
                caps[1].split(", ").collect::<Vec<&str>>(),
            )
        })
        .collect();
    let possible_allergen_ingredients: HashMap<&str, HashSet<&str>> =
        ingredients
            .iter()
            .fold(HashMap::new(), |mut acc, (ingredients, allergens)| {
                for a in allergens {
                    let set = ingredients
                        .iter()
                        .map(|s| s.clone())
                        .collect::<HashSet<&str>>();
                    let e = acc.entry(a.clone()).or_insert(set.clone());
                    *e = e
                        .intersection(&set)
                        .map(|s| s.clone())
                        .collect::<HashSet<&str>>();
                }
                acc
            });
    Ok((ingredients, possible_allergen_ingredients))
}

fn part1(
    ingredients: &Vec<(Vec<&str>, Vec<&str>)>,
    possible_allergen_ingredients: &HashMap<&str, HashSet<&str>>,
) -> Result<i64> {
    let possible_ingredients: HashSet<&str> = possible_allergen_ingredients
        .values()
        .flat_map(|is| is.iter().map(|i| *i))
        .collect();
    let excluded_ingredients: Vec<&str> = ingredients
        .into_iter()
        .flat_map(|(is, _as)| is.into_iter())
        .filter_map(|i| {
            if possible_ingredients.contains(i) {
                None
            } else {
                Some(*i)
            }
        })
        .collect();
    let num_excluded_ingredients = excluded_ingredients.len() as i64;
    println!("num_excluded_ingredients={}", num_excluded_ingredients);
    Ok(num_excluded_ingredients)
}

fn part2(possible_allergen_ingredients: &HashMap<&str, HashSet<&str>>) -> Result<String> {
    let mut ingredient_to_allergen: HashMap<&str, &str> = HashMap::new();
    while possible_allergen_ingredients.len() != ingredient_to_allergen.len() {
        for (all, ings) in possible_allergen_ingredients {
            if ings
                .into_iter()
                .filter(|ing| !ingredient_to_allergen.contains_key(*ing))
                .count()
                == 1
            {
                ingredient_to_allergen.insert(
                    ings.into_iter()
                        .filter(|ing| !ingredient_to_allergen.contains_key(*ing))
                        .next()
                        .ok_or(anyhow!("couldn't find single ingredient"))?,
                    all,
                );
            }
        }
    }
    let ings_sorted = ingredient_to_allergen
        .iter()
        .sorted_by(|(_i1, a1), (_i2, a2)| a1.cmp(a2))
        .map(|(i, _a)| *i)
        .collect::<Vec<&str>>()
        .join(",");
    println!("{}", ings_sorted);
    Ok(ings_sorted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let (ingredients, possible_allergen_ingredients) = parse(
            "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)",
        )?;
        assert_eq!(5, part1(&ingredients, &possible_allergen_ingredients)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let (_ingredients, possible_allergen_ingredients) = parse(
            "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)",
        )?;
        assert_eq!(
            "mxmxvkd,sqjhc,fvjkl",
            part2(&possible_allergen_ingredients)?
        );
        Ok(())
    }
}
