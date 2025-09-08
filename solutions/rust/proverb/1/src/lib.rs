pub fn build_proverb(list: &[&str]) -> String {
    
    let mut res = String::new();
    
    if list.len() == 0 {
        return res;
    }
    
    let head = list[0];
    let last = list.len() - 1;
    
    for i in 1 ..= last {
        res.push_str(&build(list[i-1],list[i]));
    }
        
    let end = format!("And all for the want of a {}.", head);
    
    res.push_str(&end);
    res
}

fn build(a:&str, b:&str) -> String {
    return format!("For want of a {} the {} was lost.\n",a,b);
}


