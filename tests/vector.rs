extern crate xmath;

use std::f32;
use xmath::{Vector, Vector2, Vector3, Vector4, Matrix};

#[test]
fn zero_of_vector2() {
    let v = Vector2::zero();

    assert_eq!(v.x, 0.0);
    assert_eq!(v.y, 0.0);
}
#[test]
fn zero_of_vector3() {
    let v = Vector3::zero();

    assert_eq!(v.x, 0.0);
    assert_eq!(v.y, 0.0);
    assert_eq!(v.z, 0.0);
}
#[test]
fn zero_of_vector4() {
    let v = Vector4::zero();

    assert_eq!(v.x, 0.0);
    assert_eq!(v.y, 0.0);
    assert_eq!(v.z, 0.0);
    assert_eq!(v.w, 0.0);
}

#[test]
fn one_of_vector2() {
    let v = Vector2::one();

    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 1.0);
}
#[test]
fn one_of_vector3() {
    let v = Vector3::one();

    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 1.0);
    assert_eq!(v.z, 1.0);
}
#[test]
fn one_of_vector4() {
    let v = Vector4::one();

    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 1.0);
    assert_eq!(v.z, 1.0);
    assert_eq!(v.w, 1.0);
}

#[test]
fn infinity_of_vector2() {
    let v = Vector2::infinity();

    assert!(v.x.is_infinite());
    assert!(v.y.is_infinite());
}
#[test]
fn infinity_of_vector3() {
    let v = Vector3::infinity();

    assert!(v.x.is_infinite());
    assert!(v.y.is_infinite());
    assert!(v.z.is_infinite());
}
#[test]
fn infinity_of_vector4() {
    let v = Vector4::infinity();

    assert!(v.x.is_infinite());
    assert!(v.y.is_infinite());
    assert!(v.z.is_infinite());
    assert!(v.w.is_infinite());
}

#[test]
fn nan_of_vector2() {
    let v = Vector2::nan();

    assert!(v.x.is_nan());
    assert!(v.y.is_nan());
}
#[test]
fn nan_of_vector3() {
    let v = Vector3::nan();

    assert!(v.x.is_nan());
    assert!(v.y.is_nan());
    assert!(v.z.is_nan());
}
#[test]
fn nan_of_vector4() {
    let v = Vector4::nan();

    assert!(v.x.is_nan());
    assert!(v.y.is_nan());
    assert!(v.z.is_nan());
    assert!(v.w.is_nan());
}

#[test]
fn epsilon_of_vector2() {
    let v = Vector2::epsilon();

    assert_eq!(v.x, f32::EPSILON);
    assert_eq!(v.y, f32::EPSILON);
}
#[test]
fn epsilon_of_vector3() {
    let v = Vector3::epsilon();

    assert_eq!(v.x, f32::EPSILON);
    assert_eq!(v.y, f32::EPSILON);
    assert_eq!(v.z, f32::EPSILON);
}
#[test]
fn epsilon_of_vector4() {
    let v = Vector4::epsilon();

    assert_eq!(v.x, f32::EPSILON);
    assert_eq!(v.y, f32::EPSILON);
    assert_eq!(v.z, f32::EPSILON);
    assert_eq!(v.w, f32::EPSILON);
}

#[test]
fn replicate_of_vector2() {
    let value = 3.5;
    let v = Vector2::replicate(value);

    assert_eq!(v.x, value);
    assert_eq!(v.y, value);
}
#[test]
fn replicate_of_vector3() {
    let value = 3.5;
    let v = Vector3::replicate(value);

    assert_eq!(v.x, value);
    assert_eq!(v.y, value);
    assert_eq!(v.z, value);
}
#[test]
fn replicate_of_vector4() {
    let value = 3.5;
    let v = Vector4::replicate(value);

    assert_eq!(v.x, value);
    assert_eq!(v.y, value);
    assert_eq!(v.z, value);
    assert_eq!(v.w, value);
}

#[test]
fn transform_vector2() {
    let matrix = Matrix::new(
        2.0, 3.0, 5.0, 7.0,
        11.0, 13.0, 17.0, 19.0,
        23.0, 29.0, 31.0, 37.0,
        41.0, 43.0, 47.0, 53.0,
    );

    let origin = Vector2 {
        x: 100.0,
        y: 10000.0,
    };

    let transformed = origin.transform(&matrix);

    assert_eq!(transformed.x, 110241.0);
    assert_eq!(transformed.y, 130343.0);
}

#[test]
fn transform_vector3() {
    let matrix = Matrix::new(
        2.0, 3.0, 5.0, 7.0,
        11.0, 13.0, 17.0, 19.0,
        23.0, 29.0, 31.0, 37.0,
        41.0, 43.0, 47.0, 53.0,
    );

    let origin = Vector3 {
        x: 100.0,
        y: 10000.0,
        z: 1000000.0,
    };

    let transformed = origin.transform(&matrix);

    assert_eq!(transformed.x, 23110241.0);
    assert_eq!(transformed.y, 29130343.0);
    assert_eq!(transformed.z, 31170547.0);
}

#[test]
fn transform_vector4() {
    let matrix = Matrix::new(
        2.0, 3.0, 5.0, 7.0,
        11.0, 13.0, 17.0, 19.0,
        23.0, 29.0, 31.0, 37.0,
        41.0, 43.0, 47.0, 53.0,
    );

    let origin = Vector4 {
        x: 1.0,
        y: 100.0,
        z: 10000.0,
        w: 1000000.0,
    };

    let transformed = origin.transform(&matrix);

    assert_eq!(transformed.x, 41231102.0);
    assert_eq!(transformed.y, 43291303.0);
    assert_eq!(transformed.z, 47311705.0);
    assert_eq!(transformed.w, 53371907.0);
}

#[test]
fn min_of_vector2() {
    let v1 = Vector2 {
        x: 1.9,
        y: -41.5,
    };
    let v2 = Vector2 {
        x: 14.9,
        y: 1.3,
    };
    let v3 = v1.min(&v2);

    assert_eq!(v1.x.min(v2.x), v3.x);
    assert_eq!(v1.y.min(v2.y), v3.y);
}
#[test]
fn min_of_vector3() {
    let v1 = Vector3 {
        x: 1.9,
        y: 2.0,
        z: -3.0,
    };
    let v2 = Vector3 {
        x: 1.9,
        y: -45.5,
        z: 43.43,
    };
    let v3 = v1.min(&v2);

    assert_eq!(v1.x.min(v2.x), v3.x);
    assert_eq!(v1.y.min(v2.y), v3.y);
    assert_eq!(v1.z.min(v2.z), v3.z);
}
#[test]
fn min_of_vector4() {
    let v1 = Vector4 {
        x: 1.9,
        y: 2.3,
        z: -3.1,
        w: -16.7,
    };
    let v2 = Vector4 {
        x: 1.9,
        y: -45.5,
        z: 43.43,
        w: -43.43,
    };
    let v3 = v1.min(&v2);

    assert_eq!(v1.x.min(v2.x), v3.x);
    assert_eq!(v1.y.min(v2.y), v3.y);
    assert_eq!(v1.z.min(v2.z), v3.z);
    assert_eq!(v1.w.min(v2.w), v3.w);
}

#[test]
fn max_of_vector2() {
    let v1 = Vector2 {
        x: 1.9,
        y: -41.5,
    };
    let v2 = Vector2 {
        x: 14.9,
        y: 1.3,
    };
    let v3 = v1.max(&v2);

    assert_eq!(v1.x.max(v2.x), v3.x);
    assert_eq!(v1.y.max(v2.y), v3.y);
}
#[test]
fn max_of_vector3() {
    let v1 = Vector3 {
        x: 1.9,
        y: 2.0,
        z: -3.0,
    };
    let v2 = Vector3 {
        x: 1.9,
        y: -45.5,
        z: 43.43,
    };
    let v3 = v1.max(&v2);

    assert_eq!(v1.x.max(v2.x), v3.x);
    assert_eq!(v1.y.max(v2.y), v3.y);
    assert_eq!(v1.z.max(v2.z), v3.z);
}
#[test]
fn max_of_vector4() {
    let v1 = Vector4 {
        x: 1.9,
        y: 2.3,
        z: -3.1,
        w: -16.7,
    };
    let v2 = Vector4 {
        x: 1.9,
        y: -45.5,
        z: 43.43,
        w: -43.43,
    };
    let v3 = v1.max(&v2);

    assert_eq!(v1.x.max(v2.x), v3.x);
    assert_eq!(v1.y.max(v2.y), v3.y);
    assert_eq!(v1.z.max(v2.z), v3.z);
    assert_eq!(v1.w.max(v2.w), v3.w);
}

#[test]
fn round_of_vector2() {
    let v1 = Vector2 {
        x: 1.9,
        y: -41.5,
    };
    let v2 = v1.round();

    assert_eq!(v1.x.round(), v2.x);
    assert_eq!(v1.y.round(), v2.y);
}
#[test]
fn round_of_vector3() {
    let v1 = Vector3 {
        x: 1.9,
        y: 2.0,
        z: -3.0,
    };
    let v2 = v1.round();

    assert_eq!(v1.x.round(), v2.x);
    assert_eq!(v1.y.round(), v2.y);
    assert_eq!(v1.z.round(), v2.z);
}
#[test]
fn round_of_vector4() {
    let v1 = Vector4 {
        x: 1.9,
        y: 2.3,
        z: -3.1,
        w: -16.7,
    };
    let v2 = v1.round();

    assert_eq!(v1.x.round(), v2.x);
    assert_eq!(v1.y.round(), v2.y);
    assert_eq!(v1.z.round(), v2.z);
    assert_eq!(v1.w.round(), v2.w);
}

#[test]
fn trunc_of_vector2() {
    let v1 = Vector2 {
        x: 1.9,
        y: -41.5,
    };
    let v2 = v1.trunc();

    assert_eq!(v1.x.trunc(), v2.x);
    assert_eq!(v1.y.trunc(), v2.y);
}
#[test]
fn trunc_of_vector3() {
    let v1 = Vector3 {
        x: 1.9,
        y: 2.0,
        z: -3.0,
    };
    let v2 = v1.trunc();

    assert_eq!(v1.x.trunc(), v2.x);
    assert_eq!(v1.y.trunc(), v2.y);
    assert_eq!(v1.z.trunc(), v2.z);
}
#[test]
fn trunc_of_vector4() {
    let v1 = Vector4 {
        x: 1.9,
        y: 2.3,
        z: -3.1,
        w: -16.7,
    };
    let v2 = v1.trunc();

    assert_eq!(v1.x.trunc(), v2.x);
    assert_eq!(v1.y.trunc(), v2.y);
    assert_eq!(v1.z.trunc(), v2.z);
    assert_eq!(v1.w.trunc(), v2.w);
}

#[test]
fn floor_of_vector2() {
    let v1 = Vector2 {
        x: 1.9,
        y: -41.5,
    };
    let v2 = v1.floor();

    assert_eq!(v1.x.floor(), v2.x);
    assert_eq!(v1.y.floor(), v2.y);
}
#[test]
fn floor_of_vector3() {
    let v1 = Vector3 {
        x: 1.9,
        y: 2.0,
        z: -3.0,
    };
    let v2 = v1.floor();

    assert_eq!(v1.x.floor(), v2.x);
    assert_eq!(v1.y.floor(), v2.y);
    assert_eq!(v1.z.floor(), v2.z);
}
#[test]
fn floor_of_vector4() {
    let v1 = Vector4 {
        x: 1.9,
        y: 2.3,
        z: -3.1,
        w: -16.7,
    };
    let v2 = v1.floor();

    assert_eq!(v1.x.floor(), v2.x);
    assert_eq!(v1.y.floor(), v2.y);
    assert_eq!(v1.z.floor(), v2.z);
    assert_eq!(v1.w.floor(), v2.w);
}

#[test]
fn ceil_of_vector2() {
    let v1 = Vector2 {
        x: 1.9,
        y: -41.5,
    };
    let v2 = v1.ceil();

    assert_eq!(v1.x.ceil(), v2.x);
    assert_eq!(v1.y.ceil(), v2.y);
}
#[test]
fn ceil_of_vector3() {
    let v1 = Vector3 {
        x: 1.9,
        y: 2.0,
        z: -3.0,
    };
    let v2 = v1.ceil();

    assert_eq!(v1.x.ceil(), v2.x);
    assert_eq!(v1.y.ceil(), v2.y);
    assert_eq!(v1.z.ceil(), v2.z);
}
#[test]
fn ceil_of_vector4() {
    let v1 = Vector4 {
        x: 1.9,
        y: 2.3,
        z: -3.1,
        w: -16.7,
    };
    let v2 = v1.ceil();

    assert_eq!(v1.x.ceil(), v2.x);
    assert_eq!(v1.y.ceil(), v2.y);
    assert_eq!(v1.z.ceil(), v2.z);
    assert_eq!(v1.w.ceil(), v2.w);
}

#[test]
fn clamp_of_vector2() {
    let v1 = Vector2 {
        x: 1.9,
        y: -41.5,
    };
    let v2 = Vector2 {
        x: -41.9,
        y: -4.5,
    };
    let v3 = Vector2 {
        x: 1.9,
        y: 4.5,
    };
    let v4 = v1.clamp(&v2, &v3);

    assert_eq!(v4.x, 1.9);
    assert_eq!(v4.y, -4.5);
}
#[test]
fn clamp_of_vector3() {
    let v1 = Vector3 {
        x: 1.9,
        y: 2.0,
        z: -3.0,
    };
    let v2 = Vector3 {
        x: -41.9,
        y: -4.5,
        z: -34.5,
    };
    let v3 = Vector3 {
        x: 3.7,
        y: 4.5,
        z: 4.5,
    };
    let v4 = v1.clamp(&v2, &v3);

    assert_eq!(v4.x, 1.9);
    assert_eq!(v4.y, 2.0);
    assert_eq!(v4.z, -3.0);
}
#[test]
fn clamp_of_vector4() {
    let v1 = Vector4 {
        x: 1.9,
        y: 2.3,
        z: -3.1,
        w: -16.7,
    };
    let v2 = Vector4 {
        x: -41.9,
        y: -4.5,
        z: -34.5,
        w: 4.5,
    };
    let v3 = Vector4 {
        x: 3.7,
        y: 4.5,
        z: 44.0,
        w: 54.0,
    };
    let v4 = v1.clamp(&v2, &v3);

    assert_eq!(v4.x, 1.9);
    assert_eq!(v4.y, 2.3);
    assert_eq!(v4.z, -3.1);
    assert_eq!(v4.w, 4.5);
}

#[test]
fn splat_x_of_vector2() {
    let vector2 = Vector2 {
        x: 1.0,
        y: 2.0,
    };
    let splatted = vector2.splat_x();

    assert_eq!(splatted.x, vector2.x);
    assert_eq!(splatted.y, vector2.x);
}

#[test]
fn splat_y_of_vector2() {
    let vector2 = Vector2 {
        x: 1.0,
        y: 2.0,
    };
    let splatted = vector2.splat_y();

    assert_eq!(splatted.x, vector2.y);
    assert_eq!(splatted.y, vector2.y);
}

#[test]
fn splat_z_of_vector2_fills_zero() {
    let vector2 = Vector2 {
        x: 1.0,
        y: 2.0,
    };
    let splatted = vector2.splat_z();

    assert_eq!(splatted.x, 0.0);
    assert_eq!(splatted.y, 0.0);
}

#[test]
fn splat_w_of_vector2_fills_zero() {
    let vector2 = Vector2 {
        x: 1.0,
        y: 2.0,
    };
    let splatted = vector2.splat_w();

    assert_eq!(splatted.x, 0.0);
    assert_eq!(splatted.y, 0.0);
}

#[test]
fn splat_x_of_vector3() {
    let vector3 = Vector3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let splatted = vector3.splat_x();

    assert_eq!(splatted.x, vector3.x);
    assert_eq!(splatted.y, vector3.x);
    assert_eq!(splatted.z, vector3.x);
}

#[test]
fn splat_y_of_vector3() {
    let vector3 = Vector3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let splatted = vector3.splat_y();

    assert_eq!(splatted.x, vector3.y);
    assert_eq!(splatted.y, vector3.y);
    assert_eq!(splatted.z, vector3.y);
}

#[test]
fn splat_z_of_vector3() {
    let vector3 = Vector3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let splatted = vector3.splat_z();

    assert_eq!(splatted.x, vector3.z);
    assert_eq!(splatted.y, vector3.z);
    assert_eq!(splatted.z, vector3.z);
}

#[test]
fn splat_w_of_vector3_fills_zero() {
    let vector3 = Vector3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let splatted = vector3.splat_w();

    assert_eq!(splatted.x, 0.0);
    assert_eq!(splatted.y, 0.0);
    assert_eq!(splatted.z, 0.0);
}

#[test]
fn splat_x_of_vector4() {
    let vector4 = Vector4 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
        w: 4.0,
    };
    let splatted = vector4.splat_x();

    assert_eq!(splatted.x, vector4.x);
    assert_eq!(splatted.y, vector4.x);
    assert_eq!(splatted.z, vector4.x);
    assert_eq!(splatted.w, vector4.x);
}

#[test]
fn splat_y_of_vector4() {
    let vector4 = Vector4 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
        w: 4.0,
    };
    let splatted = vector4.splat_y();

    assert_eq!(splatted.x, vector4.y);
    assert_eq!(splatted.y, vector4.y);
    assert_eq!(splatted.z, vector4.y);
    assert_eq!(splatted.w, vector4.y);
}

#[test]
fn splat_z_of_vector4() {
    let vector4 = Vector4 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
        w: 4.0,
    };
    let splatted = vector4.splat_z();

    assert_eq!(splatted.x, vector4.z);
    assert_eq!(splatted.y, vector4.z);
    assert_eq!(splatted.z, vector4.z);
    assert_eq!(splatted.w, vector4.z);
}

#[test]
fn splat_w_of_vector4() {
    let vector4 = Vector4 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
        w: 4.0,
    };
    let splatted = vector4.splat_w();

    assert_eq!(splatted.x, vector4.w);
    assert_eq!(splatted.y, vector4.w);
    assert_eq!(splatted.z, vector4.w);
    assert_eq!(splatted.w, vector4.w);
}


#[test]
fn neg_for_vector2() {
    let v1 = Vector2 {
        x: 10.0,
        y: 11.0,
    };

    let v = -v1;

    assert_eq!(v.x, -v1.x);
    assert_eq!(v.y, -v1.y);
}
#[test]
fn neg_for_vector3() {
    let v1 = Vector3 {
        x: 10.0,
        y: 11.0,
        z: 12.0,
    };

    let v = -v1;

    assert_eq!(v.x, -v1.x);
    assert_eq!(v.y, -v1.y);
    assert_eq!(v.z, -v1.z);
}
#[test]
fn neg_for_vector4() {
    let v1 = Vector4 {
        x: 10.0,
        y: 11.0,
        z: 12.0,
        w: 13.0,
    };

    let v = -v1;

    assert_eq!(v.x, -v1.x);
    assert_eq!(v.y, -v1.y);
    assert_eq!(v.z, -v1.z);
    assert_eq!(v.w, -v1.w);
}

#[test]
fn add_for_vector2() {
    let v1 = Vector2 {
        x: 10.0,
        y: 11.0,
    };
    let v2 = Vector2 {
        x: 100.0,
        y: 200.0,
    };

    let v = v1 + v2;

    assert_eq!(v.x, v1.x + v2.x);
    assert_eq!(v.y, v1.y + v2.y);
}
#[test]
fn add_for_vector3() {
    let v1 = Vector3 {
        x: 10.0,
        y: 11.0,
        z: 12.0,
    };
    let v2 = Vector3 {
        x: 100.0,
        y: 200.0,
        z: 300.0,
    };

    let v = v1 + v2;

    assert_eq!(v.x, v1.x + v2.x);
    assert_eq!(v.y, v1.y + v2.y);
    assert_eq!(v.z, v1.z + v2.z);
}
#[test]
fn add_for_vector4() {
    let v1 = Vector4 {
        x: 10.0,
        y: 11.0,
        z: 12.0,
        w: 13.0,
    };
    let v2 = Vector4 {
        x: 100.0,
        y: 200.0,
        z: 300.0,
        w: 400.0,
    };

    let v = v1 + v2;

    assert_eq!(v.x, v1.x + v2.x);
    assert_eq!(v.y, v1.y + v2.y);
    assert_eq!(v.z, v1.z + v2.z);
    assert_eq!(v.w, v1.w + v2.w);
}

#[test]
fn sub_for_vector2() {
    let v1 = Vector2 {
        x: 10.0,
        y: 11.0,
    };
    let v2 = Vector2 {
        x: 100.0,
        y: 200.0,
    };

    let v = v1 - v2;

    assert_eq!(v.x, v1.x - v2.x);
    assert_eq!(v.y, v1.y - v2.y);
}
#[test]
fn sub_for_vector3() {
    let v1 = Vector3 {
        x: 10.0,
        y: 11.0,
        z: 12.0,
    };
    let v2 = Vector3 {
        x: 100.0,
        y: 200.0,
        z: 300.0,
    };

    let v = v1 - v2;

    assert_eq!(v.x, v1.x - v2.x);
    assert_eq!(v.y, v1.y - v2.y);
    assert_eq!(v.z, v1.z - v2.z);
}
#[test]
fn sub_for_vector4() {
    let v1 = Vector4 {
        x: 10.0,
        y: 11.0,
        z: 12.0,
        w: 13.0,
    };
    let v2 = Vector4 {
        x: 100.0,
        y: 200.0,
        z: 300.0,
        w: 400.0,
    };

    let v = v1 - v2;

    assert_eq!(v.x, v1.x - v2.x);
    assert_eq!(v.y, v1.y - v2.y);
    assert_eq!(v.z, v1.z - v2.z);
    assert_eq!(v.w, v1.w - v2.w);
}

#[test]
fn div_for_vector2() {
    let v1 = Vector2 {
        x: 10.0,
        y: 11.0,
    };
    let v2 = Vector2 {
        x: 100.0,
        y: 200.0,
    };

    let v = v1 / v2;

    assert_eq!(v.x, v1.x / v2.x);
    assert_eq!(v.y, v1.y / v2.y);
}
#[test]
fn div_for_vector3() {
    let v1 = Vector3 {
        x: 10.0,
        y: 11.0,
        z: 12.0,
    };
    let v2 = Vector3 {
        x: 100.0,
        y: 200.0,
        z: 300.0,
    };

    let v = v1 / v2;

    assert_eq!(v.x, v1.x / v2.x);
    assert_eq!(v.y, v1.y / v2.y);
    assert_eq!(v.z, v1.z / v2.z);
}
#[test]
fn div_for_vector4() {
    let v1 = Vector4 {
        x: 10.0,
        y: 11.0,
        z: 12.0,
        w: 13.0,
    };
    let v2 = Vector4 {
        x: 100.0,
        y: 200.0,
        z: 300.0,
        w: 400.0,
    };

    let v = v1 / v2;

    assert_eq!(v.x, v1.x / v2.x);
    assert_eq!(v.y, v1.y / v2.y);
    assert_eq!(v.z, v1.z / v2.z);
    assert_eq!(v.w, v1.w / v2.w);
}

#[test]
fn scale_for_vector2() {
    let v1 = Vector2 {
        x: 10.0,
        y: 11.0,
    };

    let scale_factor = 5.3;

    let v = v1 * scale_factor;

    assert_eq!(v.x, v1.x * scale_factor);
    assert_eq!(v.y, v1.y * scale_factor);
}
#[test]
fn scale_for_vector3() {
    let v1 = Vector3 {
        x: 10.0,
        y: 11.0,
        z: 12.0,
    };

    let scale_factor = 5.3;

    let v = v1 * scale_factor;

    assert_eq!(v.x, v1.x * scale_factor);
    assert_eq!(v.y, v1.y * scale_factor);
    assert_eq!(v.z, v1.z * scale_factor);
}
#[test]
fn scale_for_vector4() {
    let v1 = Vector4 {
        x: 10.0,
        y: 11.0,
        z: 12.0,
        w: 13.0,
    };

    let scale_factor = 5.3;

    let v = v1 * scale_factor;

    assert_eq!(v.x, v1.x * scale_factor);
    assert_eq!(v.y, v1.y * scale_factor);
    assert_eq!(v.z, v1.z * scale_factor);
    assert_eq!(v.w, v1.w * scale_factor);
}

#[test]
fn mul_for_vector2() {
    let v1 = Vector2 {
        x: 10.0,
        y: 11.0,
    };
    let v2 = Vector2 {
        x: 100.0,
        y: 200.0,
    };

    let v = v1 * v2;

    assert_eq!(v.x, v1.x * v2.x);
    assert_eq!(v.y, v1.y * v2.y);
}
#[test]
fn mul_for_vector3() {
    let v1 = Vector3 {
        x: 10.0,
        y: 11.0,
        z: 12.0,
    };
    let v2 = Vector3 {
        x: 100.0,
        y: 200.0,
        z: 300.0,
    };

    let v = v1 * v2;

    assert_eq!(v.x, v1.x * v2.x);
    assert_eq!(v.y, v1.y * v2.y);
    assert_eq!(v.z, v1.z * v2.z);
}
#[test]
fn mul_for_vector4() {
    let v1 = Vector4 {
        x: 10.0,
        y: 11.0,
        z: 12.0,
        w: 13.0,
    };
    let v2 = Vector4 {
        x: 100.0,
        y: 200.0,
        z: 300.0,
        w: 400.0,
    };

    let v = v1 * v2;

    assert_eq!(v.x, v1.x * v2.x);
    assert_eq!(v.y, v1.y * v2.y);
    assert_eq!(v.z, v1.z * v2.z);
    assert_eq!(v.w, v1.w * v2.w);
}

#[test]
fn multiply_add_for_vector2() {
    let v1 = Vector2 {
        x: 1.0,
        y: 2.0,
    };
    let v2 = Vector2 {
        x: 10.0,
        y: 11.0,
    };
    let v3 = Vector2 {
        x: 100.0,
        y: 200.0,
    };

    let v = v1.multiply_add(&v2, &v3);

    assert_eq!(v.x, v1.x * v2.x + v3.x);
    assert_eq!(v.y, v1.y * v2.y + v3.y);
}

#[test]
fn multiply_add_for_vector3() {
    let v1 = Vector3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let v2 = Vector3 {
        x: 10.0,
        y: 11.0,
        z: 12.0,
    };
    let v3 = Vector3 {
        x: 100.0,
        y: 200.0,
        z: 300.0,
    };

    let v = v1.multiply_add(&v2, &v3);

    assert_eq!(v.x, v1.x * v2.x + v3.x);
    assert_eq!(v.y, v1.y * v2.y + v3.y);
    assert_eq!(v.z, v1.z * v2.z + v3.z);
}

#[test]
fn multiply_add_for_vector4() {
    let v1 = Vector4 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
        w: 4.0,
    };
    let v2 = Vector4 {
        x: 10.0,
        y: 11.0,
        z: 12.0,
        w: 13.0,
    };
    let v3 = Vector4 {
        x: 100.0,
        y: 200.0,
        z: 300.0,
        w: 400.0,
    };

    let v = v1.multiply_add(&v2, &v3);

    assert_eq!(v.x, v1.x * v2.x + v3.x);
    assert_eq!(v.y, v1.y * v2.y + v3.y);
    assert_eq!(v.z, v1.z * v2.z + v3.z);
    assert_eq!(v.w, v1.w * v2.w + v3.w);
}

#[test]
fn index_0_for_vector2_is_x() {
    let v = Vector2 {
        x: 1.32,
        y: 23.3,
    };
    assert_eq!(v.x, v[0]);
}

#[test]
fn index_1_for_vector2_is_y() {
    let v = Vector2 {
        x: 1.32,
        y: 23.3,
    };
    assert_eq!(v.y, v[1]);
}

#[test]
fn index_2_for_vector2_is_0() {
    let v = Vector2 {
        x: 1.32,
        y: 23.3,
    };
    assert_eq!(0.0, v[2]);
}

#[test]
fn index_3_for_vector2_is_0() {
    let v = Vector2 {
        x: 1.32,
        y: 23.3,
    };
    assert_eq!(0.0, v[3]);
}

#[test]
#[should_panic]
fn index_4_for_vector2_should_fail() {
    let v = Vector2 {
        x: 1.32,
        y: 23.3,
    };
    let _ = v[4];
}

#[test]
fn index_0_for_vector3_is_x() {
    let v = Vector3 {
        x: 1.32,
        y: 23.3,
        z: -45.4,
    };
    assert_eq!(v.x, v[0]);
}

#[test]
fn index_1_for_vector3_is_y() {
    let v = Vector3 {
        x: 1.32,
        y: 23.3,
        z: -45.4,
    };
    assert_eq!(v.y, v[1]);
}

#[test]
fn index_2_for_vector3_is_z() {
    let v = Vector3 {
        x: 1.32,
        y: 23.3,
        z: -45.4,
    };
    assert_eq!(v.z, v[2]);
}

#[test]
fn index_3_for_vector3_is_0() {
    let v = Vector3 {
        x: 1.32,
        y: 23.3,
        z: -45.4,
    };
    assert_eq!(0.0, v[3]);
}

#[test]
#[should_panic]
fn index_4_for_vector3_should_fail() {
    let v = Vector3 {
        x: 1.32,
        y: 23.3,
        z: -45.4,
    };
    let _ = v[4];
}

#[test]
fn index_0_for_vector4_is_x() {
    let v = Vector4 {
        x: 1.32,
        y: 23.3,
        z: -45.4,
        w: -53.4,
    };
    assert_eq!(v.x, v[0]);
}

#[test]
fn index_1_for_vector4_is_y() {
    let v = Vector4 {
        x: 1.32,
        y: 23.3,
        z: -45.4,
        w: -53.4,
    };
    assert_eq!(v.y, v[1]);
}

#[test]
fn index_2_for_vector4_is_z() {
    let v = Vector4 {
        x: 1.32,
        y: 23.3,
        z: -45.4,
        w: -53.4,
    };
    assert_eq!(v.z, v[2]);
}

#[test]
fn index_3_for_vector4_is_w() {
    let v = Vector4 {
        x: 1.32,
        y: 23.3,
        z: -45.4,
        w: -53.4,
    };
    assert_eq!(v.w, v[3]);
}

#[test]
#[should_panic]
fn index_4_for_vector4_should_fail() {
    let v = Vector4 {
        x: 1.32,
        y: 23.3,
        z: -45.4,
        w: -53.4,
    };
    let _ = v[4];
}

#[test]
fn swizzle_of_vector4() {
    let v1 = Vector4 {
        x: 1.32,
        y: 23.3,
        z: -45.4,
        w: -53.4,
    };
    let v2 = v1.swizzle(1, 3, 0, 2);

    assert_eq!(v2[0], v1[1]);
    assert_eq!(v2[1], v1[3]);
    assert_eq!(v2[2], v1[0]);
    assert_eq!(v2[3], v1[2]);
}

#[test]
#[should_panic]
fn swizzle_of_vector4_should_fail_when_e0_is_more_than_3() {
    let v1 = Vector4 {
        x: 1.32,
        y: 23.3,
        z: -45.4,
        w: -53.4,
    };
    let _ = v1.swizzle(4, 3, 0, 2);
}

#[test]
#[should_panic]
fn swizzle_of_vector4_should_fail_when_e1_is_more_than_3() {
    let v1 = Vector4 {
        x: 1.32,
        y: 23.3,
        z: -45.4,
        w: -53.4,
    };
    let _ = v1.swizzle(1, 4, 0, 2);
}

#[test]
#[should_panic]
fn swizzle_of_vector4_should_fail_when_e2_is_more_than_3() {
    let v1 = Vector4 {
        x: 1.32,
        y: 23.3,
        z: -45.4,
        w: -53.4,
    };
    let _ = v1.swizzle(1, 3, 4, 2);
}

#[test]
#[should_panic]
fn swizzle_of_vector4_should_fail_when_e3_is_more_than_3() {
    let v1 = Vector4 {
        x: 1.32,
        y: 23.3,
        z: -45.4,
        w: -53.4,
    };
    let _ = v1.swizzle(1, 3, 0, 4);
}
