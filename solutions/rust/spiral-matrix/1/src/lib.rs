pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0u32; size as usize]; size as usize];
    if size == 0 { return matrix; };
    
    let count = (size * size) as usize;
    let (mut l, mut r, mut t, mut b, mut row, mut col) = (0, (size - 1) as usize, 0, (size - 1) as usize, 0, 0);
    let mut d = 0;

    let mut spiral = 1;

    loop {
        matrix[row][col] = spiral;

        match d {
            0 => {
                if col == r {
                    d = 1;
                    t += 1;
                    row += 1;
                } else {
                    col += 1;
                }
            }
            1 => {
                if row == b {
                    d = 2;
                    r -= 1;
                    col -= 1;
                } else {
                    row += 1;
                }
            }
            2 => {
                if col == l {
                    d = 3;
                    b -= 1;
                    row -= 1;
                } else {
                    col -= 1;
                }
            }
            _ => {
                if row == t {
                    d = 0;
                    l += 1;
                    col += 1;
                } else {
                    row -= 1;
                }
            }
        }

        spiral += 1;
        if spiral > count as u32 {
            break;
        }
    }

    matrix
}
