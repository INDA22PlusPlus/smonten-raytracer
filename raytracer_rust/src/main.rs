fn main() {
    const aspect_ratio: f32 = 16.0 / 9.0;
    const image_width: i32 = 1200;
    const image_height: i32 = (image_width as f32 / aspect_ratio) as i32;
    const samples_per_pixel: i32 = 10;
    const max_depth: i32 = 50;

    for j in (0..image_height).rev() {
        println!("{}", j);
    }
    println!("image_height = {}", image_height);
    println!("hej");
    // ; j >= 0; --j) {
    //     std::cerr << "\rScanlines remaining: " << j << ' ' << std::flush;
    //     for (int i = 0; i < image_width; ++i) {
    //         color pixel_color(0,0,0);
    //         for (int s = 0; s < samples_per_pixel; ++s) {
    //             auto u = (i + random_double()) / (image_width-1);
    //             auto v = (j + random_double()) / (image_height-1);
    //             ray r = cam.get_ray(u, v);
    //             pixel_color += ray_color(r, world, max_depth);
    //         }
    //         write_color(std::cout, pixel_color, samples_per_pixel);
    //     }
    // }
}
