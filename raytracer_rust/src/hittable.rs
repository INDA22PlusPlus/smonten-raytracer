

use crate::{Vec3, length, Ray, vec3::normalize};

pub struct Hittable {
    pub shape: Shape,
    pub material: Material
}
impl Hittable {
    pub fn new(shape: Shape, material: Material) -> Hittable {
        Hittable { shape, material }
    }
    pub fn shortest_dist(&self, p: &Vec3) -> f32 {
        match &self.shape {
            
            Shape::Plane(plane) => {
                // print!("looking at plane, ");
                let a = plane.a;
                let b = plane.b;
                let c = plane.c;
                let d = plane.d;

                // plane eq: ax + by + cz = d
                let dist = f32::abs(a*p[0] + b*p[1] + c*p[2] - d) / f32::sqrt(a*a + b*b + c*c);
                // println!("dist = {}", dist);
                dist
            },
            Shape::Sphere(sphere) => {
                // print!("looking at sphere, ");
                let dist = length(p.clone() - sphere.c.clone()) - sphere.r.clone();
                // println!("dist = {}", dist);
                dist
            }
        }
    }

    pub fn get_normal(&self, p: &Vec3) -> Vec3 {
        // println!("getting normal at = ({}, {}, {})", p[0], p[1], p[2]);
        match &self.shape {
            Shape::Plane(plane) => {
                // println!("this is a plane");
                normalize(
                    &Vec3::new(plane.a, plane.b, plane.c)
                )
            },
            Shape::Sphere(sphere) => {
                // println!("this is a sphere");
                normalize(
                    &(p.clone() - sphere.c.clone())
                )
            }
        }
    }

    pub fn is_hit(&self, ray: &Ray) -> Result<f32, ()> {
        match &self.shape {
            Shape::Plane(plane) => {

                let origin = ray.get_origin();
                let dir = ray.get_direction();
                let (a, b, c) = (origin[0], origin[1], origin[2]);
                let (d, e, f) = (dir[0], dir[1], dir[2]);
                let (A, B, C, D) = (plane.a, plane.b, plane.c, plane.d);

                let dir_dot_norm = A*d + B*e + C*f;

                if f32::abs(dir_dot_norm) < 0.001 {
                    // line is essentially parallel with plane
                    return Err(());
                }

                let t = (D - A*a - B*b - C*c) / dir_dot_norm;

                if t < 0.0 {
                    // plane intersection is in opposite direction from point
                    return Err(());
                }

                // distance is ||dir||*t
                return Ok(t);
            },
            Shape::Sphere(sphere) => {
                let oc = ray.get_origin() - sphere.c.clone();
                let a = ray.get_direction().dot(&ray.get_direction());
                let b = 2.0 * oc.dot(&ray.get_direction());
                let c = oc.dot(&oc) - sphere.r * sphere.r;
                let discriminant = b*b - 4.0*a*c;
                if discriminant < 0.0 {
                    return Err(());
                } else {
                    let dist = (-b - f32::sqrt(discriminant)) / (2.0 * a);
                    
                    // check so that we look along the direction of ray and not opposite!
                    if dist < 0.0 {
                        return Err(());
                    }

                    // println!("dist = {}", dist);


                    return Ok(
                        dist
                    );
                }
            }
        }
    }

    pub fn distance(&self, p: &Vec3) -> f32 {
        match &self.shape {
            Shape::Plane(plane) => panic!(),
            Shape::Sphere(sphere) => {

            }
        }
        panic!()
    }
}
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane)
}
impl Shape {
    pub fn plane(a: f32, b: f32, c: f32, d: f32) -> Shape {
        Shape::Plane(Plane { a, b, c, d })
    }
    pub fn sphere(c: Vec3, r: f32) -> Shape {
        Shape::Sphere(Sphere{ c, r })
    }
}
pub struct Sphere {
    pub c: Vec3, pub r: f32
}
pub struct Plane {
    pub a: f32, pub b: f32, pub c: f32, pub d: f32
}
#[derive(Debug, Clone)]
pub enum Material {
    Light,
    Lambertian(Vec3)
}