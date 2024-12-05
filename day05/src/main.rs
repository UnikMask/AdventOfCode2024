use std::collections::{HashMap, HashSet};

use advent_of_code_2024_utils::load_file;

fn main() {
    let rules: Vec<(i64, i64)> = load_file("day05/input.txt")
        .iter()
        .take_while(|s| s.split("|").nth(1).is_some())
        .map(|s| {
            let mut split = s.split("|");
            (
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect();
    let mut prohibits: HashMap<i64, Vec<i64>> = HashMap::new();
    for (before, after) in &rules {
        prohibits.entry(*after).or_default().push(*before);
    }

    let orderings: Vec<Vec<i64>> = load_file("day05/input.txt")[rules.len() + 1..]
        .iter()
        .map(|s| s.split(",").map(|n| n.parse().unwrap()).collect::<Vec<_>>())
        .collect();

    let total_sln1: i64 = orderings
        .iter()
        .filter(|ordering| {
            let mut prohibited: HashSet<i64> = HashSet::new();
            ordering.iter().all(|n| match prohibited.get(n) {
                Some(_) => false,
                None => {
                    if let Some(new_prohibits) = prohibits.get(n) {
                        for pn in new_prohibits {
                            prohibited.insert(*pn);
                        }
                    }
                    true
                }
            })
        })
        .map(|ordering| ordering[ordering.len() / 2])
        .sum();
    println!("sln 1: {}", total_sln1);

    let total_sln2: i64 = orderings
        .iter()
        .filter(|ordering| {
            let mut prohibited: HashSet<i64> = HashSet::new();
            ordering.iter().any(|n| match prohibited.get(n) {
                Some(_) => true,
                None => {
                    if let Some(new_prohibits) = prohibits.get(n) {
                        for pn in new_prohibits {
                            prohibited.insert(*pn);
                        }
                    }
                    false
                }
            })
        })
        .map(|ordering| {
            let mut prohibited: HashMap<i64, i64> = HashMap::new();
            let mut res: Vec<i64> = Vec::new();
            for o in ordering {
                for p in prohibits.get(o).unwrap_or(&Vec::new()) {
                    *prohibited.entry(*p).or_insert(0) += 1;
                }
            }
            while res.len() < ordering.len() {
                for o in ordering {
                    if prohibited.contains_key(o) {
                        continue;
                    }
                    for p in prohibits.get(o).unwrap_or(&Vec::new()) {
                        *prohibited.entry(*p).or_insert(1) -= 1;
                        if let Some(0) = prohibited.get(p) {
                            prohibited.remove(p);
                        }
                    }
                    *prohibited.entry(*o).or_insert(0) += 1;
                    res.push(*o);
                }
            }
            res[res.len() / 2]
        })
        .sum();
    println!("sln 2: {}", total_sln2);
}
