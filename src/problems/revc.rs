use std::collections::HashMap;
use itertools::Itertools;

pub fn solve() -> String {
    let input = include_str!("../input/revc.txt");
    return reverse_complement(input);
}

fn reverse_complement(dna: &str) -> String {
    let complements = HashMap::from([
        ('A', 'T'),
        ('T', 'A'),
        ('G', 'C'),
        ('C', 'G')
    ]);

    dna.trim()
        .chars()
        .rev()
        .map(|n| complements.get(&n).unwrap())
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let sample = "AAAACCCGGT";
        assert_eq!("ACCGGGTTTT", reverse_complement(sample))
    }
}

