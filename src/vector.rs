use matrix::*;

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

