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

    fn is_correct(ordering: &[i64], rules: &HashMap<i64, Vec<i64>>) -> bool {
        let mut prohibited: HashSet<i64> = HashSet::new();
        ordering.iter().all(|n| match prohibited.get(n) {
            Some(_) => false,
            None => {
                if let Some(new_prohibits) = rules.get(n) {
                    for pn in new_prohibits {
                        prohibited.insert(*pn);
                    }
                }
                true
            }
        })
    }

    let total_sln1: i64 = orderings
        .iter()
        .filter(|ordering| is_correct(ordering, &prohibits))
        .map(|v| v[v.len() / 2])
        .sum();
    println!("sln 1: {}", total_sln1);

    let total_sln2: i64 =
        orderings
            .iter()
            .filter(|ordering| !is_correct(ordering, &prohibits))
            .map(|ordering| {
                (
                    ordering,
                    ordering
                        .iter()
                        .filter_map(|o| prohibits.get(o))
                        .flatten()
                        .fold(HashMap::new(), |mut map, p| {
                            *map.entry(*p).or_default() += 1;
                            map
                        }),
                )
            })
            .map(|(ordering, mut prohibited)| {
                (0..ordering.len())
                    .map(|_| {
                        let o = ordering
                            .iter()
                            .find(|o| !prohibited.contains_key(o))
                            .unwrap();
                        prohibits.get(o).unwrap_or(&Vec::new()).iter().for_each(
                            |p| match prohibited.get(p) {
                                Some(1) => {
                                    prohibited.remove(p);
                                }
                                Some(_) => {
                                    *prohibited.entry(*p).or_default() -= 1;
                                }
                                _ => {}
                            },
                        );
                        *prohibited.entry(*o).or_default() += 1;
                        *o
                    })
                    .collect::<Vec<_>>()
            })
            .map(|v| v[v.len() / 2])
            .sum();
    println!("sln 2: {}", total_sln2);
}
