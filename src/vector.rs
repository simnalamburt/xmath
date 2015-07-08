use matrix::*;

pub trait Vector {
    fn transform(self, matrix: &Matrix) -> Self;

    fn multiply_add(self, mul: &Self, add: &Self) -> Self;

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

        let x = self.0 * matrix[0][0] + self.1 * matrix[1][0] + matrix[3][0];
        let y = self.0 * matrix[0][1] + self.1 * matrix[1][1] + matrix[3][1];
        (x, y)
    }

    fn multiply_add(self, mul: &Self, add: &Self) -> Self {
        let x = self.0 * mul.0 + add.0;
        let y = self.1 * mul.1 + add.1;
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
        let x = self.0 * matrix[0][0] + self.1 * matrix[1][0] + self.2 * matrix[2][0] + matrix[3][0];
        let y = self.0 * matrix[0][1] + self.1 * matrix[1][1] + self.2 * matrix[2][1] + matrix[3][1];
        let z = self.0 * matrix[0][2] + self.1 * matrix[1][2] + self.2 * matrix[2][2] + matrix[3][2];
        (x, y, z)
    }

    fn multiply_add(self, mul: &Self, add: &Self) -> Self {
        let x = self.0 * mul.0 + add.0;
        let y = self.1 * mul.1 + add.1;
        let z = self.2 * mul.2 + add.2;
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
        let x = self.0 * matrix[0][0] + self.1 * matrix[1][0] + self.2 * matrix[2][0] + self.3 * matrix[3][0];
        let y = self.0 * matrix[0][1] + self.1 * matrix[1][1] + self.2 * matrix[2][1] + self.3 * matrix[3][1];
        let z = self.0 * matrix[0][2] + self.1 * matrix[1][2] + self.2 * matrix[2][2] + self.3 * matrix[3][2];
        let w = self.0 * matrix[0][3] + self.1 * matrix[1][3] + self.2 * matrix[2][3] + self.3 * matrix[3][3];
        (x, y, z, w)
    }

    fn multiply_add(self, mul: &Self, add: &Self) -> Self {
        let x = self.0 * mul.0 + add.0;
        let y = self.1 * mul.1 + add.1;
        let z = self.2 * mul.2 + add.2;
        let w = self.3 * mul.3 + add.3;
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
