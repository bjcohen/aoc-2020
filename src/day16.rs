use anyhow::Result;
use aoc::soln;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[soln]
pub fn main() -> Result<()> {
    let contents = fs::read_to_string("input_16.txt")?;
    let notes = parse(&contents)?;
    part1(&notes)?;
    part2(&notes)?;
    Ok(())
}

#[derive(Debug, PartialEq)]
struct Notes<'a> {
    field_specs: HashMap<&'a str, (i64, i64, i64, i64)>,
    your_ticket: Vec<i64>,
    nearby_tickets: Vec<Vec<i64>>,
}

fn parse(notes: &str) -> Result<Notes> {
    let mut phase = 0;
    let mut field_specs = HashMap::new();
    let mut your_ticket = vec![];
    let mut nearby_tickets = vec![];
    for line in notes.lines() {
        if line.is_empty() {
            phase += 1;
        } else if phase == 0 {
            let s: Vec<&str> = line.split(": ").collect();
            let re = Regex::new(r"(\d+)-(\d+) or (\d+)-(\d+)").unwrap();
            let captures = re.captures(s[1]).unwrap();
            field_specs.insert(
                s[0],
                (
                    captures[1].parse().unwrap(),
                    captures[2].parse().unwrap(),
                    captures[3].parse().unwrap(),
                    captures[4].parse().unwrap(),
                ),
            );
        } else if phase == 1 {
            if line.eq("your ticket:") {
                continue;
            }
            your_ticket = line
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i64>>();
        } else if phase == 2 {
            if line.eq("nearby tickets:") {
                continue;
            }
            nearby_tickets.push(
                line.split(',')
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<i64>>(),
            );
        }
    }
    Ok(Notes {
        field_specs,
        your_ticket,
        nearby_tickets,
    })
}

fn part1(notes: &Notes) -> Result<i64> {
    let sum = notes
        .nearby_tickets
        .iter()
        .filter_map(|t| is_valid_ticket(t, &notes.field_specs).err())
        .sum();
    println!("sum={}", sum);
    Ok(sum)
}

fn is_valid_ticket(
    ticket: &Vec<i64>,
    field_specs: &HashMap<&str, (i64, i64, i64, i64)>,
) -> Result<(), i64> {
    for v in ticket {
        let any_valid = field_specs
            .values()
            .any(|(l0, h0, l1, h1)| (v >= l0 && v <= h0) || (v >= l1 && v <= h1));
        if !any_valid {
            return Err(*v);
        }
    }
    Ok(())
}

fn part2(notes: &Notes) -> Result<i64> {
    let mut possible_fields: HashMap<&str, Vec<bool>> = notes
        .field_specs
        .keys()
        .map(|k| (*k, vec![true; notes.your_ticket.len()]))
        .collect();
    for nearby_ticket in notes.nearby_tickets.iter() {
        if is_valid_ticket(&nearby_ticket, &notes.field_specs).is_err() {
            continue;
        }
        for (i, v) in nearby_ticket.iter().enumerate() {
            for (field, (l0, h0, l1, h1)) in notes.field_specs.iter() {
                if !((v >= l0 && v <= h0) || (v >= l1 && v <= h1)) {
                    possible_fields.get_mut(field).unwrap()[i] = false;
                }
            }
        }
    }
    let mut prod = 1;
    let mut seen_departure_fields = 0;
    while seen_departure_fields < 6 {
        let (field, flags) = possible_fields
            .iter()
            .filter(|(_field, flags)| flags.iter().filter(|&&f| f).count() == 1)
            .nth(0)
            .unwrap();
        let i = flags.iter().position(|&f| f).unwrap();
        if field.starts_with("departure") {
            seen_departure_fields += 1;
            prod *= notes.your_ticket[i];
        }
        for flags in possible_fields.values_mut() {
            flags[i] = false;
        }
    }
    println!("prod={}", prod);
    Ok(prod)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_notes() -> Notes<'static> {
        let mut field_specs = HashMap::new();
        field_specs.insert("class", (1, 3, 5, 7));
        field_specs.insert("row", (6, 11, 33, 44));
        field_specs.insert("seat", (13, 40, 45, 50));
        Notes {
            field_specs,
            your_ticket: vec![7, 1, 14],
            nearby_tickets: vec![
                vec![7, 3, 47],
                vec![40, 4, 50],
                vec![55, 2, 20],
                vec![38, 6, 12],
            ],
        }
    }

    #[test]
    fn test_parse() {
        let notes = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        assert_eq!(get_test_notes(), parse(notes).unwrap());
    }

    #[test]
    fn test_part1() {
        assert_eq!(71, part1(&get_test_notes()).unwrap());
    }
}
