use std::collections::HashSet;

/*
 y
 ^
 |
 |
 |
 |_________> x
0/0
*/

// runtime: O(xy), space: O(x+y)
fn cross_out_zeros(matrix: &mut [&mut [i32]]) {
    let mut zeroed_rows = HashSet::new();
    let mut zeroed_cols = HashSet::new();

    for (y, row) in matrix.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if *val == 0 {
                zeroed_cols.insert(x);
                zeroed_rows.insert(y);
            }
        }
    }

    for (y, row) in matrix.iter_mut().enumerate() {
        for (x, val) in row.iter_mut().enumerate() {
            if zeroed_cols.contains(&x) || zeroed_rows.contains(&y) {
                *val = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn given_0x0_when_crossing_then_remains_unchanged() {
        let matrix: &mut [&mut [i32]] = &mut Vec::new();
        super::cross_out_zeros(matrix);
        assert_eq!(matrix.len(), 0);
    }

    #[test]
    fn given_1x1_is_1_when_crossing_then_remains_unchanged() {
        let col1: &mut [i32] = &mut vec![1];
        let matrix: &mut [&mut [i32]] = &mut vec![ col1 ];

        super::cross_out_zeros(matrix);

        assert_eq!(matrix.len(), 1);
        assert_eq!(matrix[0][0], 1);
    }

    #[test]
    fn given_1x1_is_0_when_crossing_then_remains_unchanged() {
        let col1: &mut [i32] = &mut vec![0];
        let matrix: &mut [&mut [i32]] = &mut vec![ col1 ];

        super::cross_out_zeros(matrix);

        assert_eq!(matrix.len(), 1);
        assert_eq!(matrix[0][0], 0);
    }

    #[test]
    fn given_3x3_all_1s_when_crossing_then_remains_unchanged() {
        let col1: &mut [i32] = &mut vec![1, 1, 1];
        let col2: &mut [i32] = &mut vec![1, 1, 1];
        let col3: &mut [i32] = &mut vec![1, 1, 1];
        let matrix: &mut [&mut [i32]] = &mut vec![ col1, col2, col3 ];

        super::cross_out_zeros(matrix);

        assert_eq!(matrix.len(), 3);

        assert_eq!(matrix[0].len(), 3);
        assert_eq!(matrix[0][0], 1);
        assert_eq!(matrix[0][1], 1);
        assert_eq!(matrix[0][2], 1);
        
        assert_eq!(matrix[1].len(), 3);
        assert_eq!(matrix[1][0], 1);
        assert_eq!(matrix[1][1], 1);
        assert_eq!(matrix[1][2], 1);
        
        assert_eq!(matrix[2].len(), 3);
        assert_eq!(matrix[2][0], 1);
        assert_eq!(matrix[2][1], 1);
        assert_eq!(matrix[2][2], 1);
    }

    #[test]
    fn given_3x3_mixed_when_crossing_then_is_crossed_correctly() {
        let col1: &mut [i32] = &mut vec![0, 1, 1];
        let col2: &mut [i32] = &mut vec![1, 1, 1];
        let col3: &mut [i32] = &mut vec![0, 1, 0];
        let matrix: &mut [&mut [i32]] = &mut vec![ col1, col2, col3 ];

        super::cross_out_zeros(matrix);

        assert_eq!(matrix.len(), 3);

        assert_eq!(matrix[0].len(), 3);
        assert_eq!(matrix[0][0], 0);
        assert_eq!(matrix[0][1], 0);
        assert_eq!(matrix[0][2], 0);
        
        assert_eq!(matrix[1].len(), 3);
        assert_eq!(matrix[1][0], 0);
        assert_eq!(matrix[1][1], 1);
        assert_eq!(matrix[1][2], 0);
        
        assert_eq!(matrix[2].len(), 3);
        assert_eq!(matrix[2][0], 0);
        assert_eq!(matrix[2][1], 0);
        assert_eq!(matrix[2][2], 0);
    }
}