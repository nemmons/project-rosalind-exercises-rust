use itertools::Itertools;
use regex::Regex;

pub fn solve() -> String {
    let input = include_str!("../input/gc.txt");
    return find_highest_gc_content(input);
}

fn find_highest_gc_content(input: &str) -> String {
    let regex = Regex::new(r"\s?>(Rosalind_\d+)\s+([ATGC\s]+)").unwrap();

    let dna_strings: Vec<(&str, &str, f32)> = regex.captures_iter(input).map(|captures| {
        let (_, [label, dna]) = captures.extract();
        (label, dna, calc_gc_content(dna))
    }).collect();

    let (label, _, content) = dna_strings.iter()
        .max_by(|(_, _, gc_content),(_, _, other_gc_content)| { gc_content.partial_cmp(other_gc_content).unwrap()}).unwrap();

    return format!("{}
{:.6}",label,content*100f32);

}

fn calc_gc_content(input: &str) -> f32 {
    let removed_whitespace: String = input.split_whitespace().collect(); //remove newlines etc from middle of DNA strngs
    let counts = removed_whitespace.chars().counts();
    let gc_counts = counts.get(&'G').unwrap() + counts.get(&'C').unwrap();

    (gc_counts as f32) / (removed_whitespace.chars().count() as f32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let sample = "\
>Rosalind_6404
CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCC
TCCCACTAATAATTCTGAGG
>Rosalind_5959
CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCT
ATATCCATTTGTCAGCAGACACGC
>Rosalind_0808
CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGAC
TGGGAACCTGCGGGCAGTAGGTGGAAT";
        assert_eq!("Rosalind_0808
60.919540", find_highest_gc_content(sample))
    }
}
