extern crate xmath;

use xmath::Matrix;

#[test]
fn create_zero_filled_matrix() {
    let m = Matrix::zero();

    assert_eq!(m[0][0], 0.0);
    assert_eq!(m[0][1], 0.0);
    assert_eq!(m[0][2], 0.0);
    assert_eq!(m[0][3], 0.0);
    assert_eq!(m[1][0], 0.0);
    assert_eq!(m[1][1], 0.0);
    assert_eq!(m[1][2], 0.0);
    assert_eq!(m[1][3], 0.0);
    assert_eq!(m[2][0], 0.0);
    assert_eq!(m[2][1], 0.0);
    assert_eq!(m[2][2], 0.0);
    assert_eq!(m[2][3], 0.0);
    assert_eq!(m[3][0], 0.0);
    assert_eq!(m[3][1], 0.0);
    assert_eq!(m[3][2], 0.0);
    assert_eq!(m[3][3], 0.0);
}

#[test]
fn new_matrix() {
    #[rustfmt::skip]
    let m = Matrix::new(
        1.0, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0,
    );

    assert_eq!(m[0][0], 1.0);
    assert_eq!(m[0][1], 2.0);
    assert_eq!(m[0][2], 3.0);
    assert_eq!(m[0][3], 4.0);
    assert_eq!(m[1][0], 5.0);
    assert_eq!(m[1][1], 6.0);
    assert_eq!(m[1][2], 7.0);
    assert_eq!(m[1][3], 8.0);
    assert_eq!(m[2][0], 9.0);
    assert_eq!(m[2][1], 10.0);
    assert_eq!(m[2][2], 11.0);
    assert_eq!(m[2][3], 12.0);
    assert_eq!(m[3][0], 13.0);
    assert_eq!(m[3][1], 14.0);
    assert_eq!(m[3][2], 15.0);
    assert_eq!(m[3][3], 16.0);
}

#[test]
fn indexing() {
    #[rustfmt::skip]
    let matrix = Matrix::new(
        2.0, 3.0, 5.0, 7.0,
        11.0, 13.0, 17.0, 19.0,
        23.0, 29.0, 31.0, 37.0,
        41.0, 43.0, 47.0, 53.0,
    );

    let row0 = matrix[0];
    assert_eq!(row0, [2.0, 3.0, 5.0, 7.0]);

    let row1 = matrix[1];
    assert_eq!(row1, [11.0, 13.0, 17.0, 19.0]);

    let row2 = matrix[2];
    assert_eq!(row2, [23.0, 29.0, 31.0, 37.0]);

    let row3 = matrix[3];
    assert_eq!(row3, [41.0, 43.0, 47.0, 53.0]);
}
