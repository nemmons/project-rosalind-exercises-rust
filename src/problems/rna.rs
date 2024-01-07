pub fn solve() -> String {
    let input = include_str!("../input/rna.txt");
    return transcribe_rna(input);
}

fn transcribe_rna(dna: &str) -> String {
    dna.replace('T', "U")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let sample = "GATGGAACTTGACTACGTAAATT";
        assert_eq!("GAUGGAACUUGACUACGUAAAUU", transcribe_rna(sample))
    }
}
