pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut windows = vec![];
    // println!("{}", str.len());
    if len <= digits.len() {
        let times = digits.len() - len;
        for i in 0..=times {
            windows.push((&digits[i..(i+len)]).to_string());
        }
    }
    
    windows
}
