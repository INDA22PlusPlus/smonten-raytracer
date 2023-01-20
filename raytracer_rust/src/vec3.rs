use std::ops::{Add, Mul, Div, Sub, Index, IndexMut};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Vec3 {
    xyz: [f32; 3]
}
impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { xyz: [x, y, z] }
    }

    pub fn dot(self, other: Vec3) -> f32 {
        self[0]*other[0] + self[1]*other[1] + self[2]*other[2]
    }

    pub fn red() -> Vec3 {
        Vec3 { xyz: [1.0, 0.0, 0.0] }
    }

    pub fn black() -> Vec3 {
        Vec3 { xyz: [0.0, 0.0, 0.0] }
    }

    pub fn white() -> Vec3 {
        Vec3 { xyz: [1.0, 1.0, 1.0] }
    }
}
//////////////////////////////////////// INDEXING
impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, i: usize) -> &f32 {
        match i {
            0 => &self.xyz[0],
            1 => &self.xyz[1],
            2 => &self.xyz[2],
            _ => panic!("index out of bound: {}", i),
        }
    }
}
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        match i {
            0 => &mut self.xyz[0],
            1 => &mut self.xyz[1],
            2 => &mut self.xyz[2],
            _ => panic!("index out of bound: {}", i),
        }
    }
}
//////////////////////////////////////// ADD
impl Add for Vec3 {
    type Output = Vec3;
    fn add(mut self, B: Vec3) -> Vec3 {
        self[0] += B[0];
        self[1] += B[1];
        self[2] += B[2];
        self
    }
}
impl Add<f32> for Vec3 {
    type Output = Vec3;
    fn add(mut self, f: f32) -> Vec3 {
        let F = Vec3::new(f, f, f);
        self = self + F;
        self
    }
}

//////////////////////////////////////// SUB
impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(mut self, B: Vec3) -> Vec3 {
        self[0] -= B[0];
        self[1] -= B[1];
        self[2] -= B[2];
        self
    }
}
impl Sub<f32> for Vec3 {
    type Output = Vec3;
    fn sub(mut self, f: f32) -> Vec3 {
        let F = Vec3::new(f, f, f);
        self = self - F;
        self
    }
}

//////////////////////////////////////// MUL
impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(mut self, B: Vec3) -> Vec3 {
        self[0] *= B[0];
        self[1] *= B[1];
        self[2] *= B[2];
        self
    }
}
impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(mut self, f: f32) -> Vec3 {
        let F = Vec3::new(f, f, f);
        self = self * F;
        self
    }
}

//////////////////////////////////////// DIV
impl Div for Vec3 {
    type Output = Vec3;
    fn div(mut self, B: Vec3) -> Vec3 {
        self[0] /= B[0];
        self[1] /= B[1];
        self[2] /= B[2];
        self
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(mut self, f: f32) -> Vec3 {
        let F = Vec3::new(f, f, f);
        self = self / F;
        self
    }
}
//////////////////////////////////////// DISPLAY

impl fmt::Display for Vec3 {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{} {} {}", self[0] as i32, self[1] as i32, self[2] as i32)
    }
}

pub fn length(v: Vec3) -> f32 {
    f32::sqrt(
        v[0]*v[0] + v[1]*v[1] + v[2]*v[2]
    )
}

pub fn normalize(v: &Vec3) -> Vec3 {
    v.clone() / length(v.clone())
}
