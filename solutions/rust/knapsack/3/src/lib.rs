#[derive(Debug)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}
pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    if items.is_empty() { return 0; }

    let mut dp = vec![0; (max_weight + 1) as usize];

    for i in items[0].weight..=max_weight {
        dp[i as usize] = items[0].value;
    }

    for i in 1..items.len() {
        for j in (items[i].weight..=max_weight).rev() {
            dp[j as usize] = dp[j as usize].max(dp[(j - items[i].weight) as usize] + items[i].value)
        }
    }

    dp[max_weight as usize]
}
