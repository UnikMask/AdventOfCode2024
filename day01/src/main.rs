use std::collections::{BinaryHeap, HashMap};

use advent_of_code_2024_utils::load_file;

#[derive(Default, Clone, Debug)]
struct Pair<T>(T, T);

impl<T: Default> Pair<T> {
    fn new(p0: T, p1: T) -> Self {
        Self(p0, p1)
    }
}

fn main() {
    println!("Hello, world!");

    let splits: Pair<BinaryHeap<i64>> = load_file("./input.txt")
        .iter()
        .map(|line| {
            let mut split = line.split_whitespace();
            Pair::<i64>::new(
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .fold(Pair::<BinaryHeap<i64>>::default(), |mut acc, e| {
            acc.0.push(e.0);
            acc.1.push(e.1);
            acc
        });
    let mut splits_p0 = splits.clone();
    let sln1 = (0..splits.0.len()).fold(0, |mut acc, _| {
        acc += (splits_p0.0.pop().unwrap() - splits_p0.1.pop().unwrap()).abs(); 
        acc
    });
    println!("Part 1 solution: {}", sln1);

    let mut l0_map = splits.0.iter().fold(HashMap::new(), |mut acc, e| {
        if let Some(Pair(i, _)) = acc.get(e) {
            acc.insert(e, Pair::new(i + 1, 0_i64));
        } else {
            acc.insert(e, Pair::new(1_i64, 0_i64));
        }
        acc
    });
    splits.1.iter().for_each(|e| {
        if let Some(entry) =  l0_map.get_mut(e) {
            entry.1 += 1;
        }
    });
    let sln2 = l0_map.iter().fold(0, |acc, (i, e)| {
        acc + (*i * e.0 * e.1)
    });
    println!("Part 2 solution: {}", sln2);
}
