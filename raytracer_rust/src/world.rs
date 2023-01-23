use crate::hittable::*;
use crate::vec3::{Vec3, Color, normalize, random_in_unit_sphere, random_f32};
use crate::ray::Ray;

pub struct World {
    hittables: Vec<Hittable>,
    width: i32,
    sun: Hittable
}
impl World {
    pub fn new(width: i32) -> World {
        let sun = Hittable {
            // default sun
            shape: Shape::sphere(Vec3::new(3.0, 8.0, 2.0), 1.0), material: Material::Light
        };
        World { hittables: vec![], width, sun}
    }

    pub fn add(&mut self, shape: Shape, material: Material) {
        self.hittables.push(
            Hittable::new(shape, material)
        );
    }

    pub fn set_sun(&mut self, shape: Shape) {
        self.sun = Hittable {
            shape, material: Material::Light
        }
    }

    pub fn render(self) {
        let WIDTH: i32 = self.width;
        let ASPECT_RATIO: f32 = 16.0/9.0;
        let HEIGHT: i32 = (WIDTH as f32 / ASPECT_RATIO) as i32;
        

        let camera = Vec3::new(0.0, 0.0, 10.0);

        let u_range = 4.0;
        let v_range = u_range / ASPECT_RATIO;

        let samples_per_pixel = 20;

        let lower_left_corner = Vec3::new(
            0.0 - u_range / 2.0, 
            0.0 - v_range / 2.0, 
            0.0
        );

        // preamble
        println!("P3\n{} {}\n255", WIDTH, HEIGHT);

        for y in (0..HEIGHT).rev() {
            for x in 0..WIDTH {
                let rows_remaining = y;
                dbg!(rows_remaining);
                let mut color = Color::black();

                for s in 0..samples_per_pixel {
                    let u: f32 = (x as f32 + random_f32()) / (WIDTH - 1) as f32;
                    let v: f32 = (y as f32 + random_f32()) / (HEIGHT - 1) as f32;
    
                    let uv = Vec3::new(
                        lower_left_corner[0] + u * u_range,
                        lower_left_corner[1] + v * v_range,
                        -1.0
                    );
    
                    let ray = Ray::new(
                        camera.clone(), 
                        uv - camera.clone()
                    );
                    
                    color = color + self.ray_trace(ray);
                    
                }

                //gamma correction
                let scale = 1.0 / (samples_per_pixel as f32);
                color = Vec3::new(
                    f32::sqrt(scale * color[0]),
                    f32::sqrt(scale * color[1]),
                    f32::sqrt(scale * color[2])
                ) * 255.999;
                println!("{}", color);
                
            }   
        }
    }

    // fn ray_march_02(&self, ray: Ray, mut total_dist_traveled: f32) -> Vec3 {
    //     // println!("total_dist_traveled = {}", total_dist_traveled);

    //     let nr_samples = 12;
    //     let max_iterations = 15;
    //     let max_tot_dist = 20.0;
    //     let mut local_dist_traveled = 0.0;

    //     let mut cur_pos = ray.get_origin();

    //     for i in 0..max_iterations {
    //         let (
    //             distance_to_closest_hittalbe, 
    //             material_closest_hittable
    //         ) = self.distance_to_and_material_of_closest_hittable(&cur_pos);

    //         local_dist_traveled += distance_to_closest_hittalbe;
    //         total_dist_traveled += distance_to_closest_hittalbe;
            

    //         // println!("dist to closest hittable = {}", distance_to_closest_hittalbe);
    //         // println!("tot dist traveled = {}", total_dist_traveled);
    //         // println!();

    //         if total_dist_traveled > max_tot_dist {break;}

    //         if distance_to_closest_hittalbe < 0.01 {
    //             let col = match material_closest_hittable {

    //                 Material::Light => Color::white(),

    //                 Material::Lambertian(color) => {
    //                     let mut col = Color::black();
    //                     for i in 0..nr_samples {
    //                         let normal_here = self.get_normal_at_surface_point(&cur_pos);
    //                         let new_direction = normal_here + random_in_unit_sphere();
    //                         let new_origin = cur_pos.clone() + normalize(&new_direction) * distance_to_closest_hittalbe;
    //                         let new_ray = Ray::new(new_origin, new_direction);

    //                         col = col + self.ray_march(new_ray, total_dist_traveled);
    //                     }
    //                     // col = col * 5.0 / nr_samples as f32;
    //                     return col / 2.0;

    //                     // return color;
    //                 }
    //             };
    //             return col;
    //         }
            

    //         cur_pos = ray.scale(local_dist_traveled);
    //     }

    //     // println!("reached end of ray_march()");

    //     return Color::black();
    // }


    fn reflection(&self, p: &Vec3) -> Vec3 {
        let dir_to_sun = match &self.sun.shape {
            Shape::Plane(plane) => {
                panic!()
            },
            Shape::Sphere(sphere) => {
                normalize(&(
                    sphere.c.clone() - p.clone()
                ))

            }
        };

        let normal = self.get_normal_at_surface_point(p);
        
        let dot_prod = normal.dot(&dir_to_sun);
        let mut col = Color::white();

        if dot_prod < 0.0 {
            return Color::black();
        } else {
            col = col * dot_prod;
        }

        let ray_to_sun = Ray::new(
            p.clone(),
            dir_to_sun.clone()
        );

        // check if in shadow
        for i in 0..self.hittables.len() {
            match self.hittables[i].is_hit(&ray_to_sun) {
                Ok(_) => {
                    // this point is blocked from light by another hittable
                    return Color::black();
                },
                Err(_) => () // continue looking for blocking hittables
            }
        }
        // no hittable found between p and light
        return col;

    }


    fn ray_trace(&self, ray: Ray) -> Vec3 {
        let max_distance = 30.0;

        if self.hittables.is_empty() {
            panic!("no hittables in world!");
        }

        let mut shortest_distance: Option<f32> = None;

        let mut index_closest_hittalbe = 0;

        for i in 0..self.hittables.len() {
            match self.hittables[i].is_hit(&ray) {
                Ok(new_distance) => {
                    match shortest_distance {
                        Some(closest_so_far) => {
                            if new_distance < closest_so_far {
                                shortest_distance = Some(new_distance);
                                index_closest_hittalbe = i;
                            }
                        },
                        None => {
                            shortest_distance = Some(new_distance);
                            index_closest_hittalbe = i;

                        }
                    }
                },
                Err(_) => ()
            }
        }

        match shortest_distance {
            Some(d) => {
                if d > max_distance {
                    return Color::black();
                } else {
                    let hit_point_col = match &self.hittables[index_closest_hittalbe].material {
                        Material::Light => panic!(),
                        Material::Lambertian(col) => col.clone()
                    };
                    let hit_point = ray.scale(d*0.99);
                    let col = self.reflection(&hit_point) * hit_point_col;
                    return col * (1.0 - d / max_distance);
                }
            },
            None => return Color::black(),
        }

    }

    pub fn get_normal_at_surface_point(&self, p: &Vec3) -> Vec3 {
        let index_to_closest_hittalbe = self.index_of_closest_hittable(p);
        let closest_hittable = &self.hittables[index_to_closest_hittalbe];
        return closest_hittable.get_normal(p);
    }

    // pub fn get_normal_at_surface_point_02(&self, p: &Vec3) -> Vec3 {

    //     let p = p.clone();

    //     let small_step_x: Vec3 = Vec3::new(0.001, 0.0, 0.0);
    //     let small_step_y: Vec3 = Vec3::new(0.0, 0.001, 0.0);
    //     let small_step_z: Vec3 = Vec3::new(0.0, 0.0, 0.001);

    //     let gradient_x: f32 = self.distance_to_closest_hittable(&(p.clone() + small_step_x.clone())) - self.distance_to_closest_hittable(&(p.clone()  - small_step_x));
    //     let gradient_y: f32 = self.distance_to_closest_hittable(&(p.clone()  + small_step_y.clone())) - self.distance_to_closest_hittable(&(p.clone()  - small_step_y));
    //     let gradient_z: f32 = self.distance_to_closest_hittable(&(p.clone()  + small_step_z.clone())) - self.distance_to_closest_hittable(&(p - small_step_z));
    //     let normal = Vec3::new(gradient_x, gradient_y, gradient_z);
    //     return normalize(&normal);
    // }

    fn index_of_closest_hittable(&self, p: &Vec3) -> usize {
        let mut index_closest: usize = 0;

        if self.hittables.is_empty() {panic!("No hittables in world!")}
        else if self.hittables.len() == 1 {return index_closest;}
        else {
            for i in 1..self.hittables.len() {
                if self.hittables[i].shortest_dist(p) < self.hittables[index_closest].shortest_dist(p) {
                    index_closest = i;
                }
            }
        }
        return index_closest;
    }

    // fn distance_to_and_material_of_closest_hittable(&self, p: &Vec3) -> (f32, Material) {
    //     let mut index_closest: usize = 0;

    //     if self.hittables.is_empty() {panic!("No hittables in world!")}
    //     else if self.hittables.len() == 1 {
    //         return (
    //             self.hittables[0].shortest_dist(p),
    //             self.hittables[0].material.clone()
    //         );
    //     }
    //     else {
    //         for i in 1..self.hittables.len() {
    //             if self.hittables[i].shortest_dist(p) < self.hittables[index_closest].shortest_dist(p) {
    //                 index_closest = i;
    //             }
    //         }
    //     }
    //     let distance_to_closest_hittable = self.hittables[index_closest].shortest_dist(p);
    //     let material_of_closest_hittalbe = self.hittables[index_closest].material.clone();
    //     return (
    //         distance_to_closest_hittable,
    //         material_of_closest_hittalbe
    //     )
    // }

    // fn distance_to_closest_hittable(&self, p: &Vec3) -> f32 {
    //     return self.distance_to_and_material_of_closest_hittable(p).0;
    // }


}