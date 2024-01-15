use itertools::{Itertools};
use std::str;

pub fn solve() -> String {
    let input = include_str!("../input/subs.txt");
    let (dna, dna_substring) = input.trim().split_once('\n').unwrap();
    return find_substrings(dna, dna_substring);
}

fn find_substrings(dna: &str, dna_substring: &str) -> String {
    let dna_substring_bytes = dna_substring.as_bytes();
    dna.as_bytes()
        .windows(dna_substring_bytes.len())
        .enumerate()//need to return index of matching windows later
        .filter(|&(_, window)| {
            window == dna_substring_bytes
        })
        .map(|(i, _)| { i+1 }) //convert to 1-based indexing
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let dna = "GATATATGCATATACTT";
        let substring = "ATAT";
        assert_eq!("2 4 10", find_substrings(dna, substring))
    }
}
