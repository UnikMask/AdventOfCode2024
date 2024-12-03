use advent_of_code_2024_utils::load_file;

pub enum ParseState {
    M,
    U,
    L,
    D,
    O,
    N,
    Apostrophe,
    T,
    DoBracket,
    DontBracket,
    BracketUp,
    N0,
    N1,
}

fn main() {
    let instructions = load_file("day03/input.txt").join("");

    println!("Sln 1: {}", get_total(&instructions, false));
    println!("Sln 2: {}", get_total(&instructions, true));
}

fn get_total(input: &str, conditionals: bool) -> u32 {
    let (mut total, mut n0, mut n1) = (0, None, None);
    let mut do_state = true;
    let mut state = ParseState::M;

    for c in input.chars() {
        state = match (state, c) {
            (_, 'd') => ParseState::D,
            (ParseState::D, 'o') => ParseState::O,
            (ParseState::O, '(') => ParseState::DoBracket,
            (ParseState::DoBracket, ')') => {
                do_state = true;
                ParseState::M
            }
            (ParseState::O, 'n') => ParseState::N,
            (ParseState::N, '\'') => ParseState::Apostrophe,
            (ParseState::Apostrophe, 't') => ParseState::T,
            (ParseState::T, '(') => ParseState::DontBracket,
            (ParseState::DontBracket, ')') => {
                do_state = false;
                ParseState::M
            }
            (ParseState::M, 'm') => ParseState::U,
            (ParseState::U, 'u') => ParseState::L,
            (ParseState::L, 'l') => ParseState::BracketUp,
            (ParseState::BracketUp, '(') => {
                n0 = None;
                ParseState::N0
            }
            (ParseState::N0, '0'..='9') => match (n0, c) {
                (Some(n), _) => {
                    n0 = Some(n * 10 + c.to_digit(10).unwrap());
                    ParseState::N0
                }
                (None, '1'..='9') => {
                    n0 = Some(c.to_digit(10).unwrap());
                    ParseState::N0
                }
                _ => ParseState::M,
            },
            (ParseState::N0, ',') => {
                n1 = None;
                if n0.is_none() {
                    ParseState::M
                } else {
                    ParseState::N1
                }
            }
            (ParseState::N1, '0'..='9') => match (n1, c) {
                (Some(n), _) => {
                    n1 = Some(n * 10 + c.to_digit(10).unwrap());
                    ParseState::N1
                }
                (None, '1'..='9') => {
                    n1 = Some(c.to_digit(10).unwrap());
                    ParseState::N1
                }
                _ => ParseState::M,
            },
            (ParseState::N1, ')') => match (n0, n1, do_state || !conditionals) {
                (Some(n0), Some(n1), true) => {
                    total += n0 * n1;
                    ParseState::M
                }
                _ => ParseState::M,
            },
            _ => ParseState::M,
        }
    }
    total
}
