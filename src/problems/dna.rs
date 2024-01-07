use itertools::Itertools;

pub fn solve() -> String {
    let input = include_str!("../input/dna.txt");
    return count_nucleotides(input);
}

fn count_nucleotides(input: &str) -> String {
    let counts = input.chars().counts();
    counts.keys()
        .sorted()
        .map(|&n| counts.get(&n).unwrap().to_string())
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let sample = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
        assert_eq!("20 12 17 21", count_nucleotides(sample))
    }
}
