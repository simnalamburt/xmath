use matrix::*;
use std::ops::*;

pub trait Vector {
    fn transform(&self, matrix: &Matrix) -> Self;

    fn multiply_add(&self, mul: &Self, add: &Self) -> Self;

    fn splat_x(&self) -> Self;
    fn splat_y(&self) -> Self;
    fn splat_z(&self) -> Self;
    fn splat_w(&self) -> Self;
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector for Vector2 {
    fn transform(&self, matrix: &Matrix) -> Self {

        let x = self.x * matrix[0][0] + self.y * matrix[1][0] + matrix[3][0];
        let y = self.x * matrix[0][1] + self.y * matrix[1][1] + matrix[3][1];
        Vector2 {
            x: x,
            y: y,
        }
    }

    fn multiply_add(&self, mul: &Self, add: &Self) -> Self {
        *self * *mul + *add
    }

    fn splat_x(&self) -> Self {
        let x = self.x;
        Vector2 {
            x: x,
            y: x,
        }
    }
    fn splat_y(&self) -> Self {
        let y = self.y;
        Vector2 {
            x: y,
            y: y,
        }
    }
    fn splat_z(&self) -> Self {
        let z = 0.0;
        Vector2 {
            x: z,
            y: z,
        }
    }
    fn splat_w(&self) -> Self {
        let w = 0.0;
        Vector2 {
            x: w,
            y: w,
        }
    }
}

impl Vector for Vector3 {
    fn transform(&self, matrix: &Matrix) -> Self {
        let x = self.x * matrix[0][0] + self.y * matrix[1][0] + self.z * matrix[2][0] + matrix[3][0];
        let y = self.x * matrix[0][1] + self.y * matrix[1][1] + self.z * matrix[2][1] + matrix[3][1];
        let z = self.x * matrix[0][2] + self.y * matrix[1][2] + self.z * matrix[2][2] + matrix[3][2];
        Vector3 {
            x: x,
            y: y,
            z: z,
        }
    }

    fn multiply_add(&self, mul: &Self, add: &Self) -> Self {
        *self * *mul + *add
    }

    fn splat_x(&self) -> Self {
        let x = self.x;
        Vector3 {
            x: x,
            y: x,
            z: x,
        }
    }
    fn splat_y(&self) -> Self {
        let y = self.y;
        Vector3 {
            x: y,
            y: y,
            z: y,
        }
    }
    fn splat_z(&self) -> Self {
        let z = self.z;
        Vector3 {
            x: z,
            y: z,
            z: z,
        }
    }
    fn splat_w(&self) -> Self {
        let w = 0.0;
        Vector3 {
            x: w,
            y: w,
            z: w,
        }
    }
}

impl Vector for Vector4 {
    fn transform(&self, matrix: &Matrix) -> Self {
        let x = self.x * matrix[0][0] + self.y * matrix[1][0] + self.z * matrix[2][0] + self.w * matrix[3][0];
        let y = self.x * matrix[0][1] + self.y * matrix[1][1] + self.z * matrix[2][1] + self.w * matrix[3][1];
        let z = self.x * matrix[0][2] + self.y * matrix[1][2] + self.z * matrix[2][2] + self.w * matrix[3][2];
        let w = self.x * matrix[0][3] + self.y * matrix[1][3] + self.z * matrix[2][3] + self.w * matrix[3][3];
        Vector4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    fn multiply_add(&self, mul: &Self, add: &Self) -> Self {
        *self * *mul + *add
    }

    fn splat_x(&self) -> Self {
        let x = self.x;
        Vector4 {
            x: x,
            y: x,
            z: x,
            w: x,
        }
    }
    fn splat_y(&self) -> Self {
        let y = self.y;
        Vector4 {
            x: y,
            y: y,
            z: y,
            w: y,
        }
    }
    fn splat_z(&self) -> Self {
        let z = self.z;
        Vector4 {
            x: z,
            y: z,
            z: z,
            w: z,
        }
    }
    fn splat_w(&self) -> Self {
        let w = self.w;
        Vector4 {
            x: w,
            y: w,
            z: w,
            w: w,
        }
    }
}

impl Add for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Vector2) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        Vector2 {
            x: x,
            y: y,
        }
    }
}
impl Sub for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: Vector2) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        Vector2 {
            x: x,
            y: y,
        }
    }
}
impl Div for Vector2 {
    type Output = Vector2;
    fn div(self, rhs: Vector2) -> Self::Output {
        let x = self.x / rhs.x;
        let y = self.y / rhs.y;
        Vector2 {
            x: x,
            y: y,
        }
    }
}
impl Mul for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Self::Output {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        Vector2 {
            x: x,
            y: y,
        }
    }
}


impl Add for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Vector3) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        Vector3 {
            x: x,
            y: y,
            z: z,
        }
    }
}
impl Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Vector3) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        Vector3 {
            x: x,
            y: y,
            z: z,
        }
    }
}
impl Div for Vector3 {
    type Output = Vector3;
    fn div(self, rhs: Vector3) -> Self::Output {
        let x = self.x / rhs.x;
        let y = self.y / rhs.y;
        let z = self.z / rhs.z;
        Vector3 {
            x: x,
            y: y,
            z: z,
        }
    }
}
impl Mul for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Self::Output {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        let z = self.z * rhs.z;
        Vector3 {
            x: x,
            y: y,
            z: z,
        }
    }
}

impl Add for Vector4 {
    type Output = Vector4;
    fn add(self, rhs: Vector4) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        let w = self.w + rhs.w;
        Vector4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
}
impl Sub for Vector4 {
    type Output = Vector4;
    fn sub(self, rhs: Vector4) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        let w = self.w - rhs.w;
        Vector4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
}
impl Mul for Vector4 {
    type Output = Vector4;
    fn mul(self, rhs: Vector4) -> Self::Output {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        let z = self.z * rhs.z;
        let w = self.w * rhs.w;
        Vector4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
}
impl Div for Vector4 {
    type Output = Vector4;
    fn div(self, rhs: Vector4) -> Self::Output {
        let x = self.x / rhs.x;
        let y = self.y / rhs.y;
        let z = self.z / rhs.z;
        let w = self.w / rhs.w;
        Vector4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
}
