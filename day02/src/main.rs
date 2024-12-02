use advent_of_code_2024_utils::load_file;

fn main() {
    let sequences = load_file("day02/input.txt")
        .iter()
        .map(|e| {
            e.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let sln1 = sequences
        .iter()
        .filter(|v| get_errors_rec(None, None, None, v) == 0)
        .collect::<Vec<_>>()
        .len();
    println!("sln 1: {}", sln1);
    let sln2 = sequences
        .iter()
        .filter(|v| get_errors_rec(None, None, None, v) <= 1)
        .collect::<Vec<_>>()
        .len();
    println!("sln 2: {}", sln2);
}

fn is_correct(i: i64, last: Option<i64>, plast: Option<i64>) -> bool {
    match last {
        None => true,
        Some(j) => {
            (i - j).abs() <= 3
                && (i - j).abs() >= 1
                && match plast {
                    None => true,
                    Some(k) => (i > j && j > k) || (i < j && j < k),
                }
        }
    }
}

fn get_errors_rec(
    last: Option<i64>,
    plast: Option<i64>,
    pplast: Option<i64>,
    numbers: &[i64],
) -> u32 {
    let i = numbers[0];
    let errored = !is_correct(i, last, plast);
    match (numbers.len(), errored) {
        (1, _) => errored.into(),
        (_, true) => {
            let mut res = get_errors_rec(last, plast, pplast, &numbers[1..]);
            if is_correct(i, plast, pplast) {
                res = res.min(get_errors_rec(Some(i), plast, pplast, &numbers[1..]));
            }
            if is_correct(i, last, pplast) {
                res = res.min(get_errors_rec(Some(i), last, pplast, &numbers[1..]));
            }
            res + 1
        }
        (_, false) => get_errors_rec(Some(i), last, plast, &numbers[1..]),
    }
}
