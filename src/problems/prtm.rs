use std::collections::HashMap;
pub fn solve() -> String {
    let protein = include_str!("../input/prtm.txt");
    return calculate_mass(protein);
}

fn calculate_mass(protein: &str) -> String {
    let monoisotopic_mass_table = HashMap::from([
        ('A', 71.03711),
        ('C', 103.00919),
        ('D', 115.02694),
        ('E', 129.04259),
        ('F', 147.06841),
        ('G', 57.02146),
        ('H', 137.05891),
        ('I', 113.08406),
        ('K', 128.09496),
        ('L', 113.08406),
        ('M', 131.04049),
        ('N', 114.04293),
        ('P', 97.05276),
        ('Q', 128.05858),
        ('R', 156.10111),
        ('S', 87.03203),
        ('T', 101.04768),
        ('V', 99.06841),
        ('W', 186.07931),
        ('Y', 163.06333),
    ]);

    let weight = protein.trim().chars()
        .into_iter()
        .map(|aa| {
            monoisotopic_mass_table.get(&aa).unwrap()
        })
        .sum::<f64>();

    return format!("{:.3}", weight);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let protein = "SKADYEK";
        assert_eq!("821.392", calculate_mass(protein))
    }
}
