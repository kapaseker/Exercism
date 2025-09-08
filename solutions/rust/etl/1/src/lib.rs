use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    
    let mut tree = BTreeMap::new();

    for (i,v) in h {
        for x in v {
            tree.insert(x.clone().to_ascii_lowercase(), i.clone());
        }
    }
    
    tree
}
