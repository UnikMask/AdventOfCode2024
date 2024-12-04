use advent_of_code_2024_utils::load_file;

fn main() {
    let contents = load_file("day04/input.txt")
        .iter()
        .map(|s| s.chars().collect::<Box<[_]>>())
        .collect::<Box<[_]>>();

    println!("sln 1: {}", sln1(&contents));
    println!("sln 2: {}", sln2(&contents));
}

fn sln1(v: &[Box<[char]>]) -> u64 {
    let mut total_sln1 = 0;
    for (i, line) in v.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c != 'X' {
                continue;
            }
            if j >= 3 {
                if format!("{}{}{}{}", c, line[j - 1], line[j - 2], line[j - 3]) == "XMAS" {
                    total_sln1 += 1;
                }
                if i >= 3
                    && format!(
                        "{}{}{}{}",
                        c,
                        v[i - 1][j - 1],
                        v[i - 2][j - 2],
                        v[i - 3][j - 3]
                    ) == "XMAS"
                {
                    total_sln1 += 1;
                }
                if i < line.len() - 3
                    && format!(
                        "{}{}{}{}",
                        c,
                        v[i + 1][j - 1],
                        v[i + 2][j - 2],
                        v[i + 3][j - 3]
                    ) == "XMAS"
                {
                    total_sln1 += 1;
                }
            }
            if j < line.len() - 3 {
                if format!("{}{}{}{}", c, line[j + 1], line[j + 2], line[j + 3]) == "XMAS" {
                    total_sln1 += 1;
                }
                if i >= 3
                    && format!(
                        "{}{}{}{}",
                        c,
                        v[i - 1][j + 1],
                        v[i - 2][j + 2],
                        v[i - 3][j + 3]
                    ) == "XMAS"
                {
                    total_sln1 += 1;
                }
                if i < line.len() - 3
                    && format!(
                        "{}{}{}{}",
                        c,
                        v[i + 1][j + 1],
                        v[i + 2][j + 2],
                        v[i + 3][j + 3]
                    ) == "XMAS"
                {
                    total_sln1 += 1;
                }
            }
            if i >= 3 && format!("{}{}{}{}", c, v[i - 1][j], v[i - 2][j], v[i - 3][j]) == "XMAS" {
                total_sln1 += 1;
            }
            if i < line.len() - 3
                && format!("{}{}{}{}", c, v[i + 1][j], v[i + 2][j], v[i + 3][j]) == "XMAS"
            {
                total_sln1 += 1;
            }
        }
    }
    total_sln1
}

fn sln2(v: &[Box<[char]>]) -> u64 {
    let mut total_sln2 = 0;
    for (i, line) in v.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c != 'A' || !(i > 0 && i < v.len() - 1 && j > 0 && j < line.len() - 1) {
                continue;
            }
            let potentials = [
                (i - 1, j - 1),
                (i + 1, j - 1),
                (i + 1, j + 1),
                (i - 1, j + 1),
            ];
            for x in 0..potentials.len() {
                let m0 = v[potentials[x].0][potentials[x].1];
                let m1 = v[potentials[(x + 1) % 4].0][potentials[(x + 1) % 4].1];
                let s0 = v[potentials[(x + 2) % 4].0][potentials[(x + 2) % 4].1];
                let s1 = v[potentials[(x + 3) % 4].0][potentials[(x + 3) % 4].1];
                if m0 == 'M' && m0 == m1 && s0 == 'S' && s0 == s1 {
                    total_sln2 += 1;
                    break;
                }
            }
        }
    }
    total_sln2
}
