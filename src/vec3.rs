use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub};

use crate::rtweekend::*;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 {
            x: 0.00,
            y: 0.00,
            z: 0.00,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn set_x(&mut self, x: f64) -> Self {
        self.x = x;
        *self
    }
    pub fn set_y(&mut self, y: f64) -> Self {
        self.y = y;
        *self
    }
    pub fn set_z(&mut self, z: f64) -> Self {
        self.z = z;
        *self
    }
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }
    pub fn rand_vec3_01() -> Vec3 {
        Vec3::new(rand_01(), rand_01(), rand_01())
    }

    pub fn rand_vec3(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            rand_double(min, max),
            rand_double(min, max),
            rand_double(min, max),
        )
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }
}

impl Index<i32> for Vec3 {
    type Output = f64;
    fn index(&self, index: i32) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => &0.0,
        }
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 {
        x: u.y * v.z - u.z * v.y,
        y: u.z * v.x - u.x * v.z,
        z: u.x * v.y - u.y * v.x,
    }
}

//type alias for vec3
pub type Point3 = Vec3;
pub type Color = Vec3;

// pub fn write_color(pixel_color: Color, samples_per_pixel: usize) {
//     let mut r = pixel_color.x;
//     let mut g = pixel_color.y;
//     let mut b = pixel_color.z;

//     // Divide the color by the number of pixels
//     let scale = 1.0 / samples_per_pixel as f64;
//     // gamma-correct for gamma =2.0
//     r = (scale * r).sqrt();
//     g = (scale * g).sqrt();
//     b = (scale * b).sqrt();

//     // write the translated [0,255] value of each color component
//     let ir = (256.0 * clamp(r, 0.0, 0.999)) as i32;
//     let ig = (256.0 * clamp(g, 0.0, 0.999)) as i32;
//     let ib = (256.0 * clamp(b, 0.0, 0.999)) as i32;
//     println!("{} {} {}", ir, ig, ib);
// }

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::rand_vec3(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit_vector()
}

pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if dot(&in_unit_sphere, normal) > 0.0 {
        //in the same hemisphere with the normal
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::default()
            .set_x(rand_double(-1.0, 1.0))
            .set_y(rand_double(-1.0, 1.0));

        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * dot(&v, &n) * 2.0
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = fmin(dot(&uv, &-n), 1.0);
    let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
    let r_out_parallel = -n * (1.0 - r_out_perp.length_squared()).sqrt();
    r_out_perp + r_out_parallel
}

fn fmin(a: f64, b: f64) -> f64 {
    if a > b {
        b
    } else {
        a
    }
}
