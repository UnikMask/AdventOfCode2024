use advent_of_code_2024_utils::load_file;

fn main() {
    let safes = load_file("day02/input.txt")
        .iter()
        .map(|e| {
            e.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|v| {
            v.iter().fold((true, None, None), |(res, last, plast), i| {
                match (last, plast) {
                    (None, None) => (true, Some(i), None),
                    (Some(j), None) => (
                        res && (i - j).abs() <= 3 && (i - j).abs() >= 1,
                        Some(i),
                        Some(j),
                    ),
                    (Some(j), Some(k)) => (
                        res && (i - j).abs() <= 3
                            && (i - j).abs() >= 1
                            && ((j > k && i > j) || (j < k && i < j)),
                        Some(i),
                        Some(j),
                    ),
                    _ => (false, None, None),
                }
            }).0
        })
        .collect::<Vec<_>>();
    println!("sln 1: {}", safes.len());
}
