extern crate glium;

use std::mem::zeroed;
use std::ops::*;
use glium::uniforms::*;

pub trait Vector {
    fn transform(self, matrix: &Matrix) -> Self;

    fn splat_x(self) -> Self;
    fn splat_y(self) -> Self;
    fn splat_z(self) -> Self;
    fn splat_w(self) -> Self;
}

type Vector2 = (f32, f32);
type Vector3 = (f32, f32, f32);
type Vector4 = (f32, f32, f32, f32);

impl Vector for Vector2 {
    fn transform(self, matrix: &Matrix) -> Self {
        let m = matrix.m;

        let x = self.0*m[0][0] + self.1*m[1][0];
        let y = self.0*m[0][1] + self.1*m[1][1];
        (x, y)
    }

    fn splat_x(self) -> Self {
        let x = self.0;
        (x, x)
    }
    fn splat_y(self) -> Self {
        let y = self.1;
        (y, y)
    }
    fn splat_z(self) -> Self {
        let z = 0.0;
        (z, z)
    }
    fn splat_w(self) -> Self {
        let w = 0.0;
        (w, w)
    }
}

impl Vector for Vector3 {
    fn transform(self, matrix: &Matrix) -> Self {
        let m = matrix.m;

        let x = self.0*m[0][0] + self.1*m[1][0] + self.2*m[2][0];
        let y = self.0*m[0][1] + self.1*m[1][1] + self.2*m[2][1];
        let z = self.0*m[0][2] + self.1*m[1][2] + self.2*m[2][2];
        (x, y, z)
    }

    fn splat_x(self) -> Self {
        let x = self.0;
        (x, x, x)
    }
    fn splat_y(self) -> Self {
        let y = self.1;
        (y, y, y)
    }
    fn splat_z(self) -> Self {
        let z = self.2;
        (z, z, z)
    }
    fn splat_w(self) -> Self {
        let w = 0.0;
        (w, w, w)
    }
}

impl Vector for Vector4 {
    fn transform(self, matrix: &Matrix) -> Self {
        let m = matrix.m;

        let x = self.0*m[0][0] + self.1*m[1][0] + self.2*m[2][0] + self.3*m[3][0];
        let y = self.0*m[0][1] + self.1*m[1][1] + self.2*m[2][1] + self.3*m[3][1];
        let z = self.0*m[0][2] + self.1*m[1][2] + self.2*m[2][2] + self.3*m[3][2];
        let w = self.0*m[0][3] + self.1*m[1][3] + self.2*m[2][3] + self.3*m[3][3];
        (x, y, z, w)
    }

    fn splat_x(self) -> Self {
        let x = self.0;
        (x, x, x, x)
    }
    fn splat_y(self) -> Self {
        let y = self.1;
        (y, y, y, y)
    }
    fn splat_z(self) -> Self {
        let z = self.2;
        (z, z, z, z)
    }
    fn splat_w(self) -> Self {
        let w = self.3;
        (w, w, w, w)
    }
}

#[test]
fn transform_vector2() {
    let matrix = Matrix {
        m: [
            [ 2.0, 3.0, 5.0, 7.0 ],
            [ 11.0, 13.0, 17.0, 19.0 ],
            [ 23.0, 29.0, 31.0, 37.0 ],
            [ 41.0, 43.0, 47.0, 53.0 ],
        ]
    };

    let origin = (1.0, 1.0);

    let transformed = origin.transform(&matrix);

    assert_eq!(transformed.0, 13.0);
    assert_eq!(transformed.1, 16.0);
}

#[test]
fn transform_vector3() {
    let matrix = Matrix {
        m: [
            [ 2.0, 3.0, 5.0, 7.0 ],
            [ 11.0, 13.0, 17.0, 19.0 ],
            [ 23.0, 29.0, 31.0, 37.0 ],
            [ 41.0, 43.0, 47.0, 53.0 ],
        ]
    };

    let origin = (1.0, 1.0, 1.0);

    let transformed = origin.transform(&matrix);

    assert_eq!(transformed.0, 36.0);
    assert_eq!(transformed.1, 45.0);
    assert_eq!(transformed.2, 53.0);
}

#[test]
fn transform_vector4() {
    let matrix = Matrix {
        m: [
            [ 2.0, 3.0, 5.0, 7.0 ],
            [ 11.0, 13.0, 17.0, 19.0 ],
            [ 23.0, 29.0, 31.0, 37.0 ],
            [ 41.0, 43.0, 47.0, 53.0 ],
        ]
    };

    let origin = (1.0, 1.0, 1.0, 1.0);

    let transformed = origin.transform(&matrix);

    assert_eq!(transformed.0, 77.0);
    assert_eq!(transformed.1, 88.0);
    assert_eq!(transformed.2, 100.0);
    assert_eq!(transformed.3, 116.0);
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

#[derive(PartialEq, Clone, Debug)]
pub struct Matrix { m: [[f32; 4]; 4] }

impl Matrix {
    pub fn new() -> Self { unsafe { zeroed() } }

    pub fn identity() -> Self {
        Matrix {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        }
    }

    pub fn rotation_x(rad: f32) -> Self {
        let (sin, cos) = rad.sin_cos();

        Matrix {
            m: [
                [1.0,  0.0, 0.0, 0.0],
                [0.0,  cos, sin, 0.0],
                [0.0, -sin, cos, 0.0],
                [0.0,  0.0, 0.0, 1.0],
            ]
        }
    }

    pub fn rotation_y(rad: f32) -> Self {
        let (sin, cos) = rad.sin_cos();

        Matrix {
            m: [
                [cos, 0.0, -sin, 0.0],
                [0.0, 1.0,  0.0, 0.0],
                [sin, 0.0,  cos, 0.0],
                [0.0, 0.0,  0.0, 1.0],
            ]
        }
    }

    pub fn rotation_z(rad: f32) -> Self {
        let (sin, cos) = rad.sin_cos();

        Matrix {
            m: [
                [ cos, sin, 0.0, 0.0],
                [-sin, cos, 0.0, 0.0],
                [ 0.0, 0.0, 1.0, 0.0],
                [ 0.0, 0.0, 0.0, 1.0],
            ]
        }
    }

    pub fn orthographic(view_width: f32, view_height: f32, near_z: f32, far_z: f32) -> Self {
        // assert(!XMScalarNearEqual(ViewWidth, 0.0f, 0.00001f));
        // assert(!XMScalarNearEqual(ViewHeight, 0.0f, 0.00001f));
        // assert(!XMScalarNearEqual(FarZ, NearZ, 0.00001f));
        let f_range = 1.0/(near_z - far_z);
        Matrix {
            m: [
                [2.0/view_width, 0.0, 0.0, 0.0],
                [0.0, 2.0/view_height, 0.0, 0.0],
                [0.0, 0.0, f_range, 0.0],
                [0.0, 0.0, f_range*near_z, 1.0],
            ]
        }

    }

    pub fn orthographic_off_center(view_left: f32, view_right: f32, view_bottom: f32, view_top: f32, near_z: f32, far_z: f32) -> Self {
        // reciprocal width and height
        let r_width = 1.0/(view_right - view_left);
        let r_height = 1.0/(view_top - view_bottom);
        let range = 1.0/(near_z-far_z);

        Matrix {
            m: [
                [r_width + r_width, 0.0, 0.0, 0.0],
                [0.0, r_height + r_height, 0.0, 0.0],
                [0.0, 0.0, range, 0.0],
                [-(view_left + view_right)*r_width, -(view_top + view_bottom)*r_height, range*near_z, 1.0],
            ]
        }
    }

    pub fn perspective(width: f32, height: f32, near_z: f32, far_z: f32) -> Self {
        let two_near_z = near_z + near_z;
        let range = far_z/(near_z - far_z);

        Matrix {
            m: [
                [two_near_z/width, 0.0, 0.0, 0.0],
                [0.0, two_near_z/height, 0.0, 0.0],
                [0.0, 0.0, range, -1.0],
                [0.0, 0.0, range*near_z, 0.0],
            ]
        }
    }

    /// aspect: Width / Height
    pub fn perspective_fov(fov: f32, aspect: f32, near_z: f32, far_z: f32) -> Self {
        let (sin, cos) = (0.5 * fov).sin_cos();
        let f = cos/sin;
        let range = far_z/(near_z - far_z);

        Matrix {
            m: [
                [f/aspect, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, range, -1.0],
                [0.0, 0.0, range*near_z, 0.0],
            ]
        }
    }

    pub fn translation(ox: f32, oy: f32, oz: f32) -> Self {
        Matrix {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ox,  oy,  oz,  1.0],
            ]
        }
    }

    pub fn transpose(self) -> Self {
        Matrix {
            m: [
                [self.m[0][0], self.m[1][0], self.m[2][0], self.m[3][0]],
                [self.m[0][1], self.m[1][1], self.m[2][1], self.m[3][1]],
                [self.m[0][2], self.m[1][2], self.m[2][2], self.m[3][2]],
                [self.m[0][3], self.m[1][3], self.m[2][3], self.m[3][3]],
            ]
        }
    }
}

#[test]
fn new_matrix_is_zeroed() {
    let m = Matrix::new();

    assert_eq!(m.m[0][0], 0.0);
    assert_eq!(m.m[0][1], 0.0);
    assert_eq!(m.m[0][2], 0.0);
    assert_eq!(m.m[0][3], 0.0);
    assert_eq!(m.m[1][0], 0.0);
    assert_eq!(m.m[1][1], 0.0);
    assert_eq!(m.m[1][2], 0.0);
    assert_eq!(m.m[1][3], 0.0);
    assert_eq!(m.m[2][0], 0.0);
    assert_eq!(m.m[2][1], 0.0);
    assert_eq!(m.m[2][2], 0.0);
    assert_eq!(m.m[2][3], 0.0);
    assert_eq!(m.m[3][0], 0.0);
    assert_eq!(m.m[3][1], 0.0);
    assert_eq!(m.m[3][2], 0.0);
    assert_eq!(m.m[3][3], 0.0);
}

impl Mul for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Matrix { &self * &rhs }
}

impl<'a> Mul<Matrix> for &'a Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Matrix { self * &rhs }
}

impl<'a> Mul<&'a Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: &'a Matrix) -> Matrix { &self * rhs }
}

impl<'a, 'b> Mul<&'a Matrix> for &'b Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &'a Matrix) -> Matrix {
        macro_rules! row {
            ($col:expr) => ({
                let x = self.m[$col][0];
                let y = self.m[$col][1];
                let z = self.m[$col][2];
                let w = self.m[$col][3];
                [
                    (rhs.m[0][0]*x)+(rhs.m[1][0]*y)+(rhs.m[2][0]*z)+(rhs.m[3][0]*w),
                    (rhs.m[0][1]*x)+(rhs.m[1][1]*y)+(rhs.m[2][1]*z)+(rhs.m[3][1]*w),
                    (rhs.m[0][2]*x)+(rhs.m[1][2]*y)+(rhs.m[2][2]*z)+(rhs.m[3][2]*w),
                    (rhs.m[0][3]*x)+(rhs.m[1][3]*y)+(rhs.m[2][3]*z)+(rhs.m[3][3]*w),
                ]
            })
        }

        Matrix { m: [ row!(0), row!(1), row!(2), row!(3) ] }
    }
}

impl AsUniformValue for Matrix {
    fn as_uniform_value(&self) -> UniformValue<'static> {
        UniformValue::Mat4(self.m)
    }

    fn matches(ty: &UniformType) -> bool {
        ty == &UniformType::FloatMat4
    }
}
