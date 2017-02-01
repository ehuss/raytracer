extern crate raytracer;

use raytracer::*;

/// Get color for ray r cast into scene.
///
/// Very basic shading model where a hit on the surface yields a color that
/// represents the normal vector (mapped to range 0..1).
/// A miss into the background is a linear gradient from white to blue.
fn color<T: Hitable>(r: &Ray<f64>, world: &T) -> Vec3<f64> {
    if let Some(h) = world.hit(r, 0.0, std::f64::MAX) {
        return 0.5 * Vec3::new(h.normal.x + 1.0,
                          h.normal.y + 1.0,
                          h.normal.z + 1.0);
    } else {
        let unit_direction = r.direction().unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        return (1.0-t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    }
}

// Camera is at 0,0,0
// y-up
// into screen is -z
// Traverse from lower-left corner (-2, -1) to upper-right +(+2,+2)

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::zero();
    let mut world = HitableList::new();
    // Small sphere.
    world.add_hitable(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    // Really huge sphere (covers bottom of screen).
    world.add_hitable(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));
    for j in (0..ny - 1).rev() {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;
            let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);
            let col = color(&r, &world);
            let ir = (255.99 * col.x) as u32;
            let ig = (255.99 * col.y) as u32;
            let ib = (255.99 * col.z) as u32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
