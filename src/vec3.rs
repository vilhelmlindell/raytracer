use image::Rgb;
use rand::{random, thread_rng, Rng};
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn normalized(&self) -> Vec3 {
        *self / self.length()
    }
    pub fn dot(v: &Self, u: &Self) -> f64 {
        v.x * u.x + v.y * u.y + v.z * u.z
    }
    pub fn cross(v: &Self, u: &Self) -> Self {
        Self {
            x: v.z * u.y - v.y * u.z,
            y: v.x * u.z - v.z * u.x,
            z: v.y * u.x - v.x * u.y,
        }
    }
    pub fn random() -> Self {
        Self::new(random(), random(), random())
    }
    pub fn random_in_range(min: &f64, max: &f64) -> Self {
        Self::new(
            thread_rng().gen_range(*min..*max),
            thread_rng().gen_range(*min..*max),
            thread_rng().gen_range(*min..*max),
        )
    }
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let point = Vec3::random_in_range(&-1.0, &1.0);
            if point.length_squared() < 1.0 {
                return point;
            }
        }
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl From<Vec3> for Rgb<u8> {
    fn from(value: Vec3) -> Self {
        Self([
            (value.x * 256.0) as u8,
            (value.y * 256.0) as u8,
            (value.z * 256.0) as u8,
        ])
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}
impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}
impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("({}, {}, {})", self.x, self.y, self.z);
        Ok(())
    }
}

mod test {
    #[test]
    fn test_vec3_add() {}
}
