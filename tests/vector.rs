extern crate xmath;

use xmath::{Vector, Matrix};

#[test]
fn transform_vector2() {
    let matrix = Matrix::new(
        2.0, 3.0, 5.0, 7.0,
        11.0, 13.0, 17.0, 19.0,
        23.0, 29.0, 31.0, 37.0,
        41.0, 43.0, 47.0, 53.0,
    );

    let origin = (100.0, 10000.0);

    let transformed = origin.transform(&matrix);

    assert_eq!(transformed.0, 110241.0);
    assert_eq!(transformed.1, 130343.0);
}

#[test]
fn transform_vector3() {
    let matrix = Matrix::new(
        2.0, 3.0, 5.0, 7.0,
        11.0, 13.0, 17.0, 19.0,
        23.0, 29.0, 31.0, 37.0,
        41.0, 43.0, 47.0, 53.0,
    );

    let origin = (100.0, 10000.0, 1000000.0);

    let transformed = origin.transform(&matrix);

    assert_eq!(transformed.0, 23110241.0);
    assert_eq!(transformed.1, 29130343.0);
    assert_eq!(transformed.2, 31170547.0);
}

#[test]
fn transform_vector4() {
    let matrix = Matrix::new(
        2.0, 3.0, 5.0, 7.0,
        11.0, 13.0, 17.0, 19.0,
        23.0, 29.0, 31.0, 37.0,
        41.0, 43.0, 47.0, 53.0,
    );

    let origin = (1.0, 100.0, 10000.0, 1000000.0);

    let transformed = origin.transform(&matrix);

    assert_eq!(transformed.0, 41231102.0);
    assert_eq!(transformed.1, 43291303.0);
    assert_eq!(transformed.2, 47311705.0);
    assert_eq!(transformed.3, 53371907.0);
}

#[test]
fn splat_x_of_vector2() {
    let vector2 = (1.0, 2.0);
    let splatted = vector2.splat_x();

    assert_eq!(splatted.0, vector2.0);
    assert_eq!(splatted.1, vector2.0);
}

#[test]
fn splat_y_of_vector2() {
    let vector2 = (1.0, 2.0);
    let splatted = vector2.splat_y();

    assert_eq!(splatted.0, vector2.1);
    assert_eq!(splatted.1, vector2.1);
}

#[test]
fn splat_z_of_vector2_fills_zero() {
    let vector2 = (1.0, 2.0);
    let splatted = vector2.splat_z();

    assert_eq!(splatted.0, 0.0);
    assert_eq!(splatted.1, 0.0);
}

#[test]
fn splat_w_of_vector2_fills_zero() {
    let vector2 = (1.0, 2.0);
    let splatted = vector2.splat_w();

    assert_eq!(splatted.0, 0.0);
    assert_eq!(splatted.1, 0.0);
}

#[test]
fn splat_x_of_vector3() {
    let vector3 = (1.0, 2.0, 3.0);
    let splatted = vector3.splat_x();

    assert_eq!(splatted.0, vector3.0);
    assert_eq!(splatted.1, vector3.0);
    assert_eq!(splatted.2, vector3.0);
}

#[test]
fn splat_y_of_vector3() {
    let vector3 = (1.0, 2.0, 3.0);
    let splatted = vector3.splat_y();

    assert_eq!(splatted.0, vector3.1);
    assert_eq!(splatted.1, vector3.1);
    assert_eq!(splatted.2, vector3.1);
}

#[test]
fn splat_z_of_vector3() {
    let vector3 = (1.0, 2.0, 3.0);
    let splatted = vector3.splat_z();

    assert_eq!(splatted.0, vector3.2);
    assert_eq!(splatted.1, vector3.2);
    assert_eq!(splatted.2, vector3.2);
}

#[test]
fn splat_w_of_vector3_fills_zero() {
    let vector3 = (1.0, 2.0, 3.0);
    let splatted = vector3.splat_w();

    assert_eq!(splatted.0, 0.0);
    assert_eq!(splatted.1, 0.0);
    assert_eq!(splatted.2, 0.0);
}

#[test]
fn splat_x_of_vector4() {
    let vector4 = (1.0, 2.0, 3.0, 4.0);
    let splatted = vector4.splat_x();

    assert_eq!(splatted.0, vector4.0);
    assert_eq!(splatted.1, vector4.0);
    assert_eq!(splatted.2, vector4.0);
    assert_eq!(splatted.3, vector4.0);
}

#[test]
fn splat_y_of_vector4() {
    let vector4 = (1.0, 2.0, 3.0, 4.0);
    let splatted = vector4.splat_y();

    assert_eq!(splatted.0, vector4.1);
    assert_eq!(splatted.1, vector4.1);
    assert_eq!(splatted.2, vector4.1);
    assert_eq!(splatted.3, vector4.1);
}

#[test]
fn splat_z_of_vector4() {
    let vector4 = (1.0, 2.0, 3.0, 4.0);
    let splatted = vector4.splat_z();

    assert_eq!(splatted.0, vector4.2);
    assert_eq!(splatted.1, vector4.2);
    assert_eq!(splatted.2, vector4.2);
    assert_eq!(splatted.3, vector4.2);
}

#[test]
fn splat_w_of_vector4() {
    let vector4 = (1.0, 2.0, 3.0, 4.0);
    let splatted = vector4.splat_w();

    assert_eq!(splatted.0, vector4.3);
    assert_eq!(splatted.1, vector4.3);
    assert_eq!(splatted.2, vector4.3);
    assert_eq!(splatted.3, vector4.3);
}
