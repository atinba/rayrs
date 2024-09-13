use crate::utils::{rand_f64, rand_range_f64};

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
        self.len_sq().sqrt()
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

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        let [x, y, z] = self.xyz();
        let [x1, y1, z1] = v.xyz();

        Vec3::new(y * z1 - y1 * z, z * x1 - z1 * x, x * y1 - x1 * y)
    }

    pub fn rand() -> Vec3 {
        Vec3::new(rand_f64(), rand_f64(), rand_f64())
    }

    pub fn rand_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            rand_range_f64(min, max),
            rand_range_f64(min, max),
            rand_range_f64(min, max),
        )
    }

    pub fn rand_unit_vec() -> Vec3 {
        loop {
            let p = Vec3::rand_range(-1.0, 1.0);
            let len_sq = p.len_sq();
            if 1e-160 < len_sq && len_sq <= 1.0 {
                return p / len_sq.sqrt();
            }
        }
    }

    pub fn rand_vec_on_hemisphere(normal: &Vec3) -> Vec3 {
        let p = Vec3::rand_unit_vec();
        if p.dot(normal) > 0.0 {
            p
        } else {
            -p
        }
    }

    pub fn is_near_zero(&self) -> bool {
        const EPS: f64 = 1.0E-8;
        let [x, y, z] = self.xyz();

        x.abs() < EPS && y.abs() < EPS && z.abs() < EPS
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        *self - 2.0 * self.dot(normal) * (*normal)
    }

    pub fn refract(&self, normal: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min(-self.dot(&normal), 1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * *normal);
        let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.len_sq())) * *normal;
        r_out_perp + r_out_parallel
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);

        assert_eq!(v1.dot(&v2), 20.0);
    }
}
