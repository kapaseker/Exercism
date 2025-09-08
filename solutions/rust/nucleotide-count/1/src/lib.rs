use std::collections::HashMap;
//A, C, G, or T 
pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if matches!(nucleotide,'A' | 'C' | 'G' | 'T') {
        let mut count = 0;
        for x in dna.chars() {
            if matches!(x,'A' | 'C' | 'G' | 'T') {
                if x == nucleotide {
                    count += 1;
                }
            } else {
                return Err(x);
            }
        }
        Ok(count)
    } else {
        Err(nucleotide)
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut dna_map = HashMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0)]);
    for x in dna.chars() {
        if matches!(x,'A' | 'C' | 'G' | 'T') {
            dna_map.entry(x).and_modify(|c| *c += 1);
        } else {
            return Err(x);
        }
    }
    Ok(dna_map)
}
