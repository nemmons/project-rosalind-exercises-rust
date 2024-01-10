use std::collections::HashMap;
use itertools::Itertools;

pub fn solve() -> String {
    let rna = include_str!("../input/prot.txt");
    return decode_protein(rna);
}

fn decode_protein(rna: &str) -> String {
    let codons = HashMap::from([
        ("UUU", "F"),
        ("CUU", "L"),
        ("AUU", "I"),
        ("GUU", "V"),
        ("UUC", "F"),
        ("CUC", "L"),
        ("AUC", "I"),
        ("GUC", "V"),
        ("UUA", "L"),
        ("CUA", "L"),
        ("AUA", "I"),
        ("GUA", "V"),
        ("UUG", "L"),
        ("CUG", "L"),
        ("AUG", "M"),
        ("GUG", "V"),
        ("UCU", "S"),
        ("CCU", "P"),
        ("ACU", "T"),
        ("GCU", "A"),
        ("UCC", "S"),
        ("CCC", "P"),
        ("ACC", "T"),
        ("GCC", "A"),
        ("UCA", "S"),
        ("CCA", "P"),
        ("ACA", "T"),
        ("GCA", "A"),
        ("UCG", "S"),
        ("CCG", "P"),
        ("ACG", "T"),
        ("GCG", "A"),
        ("UAU", "Y"),
        ("CAU", "H"),
        ("AAU", "N"),
        ("GAU", "D"),
        ("UAC", "Y"),
        ("CAC", "H"),
        ("AAC", "N"),
        ("GAC", "D"),
        ("CAA", "Q"),
        ("AAA", "K"),
        ("GAA", "E"),
        ("CAG", "Q"),
        ("AAG", "K"),
        ("GAG", "E"),
        ("UGU", "C"),
        ("CGU", "R"),
        ("AGU", "S"),
        ("GGU", "G"),
        ("UGC", "C"),
        ("CGC", "R"),
        ("AGC", "S"),
        ("GGC", "G"),
        ("CGA", "R"),
        ("AGA", "R"),
        ("GGA", "G"),
        ("UGG", "W"),
        ("CGG", "R"),
        ("AGG", "R"),
        ("GGG", "G"),
        ("UGA", ""), //STOP
        ("UAA", ""), //STOP
        ("UAG", "") //STOP
    ]);

    rna.trim().chars()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let codon: String = chunk.collect::<String>();
            codons.get(codon.as_str()).unwrap().to_string()
        })
        .collect::<String>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let rna = "AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA";
        assert_eq!("MAMAPRTEINSTRING", decode_protein(rna))
    }
}
