use std::{fs::read_to_string, vec::Vec};

pub fn load_file(fp: &str) -> Vec<String> {
    read_to_string(fp)
        .expect("File doesn't exist!")
        .lines()
        .map(|s| s.to_owned())
        .collect()
}
