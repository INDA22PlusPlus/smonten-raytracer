use std::ops::{Add, Mul, Div, Sub, Index, IndexMut};
use std::fmt;
use rand::Rng;


#[derive(Debug, Clone, PartialEq)]
pub struct Vec3 {
    xyz: [f32; 3]
}
impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { xyz: [x, y, z] }
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
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

pub fn clamp(v: &Vec3) -> Vec3{
    Vec3::new(
        clamp_f32(v[0]),
        clamp_f32(v[1]),
        clamp_f32(v[2])
    )
}

fn clamp_f32(f: f32) -> f32 {
    if f < 0.0 {
        return 0.0
    } else if f > 0.999 {
        return 0.999
    } else {
        return f
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

pub fn random_f32() -> f32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(0.0..0.9);
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    
    let rho = rng.gen_range(0.0..0.9);
    let theta = rng.gen_range(0.0..360.0);
    let phi = rng.gen_range(0.0..180.0);

    let x = rho * f32::sin(phi) * f32::cos(theta);
    let y = rho * f32::sin(phi) * f32::sin(theta);
    let z = rho * f32::cos(phi);

    return Vec3::new(x, y, z);
}

pub struct Color {
}
impl Color {
    pub fn red() -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
    pub fn blue() -> Vec3 {
        Vec3::new(0.0, 0.0, 1.0)
    }
    pub fn green() -> Vec3 {
        Vec3::new(0.0, 1.0, 0.0)
    }
    pub fn orange() -> Vec3 {
        Vec3::new(255.0, 165.0, 0.0) / 255.0
    }
    pub fn white() -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
    }
    pub fn black() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}