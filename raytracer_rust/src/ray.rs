

use crate::vec3::{Vec3, normalize};

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        let direction = normalize(&direction);
        Ray {origin, direction}
    }

    pub fn scale(&self, distance: f32) -> Vec3 {
        return self.get_origin() + self.get_direction() * distance;
    }

    pub fn get_origin(&self) -> Vec3 {
        return self.origin.clone();
    }

    pub fn get_direction(&self) -> Vec3 {
        return self.direction.clone();
    }
}