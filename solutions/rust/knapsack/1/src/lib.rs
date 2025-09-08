#[derive(Debug)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {

    if items.is_empty() { return 0; }

    let mut dp: Vec<Vec<u32>> = vec![vec![0; (max_weight + 1) as usize]; items.len()];

    for i in 0..=max_weight {
        if i >= items[0].weight {
            dp[0][i as usize] = items[0].value
        }
    }

    for i in 1..items.len() {
        for j in 0..=max_weight {
            if j < items[i].weight {
                dp[i][j as usize] = dp[i - 1][j as usize]
            } else {
                dp[i][j as usize] = dp[i - 1][j as usize].max(dp[i - 1][(j - items[i].weight) as usize] + items[i].value)
            }
        }
    }

    dp[items.len() - 1][max_weight as usize]
}
