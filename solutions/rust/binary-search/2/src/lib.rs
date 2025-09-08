pub fn find(array: &[i32], key: i32) -> Option<usize> {
    Vec::from(array).binary_search(&key).ok()
}
