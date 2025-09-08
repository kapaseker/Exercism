#[derive(Debug, PartialEq, Eq)]
pub struct Dna(Vec<char>);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(Vec<char>);

///  (A), cytosine (C), guanine (G) and thymine (T).
///    G -> C
///    C -> G
///    T -> A
///    A -> U
impl Dna {
    /// Construct new Dna from '{dna}' string. If string contains invalid nucleotides return index of first invalid nucleotide
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let mut dna_chars = vec![];
        for (i, c) in dna.chars().enumerate() {
            if matches!(c, 'A' | 'C' | 'G' | 'T') {
                dna_chars.push(c)
            } else {
                return Err(i);
            }
        }
        Ok(Dna(dna_chars))
    }

    /// Transform Dna {self:?} into corresponding Rna
    pub fn into_rna(self) -> Rna {
        let mut rna_chars = vec![];
        for c in self.0 {
            rna_chars.push(match c {
                'A' => 'U',
                'C' => 'G',
                'G' => 'C',
                _ => 'A',
            })
        }
        Rna(rna_chars)
    }
}

///  (A), cytosine (C), guanine (G) and uracil (U).
impl Rna {
    /// Construct new Rna from '{rna}' string. If string contains invalid nucleotides return index of first invalid nucleotide
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let mut rna_chars = vec![];
        for (i, c) in rna.chars().enumerate() {
            if matches!(c, 'A' | 'C' | 'G' | 'U') {
                rna_chars.push(c)
            } else {
                return Err(i);
            }
        }
        Ok(Rna(rna_chars))
    }
}

