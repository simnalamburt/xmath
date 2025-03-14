use crate::vector::{Vector, Vector3};
use std::mem::zeroed;
use std::ops::*;

pub type Row = [f32; 4];

#[derive(PartialEq, Clone, Debug)]
pub struct Matrix {
    m: [Row; 4],
}

impl Matrix {
    pub fn zero() -> Self {
        unsafe { zeroed() }
    }

    #[allow(clippy::too_many_arguments)]
    #[rustfmt::skip]
    pub fn new(
        m00: f32, m01: f32, m02: f32, m03: f32,
        m10: f32, m11: f32, m12: f32, m13: f32,
        m20: f32, m21: f32, m22: f32, m23: f32,
        m30: f32, m31: f32, m32: f32, m33: f32,
    ) -> Self {
        Matrix {
            m: [
                [m00, m01, m02, m03],
                [m10, m11, m12, m13],
                [m20, m21, m22, m23],
                [m30, m31, m32, m33],
            ]
        }
    }

    pub fn identity() -> Self {
        Matrix {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotation_x(rad: f32) -> Self {
        let (sin, cos) = rad.sin_cos();

        Matrix {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, cos, sin, 0.0],
                [0.0, -sin, cos, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotation_y(rad: f32) -> Self {
        let (sin, cos) = rad.sin_cos();

        Matrix {
            m: [
                [cos, 0.0, -sin, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [sin, 0.0, cos, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotation_z(rad: f32) -> Self {
        let (sin, cos) = rad.sin_cos();

        Matrix {
            m: [
                [cos, sin, 0.0, 0.0],
                [-sin, cos, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn orthographic(view_width: f32, view_height: f32, near_z: f32, far_z: f32) -> Self {
        // assert(!XMScalarNearEqual(ViewWidth, 0.0f, 0.00001f));
        // assert(!XMScalarNearEqual(ViewHeight, 0.0f, 0.00001f));
        // assert(!XMScalarNearEqual(FarZ, NearZ, 0.00001f));
        let f_range = 1.0 / (near_z - far_z);
        Matrix {
            m: [
                [2.0 / view_width, 0.0, 0.0, 0.0],
                [0.0, 2.0 / view_height, 0.0, 0.0],
                [0.0, 0.0, f_range, 0.0],
                [0.0, 0.0, f_range * near_z, 1.0],
            ],
        }
    }

    pub fn orthographic_off_center(
        view_left: f32,
        view_right: f32,
        view_bottom: f32,
        view_top: f32,
        near_z: f32,
        far_z: f32,
    ) -> Self {
        // reciprocal width and height
        let r_width = 1.0 / (view_right - view_left);
        let r_height = 1.0 / (view_top - view_bottom);
        let range = 1.0 / (near_z - far_z);

        Matrix {
            m: [
                [r_width + r_width, 0.0, 0.0, 0.0],
                [0.0, r_height + r_height, 0.0, 0.0],
                [0.0, 0.0, range, 0.0],
                [
                    -(view_left + view_right) * r_width,
                    -(view_top + view_bottom) * r_height,
                    range * near_z,
                    1.0,
                ],
            ],
        }
    }

    pub fn look_at(eye: Vector3, focus: Vector3, up: Vector3) -> Self {
        Self::look_to(eye, focus - eye, up)
    }

    pub fn look_to(eye: Vector3, dir: Vector3, up: Vector3) -> Self {
        assert!(dir != Vector3::zero());
        assert!(!dir.is_infinite());
        assert!(up != Vector3::zero());
        assert!(!up.is_infinite());

        let neg_eye = -eye;
        let neg_dir = -dir;

        let r2 = neg_dir.normalize();
        let r0 = up.cross(&r2).normalize();
        let r1 = r2.cross(&r0);

        let d0 = r0.dot(&neg_eye);
        let d1 = r1.dot(&neg_eye);
        let d2 = r2.dot(&neg_eye);

        Matrix {
            m: [
                [r0.x, r1.x, r2.x, 0.0],
                [r0.y, r1.y, r2.y, 0.0],
                [r0.z, r1.z, r2.z, 0.0],
                [d0, d1, d2, 1.0],
            ],
        }
    }

    pub fn perspective(width: f32, height: f32, near_z: f32, far_z: f32) -> Self {
        let two_near_z = near_z + near_z;
        let range = far_z / (near_z - far_z);

        Matrix {
            m: [
                [two_near_z / width, 0.0, 0.0, 0.0],
                [0.0, two_near_z / height, 0.0, 0.0],
                [0.0, 0.0, range, -1.0],
                [0.0, 0.0, range * near_z, 0.0],
            ],
        }
    }

    /// aspect: Width / Height
    pub fn perspective_fov(fov: f32, aspect: f32, near_z: f32, far_z: f32) -> Self {
        let (sin, cos) = (0.5 * fov).sin_cos();
        let f = cos / sin;
        let range = far_z / (near_z - far_z);

        Matrix {
            m: [
                [f / aspect, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, range, -1.0],
                [0.0, 0.0, range * near_z, 0.0],
            ],
        }
    }

    pub fn translation(ox: f32, oy: f32, oz: f32) -> Self {
        Matrix {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ox, oy, oz, 1.0],
            ],
        }
    }

    pub fn transpose(self) -> Self {
        Matrix {
            m: [
                [self.m[0][0], self.m[1][0], self.m[2][0], self.m[3][0]],
                [self.m[0][1], self.m[1][1], self.m[2][1], self.m[3][1]],
                [self.m[0][2], self.m[1][2], self.m[2][2], self.m[3][2]],
                [self.m[0][3], self.m[1][3], self.m[2][3], self.m[3][3]],
            ],
        }
    }
}

impl Mul for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Matrix {
        &self * &rhs
    }
}

impl Mul<Matrix> for &Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Matrix {
        self * &rhs
    }
}

impl<'a> Mul<&'a Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: &'a Matrix) -> Matrix {
        &self * rhs
    }
}

impl<'a> Mul<&'a Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &'a Matrix) -> Matrix {
        macro_rules! row {
            ($col:expr) => {{
                let x = self.m[$col][0];
                let y = self.m[$col][1];
                let z = self.m[$col][2];
                let w = self.m[$col][3];
                [
                    (rhs.m[0][0] * x) + (rhs.m[1][0] * y) + (rhs.m[2][0] * z) + (rhs.m[3][0] * w),
                    (rhs.m[0][1] * x) + (rhs.m[1][1] * y) + (rhs.m[2][1] * z) + (rhs.m[3][1] * w),
                    (rhs.m[0][2] * x) + (rhs.m[1][2] * y) + (rhs.m[2][2] * z) + (rhs.m[3][2] * w),
                    (rhs.m[0][3] * x) + (rhs.m[1][3] * y) + (rhs.m[2][3] * z) + (rhs.m[3][3] * w),
                ]
            }};
        }

        Matrix {
            m: [row!(0), row!(1), row!(2), row!(3)],
        }
    }
}

impl Index<usize> for Matrix {
    type Output = [f32; 4];
    fn index(&self, index: usize) -> &Self::Output {
        &self.m[index]
    }
}

/// glium_text support
impl From<Matrix> for [[f32; 4]; 4] {
    fn from(matrix: Matrix) -> Self {
        matrix.m
    }
}

#[cfg(feature = "glium-support")]
mod glium_support {
    use super::Matrix;
    use glium::uniforms::{AsUniformValue, UniformValue};

    impl AsUniformValue for Matrix {
        fn as_uniform_value(&self) -> UniformValue<'static> {
            UniformValue::Mat4(self.m)
        }
    }
}
