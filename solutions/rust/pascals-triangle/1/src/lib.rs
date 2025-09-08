pub struct PascalsTriangle(u32);


impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (1..=self.0).map(|v| {
            let mut row_vec = Vec::with_capacity(v as usize);
            row_vec.push(1);
            (1..v).for_each(|r| {
                row_vec.push(row_vec[r as usize - 1] * (v - r) / r); //二项式递推公式：当前项等于前一项 * (n - (k - 1) / k)，n = v - 1，所以 (v - r) / r。
            });
            row_vec
        }).collect()
    }
}
