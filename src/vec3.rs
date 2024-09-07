use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

#[derive(Copy, Clone, Default)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn len(&self) -> f64 {
        f64::sqrt(self.len_sq())
    }

    pub fn len_sq(&self) -> f64 {
        let [x, y, z] = self.e;
        x * x + y * y + z * z
    }

    pub fn xyz(&self) -> [f64; 3] {
        self.e
    }

    pub fn sum(&self) -> f64 {
        self.e[0] + self.e[1] + self.e[2]
    }

    pub fn unit(&self) -> Self {
        *self / self.len()
    }

    pub fn dot(&self, v: &Vec3) -> f64 {
        (*self * *v).sum()
    }
}

pub type Point3 = Vec3;

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self {
        let [x, y, z] = self.xyz();
        Vec3::new(-x, -y, -z)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Self {
        let [x, y, z] = self.xyz();
        let [x1, y1, z1] = v.xyz();

        Vec3::new(x + x1, y + y1, z + z1)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, v: Vec3) -> Self {
        let [x, y, z] = self.xyz();
        let [x1, y1, z1] = v.xyz();

        Vec3::new(x - x1, y - y1, z - z1)
    }
}

// vec3 * vec3
impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self {
        let [x, y, z] = self.xyz();
        let [x1, y1, z1] = v.xyz();

        Vec3::new(x * x1, y * y1, z * z1)
    }
}

// f64 * vec3
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        let [x, y, z] = v.xyz();
        Vec3::new(self * x, self * y, self * z)
    }
}

// vec3 * f64
impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Self {
        let [x, y, z] = self.xyz();
        Vec3::new(x * t, y * t, z * t)
    }
}

// vec3 / vec3
impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, v: Vec3) -> Self {
        let [x, y, z] = self.xyz();
        let [x1, y1, z1] = v.xyz();

        Vec3::new(x / x1, y / y1, z / z1)
    }
}

// f64 / vec3
impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, v: Vec3) -> Vec3 {
        let [x, y, z] = v.xyz();
        Vec3::new(self / x, self / y, self / z)
    }
}

// vec3 / f64
impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Self {
        let [x, y, z] = self.xyz();
        Vec3::new(x / t, y / t, z / t)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        *self = *self + v;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        *self = *self * t;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self = *self / t;
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.dot(&v)
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    let [x, y, z] = u.xyz();
    let [x1, y1, z1] = v.xyz();

    Vec3::new(y * z1 - y1 * z, z * x1 - z1 * x, x * y1 - x1 * y)
}
