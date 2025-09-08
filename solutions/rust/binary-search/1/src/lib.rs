pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let vec = Vec::from(array);
    let result = vec.binary_search(&key);
    if result.is_ok() {
        Some(result.unwrap())
    } else {
        None
    }
}
