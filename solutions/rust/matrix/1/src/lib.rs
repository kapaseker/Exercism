pub struct Matrix {
    rows: usize,
    cols: usize,
    nums: Vec<u32>,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let mut rows: usize = 0;
        let mut vec = vec![];

        for r in input.split('\n') {
            for c in r.split(' ') {
                vec.push(c.parse::<u32>().unwrap());
            }
            rows += 1;
        }

        Matrix {
            rows,
            cols: vec.len() / rows,
            nums: vec,
        }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        if row_no <= self.rows {
            let start = (row_no - 1) * self.cols;
            let end = start + self.cols;
            Some(self.nums[start..end].to_vec())
        } else {
            None
        }
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        if col_no <= self.cols {
            let start = (col_no - 1);
            let end = self.nums.len();
            Some(self.nums[start..end].to_vec().into_iter().step_by(self.cols).collect::<Vec<u32>>())
        } else {
            None
        }
    }
}

