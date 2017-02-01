extern crate raytracer;

use raytracer::vec3::*;

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);
    for j in (0..ny - 1).rev() {
        for i in 0..nx {
            let col = Vec3::new(i as f64 / nx as f64,
                j as f64 / ny as f64, 0.2);
            let ir = (255.99 * col.x) as u32;
            let ig = (255.99 * col.y) as u32;
            let ib = (255.99 * col.z) as u32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
