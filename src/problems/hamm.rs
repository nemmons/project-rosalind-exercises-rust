use std::iter::zip;

pub fn solve() -> String {
    let input = include_str!("../input/hamm.txt");
    let (first, second) = input.trim().split_once('\n').unwrap();
    return calc_hamming_distance(first,second).to_string();
}

fn calc_hamming_distance(first: &str, second: &str) -> usize {
    zip(first.chars(), second.chars())
        .filter(|(a,b)| { a != b })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let sample = "GAGCCTACTAACGGGAT
CATCGTAATGACGGCCT";
        let (first, second) = sample.trim().split_once('\n').unwrap();
        assert_eq!(7, calc_hamming_distance(first, second))
    }
}
