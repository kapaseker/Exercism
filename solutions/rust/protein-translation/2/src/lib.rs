use std::collections::HashMap;

pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let protein_map = HashMap::from([
        ("AUG", "Methionine"),
        ("UUU", "Phenylalanine"),
        ("UUC", "Phenylalanine"),
        ("UUA", "Leucine"),
        ("UUG", "Leucine"),
        ("UCU", "Serine"),
        ("UCC", "Serine"),
        ("UCA", "Serine"),
        ("UCG", "Serine"),
        ("UAU", "Tyrosine"),
        ("UAC", "Tyrosine"),
        ("UGU", "Cysteine"),
        ("UGC", "Cysteine"),
        ("UGG", "Tryptophan"),
        ("UAA", ""),
        ("UAG", ""),
        ("UGA", ""),
    ]);
    let mut protein: Vec<&str> = vec![];
    if !rna.is_empty() {
        let chunked: Vec<String> = Vec::from_iter(rna.chars()).chunks(3).map(String::from_iter).collect();
        for x in chunked {
            if x.len() == 3 {
                let acid = protein_map.get(x.as_str());
                if acid.is_some() {
                    let acid = *acid.unwrap_or(&"");
                    if acid.is_empty() {
                        break;
                    } else {
                        protein.push(acid)
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
    }


    Some(protein)
}