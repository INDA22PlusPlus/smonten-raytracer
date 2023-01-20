mod vec3;


use vec3::{Vec3, length, normalize};


mod ray;
use ray::Ray;


fn distance_from_sphere(p: Vec3, c: Vec3, r: f32) -> f32 {
    length(p - c) - r
}

// fn distance_from_plane(p: Vec3, n: Vec3, q: Vec3) -> f32 {
//     let q_to_p = p - q; // q is a point in the plane
//     let shortest_dist = f32::abs(n.dot(q_to_p));
//     return shortest_dist;
// }

fn distance_from_plane(p: Vec3, a: f32, b: f32, c: f32, d: f32) -> f32 {
    // plane eq: ax + by + cz = d
    return f32::abs(a*p[0] + b*p[1] + c*p[2] + d) / f32::sqrt(a*a + b*b + c*c);
}



// fn ray_march(ray: Ray) -> Vec3 {
//     let mut total_distance_traveled: f32 = 0.0;
//     const NUMBER_OF_STEPS: i32 = 10;
//     const MINIMUM_HIT_DISTANCE: f32 = 0.001;
//     const MAXIMUM_TRACE_DISTANCE: f32 = 1000.0;

//     for i in 0..NUMBER_OF_STEPS {
//         let current_position = ray.clone().scale(total_distance_traveled);

//         let distance_to_closest = map_the_world(current_position.clone());

//         if distance_to_closest < MINIMUM_HIT_DISTANCE {
//             // let normal = calculate_normal(current_position.clone());


//             // // // HARD CODED LIGHT
//             // let light_position = Vec3::new(10.0, -10.0, -10.0);
            
//             // let direction_to_light = normalize(current_position - light_position);

//             // let diffuse_intensity: f32 = f32::max(0.0, normal.dot(direction_to_light));

//             // return Vec3::new(1.0, 0.0, 0.0) * diffuse_intensity;

//             let red = Vec3::new(1.0, 0.0, 0.0);
//             return red;

//             // return normal * 0.5 + 0.5;
            
//         } else if total_distance_traveled > MAXIMUM_TRACE_DISTANCE {
//             break;
//         } else {
//             total_distance_traveled += distance_to_closest;
//         }
//     }
//     // if we reach here we did not reach anything, return black
//     let black = Vec3::new(0.0, 0.0, 0.0);
//     return black;

// }

// fn map_the_world(p: Vec3) -> f32 {
//     let displacement =
//         f32::sin(5.0 * p[0]) *
//         f32::sin(5.0 * p[1]) *
//         f32::sin(5.0 * p[2]) * 0.1;
    
    

//     let sphere_0 = distance_from_sphere(
//         p.clone(),
//         Vec3::new(2.0, 0.0, -5.0),
//         2.0
//     );
//     let sphere_1 = distance_from_sphere(
//         p.clone(),
//         Vec3::new(-1.0, 0.0, -10.0),
//         2.0
//     );

//     return f32::min(sphere_0, sphere_1);
// }

fn distance_to_closest_hittable(p: &Vec3) -> f32 {
    
    let sphere_01 =  distance_from_sphere(
        p.clone(),
        Vec3::new(0.0, 0.0, 1.0),
        0.5
    );

    let sphere_02 =  distance_from_sphere(
        p.clone(),
        Vec3::new(-1.0, 0.0, -3.0),
        0.5
    );

    let plane_01 = distance_from_plane(
        p.clone(),
        0.0, -10.0, -2.0, -15.0
    );

    // return plane_01;

    let x = [sphere_01, sphere_02, plane_01, f32::NAN];
    let min = x.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    return min;

}
fn ray_march(ray: Ray) -> Vec3 {
    let max_iterations = 60;
    let max_tot_dist = 100.0;
    let mut cur_pos = ray.get_origin();
    let mut tot_dist_traveled = 0.0;

    let fade_dist = 20.0;

    for i in 0..max_iterations {
        let dist_to_closest = distance_to_closest_hittable(&cur_pos);
        if dist_to_closest < 0.01 {
            let factor = f32::max(0.0, 1.0 - tot_dist_traveled / fade_dist);
            return Vec3::red() * factor;
        }
        tot_dist_traveled += dist_to_closest;
        if tot_dist_traveled > max_tot_dist {
            break;
        }
        cur_pos = ray.scale(tot_dist_traveled);
    }
    return Vec3::black();
}

// fn calculate_normal(p: Vec3) -> Vec3 {
//     let small_step_x: Vec3 = Vec3::new(0.001, 0.0, 0.0);
//     let small_step_y: Vec3 = Vec3::new(0.0, 0.001, 0.0);
//     let small_step_z: Vec3 = Vec3::new(0.0, 0.0, 0.001);

//     let gradient_x: f32 = distance_to_closest_hittable(&(p + small_step_x.clone())) - distance_to_closest_hittable(&(p - small_step_x));
//     let gradient_y: f32 = distance_to_closest_hittable(&(p + small_step_y.clone())) - distance_to_closest_hittable(&(p - small_step_y));
//     let gradient_z: f32 = distance_to_closest_hittable(&(p + small_step_z.clone())) - distance_to_closest_hittable(&(p - small_step_z));
//     let normal = Vec3::new(gradient_x, gradient_y, gradient_z);
//     return normalize(&normal)
// }

fn render() {
    const WIDTH: i32 = 600;
    const ASPECT_RATIO: f32 = 16.0/9.0;
    const HEIGHT: i32 = (WIDTH as f32 / ASPECT_RATIO) as i32;

    let camera = Vec3::new(0.0, 0.0, 10.0);

    let u_range = 4.0;
    let v_range = u_range / ASPECT_RATIO;

    let lower_left_corner = Vec3::new(
        0.0 - u_range / 2.0, 
        0.0 - v_range / 2.0, 
        0.0
    );

    

    println!("P3\n{} {}\n255", WIDTH, HEIGHT);

    for y in (0..HEIGHT).rev() {
        for x in 0..WIDTH {
            let u: f32 = x as f32 / (WIDTH - 1) as f32;
            let v: f32 = y as f32 / (HEIGHT - 1) as f32;

            let uv = Vec3::new(
                lower_left_corner[0] + u * u_range,
                lower_left_corner[1] + v * v_range,
                -1.0
            );

            let ray = Ray::new(
                camera.clone(), 
                uv - camera.clone()
            );

            let color = ray_march(ray) * 255.999;
            println!("{}", color);

        }
    }
}


fn main() {
    // Image 

    const WIDTH: i32 = 256;
    const ASPECT_RATIO: f32 = 16.0/9.0;
    const HEIGHT: i32 = (WIDTH as f32 / ASPECT_RATIO) as i32;

    // // Camera
    // let viewport_height: f32 = 2.0;
    // let viewport_width:f32 = viewport_height * ASPECT_RATIO;
    // let focal_length: f32 = 1.0;

    // let origin = Vec3::new(0.0, 0.0, 0.0);
    // let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    // let vertical = Vec3::new(0.0, viewport_height, 0.0);
    // let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);


    // Render

    // println!("P3\n{} {}\n255", WIDTH, HEIGHT);
    // for j in (0..HEIGHT).rev() {
    //     for i in 0..WIDTH {
    //         let mut color = Vec3::new(
    //             i as f32 / (WIDTH-1) as f32,
    //             j as f32 / (HEIGHT-1) as f32,
    //             0.25
    //         );
    //         color = color * 255.999;
    //         println!("{}", color);
    //     }
    // }

    render();
    // my_test();



    // // TEST
    // let w = 10;
    // for x in 0..w {
    //     let u: f32 = x as f32 / (w - 1) as f32;
    //     println!("u = {}", u);
    // }


}


fn my_test() {
    let d = distance_from_plane(
        Vec3::new(0.0, 0.0, 10.0),
        0.0, -10.0, -2.0, -15.0
    );
    dbg!(d);
    // let ray = Ray::new(
    //     Vec3::new(0.0, 0.0, 0.0),
    //     Vec3::new(1.0, 1.0, 0.0)
    // );

    // dbg!(length(
    //     ray.scale(9.0)
    // ));

}