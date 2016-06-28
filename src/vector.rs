use std::f32;
use std::ops::*;
use matrix::{Matrix, Row};

pub trait Vector {
    fn zero() -> Self;
    fn one() -> Self;
    fn infinity() -> Self;
    fn nan() -> Self;
    fn epsilon() -> Self;
    fn replicate(value: f32) -> Self;

    fn dot(&self, other: &Self) -> f32;

    fn swizzle(&self, e0: usize, e1: usize, e2: usize, e3: usize) -> Self;
    fn permute(&self, other: &Self, permute_x: usize, permute_y: usize, permute_w: usize, permute_z: usize) -> Self;

    fn transform(&self, matrix: &Matrix) -> Self;

    fn min(&self, other: &Self) -> Self;
    fn max(&self, other: &Self) -> Self;

    fn round(&self) -> Self;
    fn trunc(&self) -> Self;
    fn floor(&self) -> Self;
    fn ceil(&self) -> Self;
    fn clamp(&self, min: &Self, max: &Self) -> Self;

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
    z: f32,
    w: f32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    w: f32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vector2 {
            x: x,
            y: y,
            z: 0.0,
            w: 0.0,
        }
    }
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 {
            x: x,
            y: y,
            z: z,
            w: 0.0,
        }
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vector3::new(
            self.y*other.z - self.z*other.y,
            self.z*other.x - self.x*other.z,
            self.x*other.y - self.y*other.x,
        )
    }
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vector4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
}

impl Vector for Vector2 {
    fn zero() -> Self {
        Self::replicate(0.0)
    }

    fn one() -> Self {
        Self::replicate(1.0)
    }

    fn infinity() -> Self {
        Self::replicate(f32::INFINITY)
    }

    fn nan() -> Self {
        Self::replicate(f32::NAN)
    }

    fn epsilon() -> Self {
        Self::replicate(f32::EPSILON)
    }

    fn replicate(value: f32) -> Self {
        Self::new(value, value)
    }

    fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    fn swizzle(&self, e0: usize, e1: usize, _e2: usize, _e3: usize) -> Self {
        assert!(e0 < 4);
        assert!(e1 < 4);
        Self::new(self[e0], self[e1])
    }

    fn permute(&self, other: &Self, permute_x: usize, permute_y: usize, _permute_z: usize, _permute_w: usize) -> Self {
        assert!(permute_x < 8);
        assert!(permute_y < 8);
        let x = if permute_x < 4 { self[permute_x] } else { other[permute_x - 4] };
        let y = if permute_y < 4 { self[permute_y] } else { other[permute_y - 4] };
        Self::new(x, y)
    }

    fn transform(&self, matrix: &Matrix) -> Self {
        let x = self.splat_x();
        let y = self.splat_y();

        let m0 = Self::from(matrix[0]);
        let m1 = Self::from(matrix[1]);
        let m3 = Self::from(matrix[3]);

        x * m0 + y * m1 + m3
    }

    fn min(&self, other: &Self) -> Self {
        let x = self.x.min(other.x);
        let y = self.y.min(other.y);
        Self::new(x, y)
    }
    fn max(&self, other: &Self) -> Self {
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        Self::new(x, y)
    }

    fn round(&self) -> Self {
        let x = self.x.round();
        let y = self.y.round();
        Self::new(x, y)
    }
    fn trunc(&self) -> Self {
        let x = self.x.trunc();
        let y = self.y.trunc();
        Self::new(x, y)
    }
    fn floor(&self) -> Self {
        let x = self.x.floor();
        let y = self.y.floor();
        Self::new(x, y)
    }
    fn ceil(&self) -> Self {
        let x = self.x.ceil();
        let y = self.y.ceil();
        Self::new(x, y)
    }
    fn clamp(&self, min: &Self, max: &Self) -> Self {
        assert!(min.x < max.x);
        assert!(min.y < max.y);
        self.max(min).min(max)
    }

    fn multiply_add(&self, mul: &Self, add: &Self) -> Self {
        *self * *mul + *add
    }

    fn splat_x(&self) -> Self {
        Self::replicate(self.x)
    }
    fn splat_y(&self) -> Self {
        Self::replicate(self.y)
    }
    fn splat_z(&self) -> Self {
        Self::replicate(0.0)
    }
    fn splat_w(&self) -> Self {
        Self::replicate(0.0)
    }
}

impl Vector for Vector3 {
    fn zero() -> Self {
        Self::replicate(0.0)
    }
    fn one() -> Self {
        Self::replicate(1.0)
    }

    fn infinity() -> Self {
        Self::replicate(f32::INFINITY)
    }

    fn nan() -> Self {
        Self::replicate(f32::NAN)
    }

    fn epsilon() -> Self {
        Self::replicate(f32::EPSILON)
    }

    fn replicate(value: f32) -> Self {
        Self::new(value, value, value)
    }

    fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn swizzle(&self, e0: usize, e1: usize, e2: usize, _e3: usize) -> Self {
        assert!(e0 < 4);
        assert!(e1 < 4);
        assert!(e2 < 4);
        Self::new(self[e0], self[e1], self[e2])
    }

    fn permute(&self, other: &Self, permute_x: usize, permute_y: usize, permute_z: usize, _permute_w: usize) -> Self {
        assert!(permute_x < 8);
        assert!(permute_y < 8);
        assert!(permute_z < 8);
        let x = if permute_x < 4 { self[permute_x] } else { other[permute_x - 4] };
        let y = if permute_y < 4 { self[permute_y] } else { other[permute_y - 4] };
        let z = if permute_z < 4 { self[permute_z] } else { other[permute_z - 4] };
        Self::new(x, y, z)
    }

    fn transform(&self, matrix: &Matrix) -> Self {
        let x = self.splat_x();
        let y = self.splat_y();
        let z = self.splat_z();

        let m0 = Self::from(matrix[0]);
        let m1 = Self::from(matrix[1]);
        let m2 = Self::from(matrix[2]);
        let m3 = Self::from(matrix[3]);

        x * m0 + y * m1 + z * m2 + m3
    }

    fn min(&self, other: &Self) -> Self {
        let x = self.x.min(other.x);
        let y = self.y.min(other.y);
        let z = self.z.min(other.z);
        Self::new(x, y, z)
    }
    fn max(&self, other: &Self) -> Self {
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let z = self.z.max(other.z);
        Self::new(x, y, z)
    }

    fn round(&self) -> Self {
        let x = self.x.round();
        let y = self.y.round();
        let z = self.z.round();
        Self::new(x, y, z)
    }
    fn trunc(&self) -> Self {
        let x = self.x.trunc();
        let y = self.y.trunc();
        let z = self.z.trunc();
        Self::new(x, y, z)
    }
    fn floor(&self) -> Self {
        let x = self.x.floor();
        let y = self.y.floor();
        let z = self.z.floor();
        Self::new(x, y, z)
    }
    fn ceil(&self) -> Self {
        let x = self.x.ceil();
        let y = self.y.ceil();
        let z = self.z.ceil();
        Self::new(x, y, z)
    }
    fn clamp(&self, min: &Self, max: &Self) -> Self {
        assert!(min.x < max.x);
        assert!(min.y < max.y);
        assert!(min.z < max.z);
        self.max(min).min(max)
    }

    fn multiply_add(&self, mul: &Self, add: &Self) -> Self {
        *self * *mul + *add
    }

    fn splat_x(&self) -> Self {
        Self::replicate(self.x)
    }
    fn splat_y(&self) -> Self {
        Self::replicate(self.y)
    }
    fn splat_z(&self) -> Self {
        Self::replicate(self.z)
    }
    fn splat_w(&self) -> Self {
        Self::replicate(0.0)
    }
}

impl Vector for Vector4 {
    fn zero() -> Self {
        Self::replicate(0.0)
    }
    fn one() -> Self {
        Self::replicate(1.0)
    }

    fn infinity() -> Self {
        Self::replicate(f32::INFINITY)
    }

    fn nan() -> Self {
        Self::replicate(f32::NAN)
    }

    fn epsilon() -> Self {
        Self::replicate(f32::EPSILON)
    }

    fn replicate(value: f32) -> Self {
        Self::new(value, value, value, value)
    }

    fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    fn swizzle(&self, e0: usize, e1: usize, e2: usize, e3: usize) -> Self {
        assert!(e0 < 4);
        assert!(e1 < 4);
        assert!(e2 < 4);
        assert!(e3 < 4);
        Self::new(self[e0], self[e1], self[e2], self[e3])
    }

    fn permute(&self, other: &Self, permute_x: usize, permute_y: usize, permute_z: usize, permute_w: usize) -> Self {
        assert!(permute_x < 8);
        assert!(permute_y < 8);
        assert!(permute_z < 8);
        assert!(permute_w < 8);
        let x = if permute_x < 4 { self[permute_x] } else { other[permute_x - 4] };
        let y = if permute_y < 4 { self[permute_y] } else { other[permute_y - 4] };
        let z = if permute_z < 4 { self[permute_z] } else { other[permute_z - 4] };
        let w = if permute_w < 4 { self[permute_w] } else { other[permute_w - 4] };
        Self::new(x, y, z, w)
    }

    fn transform(&self, matrix: &Matrix) -> Self {
        let x = self.splat_x();
        let y = self.splat_y();
        let z = self.splat_z();
        let w = self.splat_w();

        let m0 = Self::from(matrix[0]);
        let m1 = Self::from(matrix[1]);
        let m2 = Self::from(matrix[2]);
        let m3 = Self::from(matrix[3]);

        x * m0 + y * m1 + z * m2 + w * m3
    }

    fn min(&self, other: &Self) -> Self {
        let x = self.x.min(other.x);
        let y = self.y.min(other.y);
        let z = self.z.min(other.z);
        let w = self.w.min(other.w);
        Self::new(x, y, z, w)
    }
    fn max(&self, other: &Self) -> Self {
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let z = self.z.max(other.z);
        let w = self.w.max(other.w);
        Self::new(x, y, z, w)
    }

    fn round(&self) -> Self {
        let x = self.x.round();
        let y = self.y.round();
        let z = self.z.round();
        let w = self.w.round();
        Self::new(x, y, z, w)
    }
    fn trunc(&self) -> Self {
        let x = self.x.trunc();
        let y = self.y.trunc();
        let z = self.z.trunc();
        let w = self.w.trunc();
        Self::new(x, y, z, w)
    }
    fn floor(&self) -> Self {
        let x = self.x.floor();
        let y = self.y.floor();
        let z = self.z.floor();
        let w = self.w.floor();
        Self::new(x, y, z, w)
    }
    fn ceil(&self) -> Self {
        let x = self.x.ceil();
        let y = self.y.ceil();
        let z = self.z.ceil();
        let w = self.w.ceil();
        Self::new(x, y, z, w)
    }
    fn clamp(&self, min: &Self, max: &Self) -> Self {
        assert!(min.x < max.x);
        assert!(min.y < max.y);
        assert!(min.z < max.z);
        assert!(min.w < max.w);
        self.max(min).min(max)
    }


    fn multiply_add(&self, mul: &Self, add: &Self) -> Self {
        *self * *mul + *add
    }

    fn splat_x(&self) -> Self {
        Self::replicate(self.x)
    }
    fn splat_y(&self) -> Self {
        Self::replicate(self.y)
    }
    fn splat_z(&self) -> Self {
        Self::replicate(self.z)
    }
    fn splat_w(&self) -> Self {
        Self::replicate(self.w)
    }
}


//
// Operator overloadings
//

impl Add for Vector2 {
    type Output = Self;
    fn add(self, rhs: Vector2) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        Self::new(x, y)
    }
}
impl Add for Vector3 {
    type Output = Self;
    fn add(self, rhs: Vector3) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        Self::new(x, y, z)
    }
}
impl Add for Vector4 {
    type Output = Self;
    fn add(self, rhs: Vector4) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        let w = self.w + rhs.w;
        Self::new(x, y, z, w)
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl AddAssign for Vector4 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}


impl Sub for Vector2 {
    type Output = Self;
    fn sub(self, rhs: Vector2) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        Self::new(x, y)
    }
}
impl Sub for Vector3 {
    type Output = Self;
    fn sub(self, rhs: Vector3) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        Self::new(x, y, z)
    }
}
impl Sub for Vector4 {
    type Output = Self;
    fn sub(self, rhs: Vector4) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        let w = self.w - rhs.w;
        Self::new(x, y, z, w)
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
impl SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl SubAssign for Vector4 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}


impl Div for Vector2 {
    type Output = Self;
    fn div(self, rhs: Vector2) -> Self::Output {
        let x = self.x / rhs.x;
        let y = self.y / rhs.y;
        Self::new(x, y)
    }
}
impl Div for Vector3 {
    type Output = Self;
    fn div(self, rhs: Vector3) -> Self::Output {
        let x = self.x / rhs.x;
        let y = self.y / rhs.y;
        let z = self.z / rhs.z;
        Self::new(x, y, z)
    }
}
impl Div for Vector4 {
    type Output = Self;
    fn div(self, rhs: Vector4) -> Self::Output {
        let x = self.x / rhs.x;
        let y = self.y / rhs.y;
        let z = self.z / rhs.z;
        let w = self.w / rhs.w;
        Self::new(x, y, z, w)
    }
}

impl DivAssign for Vector2 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}
impl DivAssign for Vector3 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}
impl DivAssign for Vector4 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
        self.w /= rhs.w;
    }
}



impl Div<f32> for Vector2 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        let x = self.x / rhs;
        let y = self.y / rhs;
        Self::new(x, y)
    }
}
impl Div<f32> for Vector3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        let x = self.x / rhs;
        let y = self.y / rhs;
        let z = self.z / rhs;
        Self::new(x, y, z)
    }
}
impl Div<f32> for Vector4 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        let x = self.x / rhs;
        let y = self.y / rhs;
        let z = self.z / rhs;
        let w = self.w / rhs;
        Self::new(x, y, z, w)
    }
}

impl DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
impl DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}
impl DivAssign<f32> for Vector4 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
}


impl Mul for Vector2 {
    type Output = Self;
    fn mul(self, rhs: Vector2) -> Self::Output {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        Self::new(x, y)
    }
}
impl Mul for Vector3 {
    type Output = Self;
    fn mul(self, rhs: Vector3) -> Self::Output {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        let z = self.z * rhs.z;
        Self::new(x, y, z)
    }
}
impl Mul for Vector4 {
    type Output = Self;
    fn mul(self, rhs: Vector4) -> Self::Output {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        let z = self.z * rhs.z;
        let w = self.w * rhs.w;
        Self::new(x, y, z, w)
    }
}

impl MulAssign for Vector2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}
impl MulAssign for Vector3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}
impl MulAssign for Vector4 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self.w *= rhs.w;
    }
}


impl Mul<f32> for Vector2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;
        Self::new(x, y)
    }
}
impl Mul<f32> for Vector3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;
        let z = self.z * rhs;
        Self::new(x, y, z)
    }
}
impl Mul<f32> for Vector4 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;
        let z = self.z * rhs;
        let w = self.w * rhs;
        Self::new(x, y, z, w)
    }
}

impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}
impl MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
impl MulAssign<f32> for Vector4 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

impl Mul<Vector2> for f32 {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Self::Output {
        let x = self * rhs.x;
        let y = self * rhs.y;
        Self::Output::new(x, y)
    }
}
impl Mul<Vector3> for f32 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Self::Output {
        let x = self * rhs.x;
        let y = self * rhs.y;
        let z = self * rhs.z;
        Self::Output::new(x, y, z)
    }
}
impl Mul<Vector4> for f32 {
    type Output = Vector4;
    fn mul(self, rhs: Vector4) -> Self::Output {
        let x = self * rhs.x;
        let y = self * rhs.y;
        let z = self * rhs.z;
        let w = self * rhs.w;
        Self::Output::new(x, y, z, w)
    }
}


impl Neg for Vector2 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let x = -self.x;
        let y = -self.y;
        Self::new(x, y)
    }
}
impl Neg for Vector3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let x = -self.x;
        let y = -self.y;
        let z = -self.z;
        Self::new(x, y, z)
    }
}
impl Neg for Vector4 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let x = -self.x;
        let y = -self.y;
        let z = -self.z;
        let w = -self.w;
        Self::new(x, y, z, w)
    }
}

const ZERO: &'static f32 = &0.0;

impl Index<usize> for Vector2 {
    type Output = f32;
    fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => ZERO,
            3 => ZERO,
            _ => panic!("index must be between 0~3, but {}", index),
        }
    }
}
impl Index<usize> for Vector3 {
    type Output = f32;
    fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => ZERO,
            _ => panic!("index must be between 0~3, but {}", index),
        }
    }
}
impl Index<usize> for Vector4 {
    type Output = f32;
    fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("index must be between 0~3, but {}", index),
        }
    }
}


impl From<Row> for Vector2 {
    fn from(row: Row) -> Self {
        Self::new(row[0], row[1])
    }
}

impl From<Row> for Vector3 {
    fn from(row: Row) -> Self {
        Self::new(row[0], row[1], row[2])
    }
}

impl From<Row> for Vector4 {
    fn from(row: Row) -> Self {
        Self::new(row[0], row[1], row[2], row[3])
    }
}


#[cfg(feature = "glium-support")]
mod glium_support {
    use super::{Vector2, Vector3, Vector4};
    use glium::vertex::{Attribute, AttributeType};

    unsafe impl Attribute for Vector2 { fn get_type() -> AttributeType { AttributeType::F32F32 } }
    unsafe impl Attribute for Vector3 { fn get_type() -> AttributeType { AttributeType::F32F32F32 } }
    unsafe impl Attribute for Vector4 { fn get_type() -> AttributeType { AttributeType::F32F32F32F32 } }
}
