use advent_of_code_2024_utils::load_file;

fn main() {
    let safes = load_file("day02/input.txt")
        .iter()
        .map(|e| {
            e.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|v| get_errors_rec(None, None, None, v) == 0)
        .collect::<Vec<_>>();
    println!("sln 1: {}", safes.len());

    let safes_hold_one = load_file("day02/input.txt")
        .iter()
        .map(|e| {
            e.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|v| {
            get_errors_rec(None, None, None, v).min(get_errors_rec(None, None, None, &v[1..]) + 1)
                <= 1
        })
        .collect::<Vec<_>>();

    println!("sln 2: {}", safes_hold_one.len());
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
    if numbers.len() == 1 {
        return (!is_correct(i, last, plast)).into();
    }

    if !is_correct(i, last, plast) {
        let mut res = get_errors_rec(last, plast, pplast, &numbers[1..]);
        if is_correct(i, plast, pplast) {
            res = res.min(get_errors_rec(Some(i), plast, pplast, &numbers[1..]));
        }
        return res + 1;
    }
    get_errors_rec(Some(i), last, plast, &numbers[1..])
}
