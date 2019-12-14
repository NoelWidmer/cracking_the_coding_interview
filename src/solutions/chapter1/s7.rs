fn rotate_90_deg_clockwise(matrix: &mut [&mut [i32]]) {
    if matrix.len() > 1 {
        recursion_helper(matrix, 0)
    }
}

fn recursion_helper(matrix: &mut [&mut [i32]], offset: usize) {
    let n = matrix.len() - (offset*2);

    if n > 1 {
        for i in 0..n-1 {
            let (x1, y1) = (offset, offset+i);
            let (x2, y2) = get_rotation_target(x1, y1, offset, n);
            let (x3, y3) = get_rotation_target(x2, y2, offset, n);
            let (x4, y4) = get_rotation_target(x3, y3, offset, n);

            let value1 = matrix[x1][y1];
            let value2 = matrix[x2][y2];
            let value3 = matrix[x3][y3];
            let value4 = matrix[x4][y4];

            matrix[x1][y1] = value4;
            matrix[x2][y2] = value1;
            matrix[x3][y3] = value2;
            matrix[x4][y4] = value3;
        }

        recursion_helper(matrix, offset+1);
    }
}

fn get_rotation_target(x: usize, y: usize, offset: usize, n: usize) -> (usize, usize) {
    let x2 = y;
    let y2 = (n-1) - (x-offset);
    (x2, y2 + offset)
}

#[cfg(test)]
mod tests {
    #[test]
    fn given_0x0_when_rotating_then_remains_unchanged() {
        let mut col1 = vec![ ];
        let mut matrix = vec![ &mut col1[..] ];

        super::rotate_90_deg_clockwise(&mut matrix);

        assert_eq!(matrix[0], &vec![ ][..]);
    }

    #[test]
    fn given_1x1_when_rotating_then_remains_unchanged() {
        let mut col1 = vec![ 1 ];
        let mut matrix = vec![ &mut col1[..] ];

        super::rotate_90_deg_clockwise(&mut matrix);

        assert_eq!(matrix[0], &vec![ 1 ][..]);
    }

    #[test]
    fn given_2x2_when_rotating_then_is_rotated_correctly() {
        let mut col1 = vec![ 1, 4 ];
        let mut col2 = vec![ 2, 5 ];
        let mut matrix = vec![ &mut col1[..], &mut col2 ];

        super::rotate_90_deg_clockwise(&mut matrix);

        assert_eq!(matrix[0], &vec![ 2, 1 ][..]);
        assert_eq!(matrix[1], &vec![ 5, 4 ][..]);
    }

    #[test]
    fn given_3x3_when_rotating_then_is_rotated_correctly() {
        let mut col1 = vec![ 1, 4, 7 ];
        let mut col2 = vec![ 2, 5, 8 ];
        let mut col3 = vec![ 3, 6, 9 ];
        let mut matrix = vec![ &mut col1[..], &mut col2, &mut col3 ];

        super::rotate_90_deg_clockwise(&mut matrix);

        assert_eq!(matrix[0], &vec![ 3, 2, 1 ][..]);
        assert_eq!(matrix[1], &vec![ 6, 5, 4 ][..]);
        assert_eq!(matrix[2], &vec![ 9, 8, 7 ][..]);
    }
}