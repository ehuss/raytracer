extern crate raytracer;
extern crate rand;

use raytracer::*;

/// Get color for ray r cast into scene.
///
/// A miss into the background is a linear gradient from white to blue.
fn color<T: Hitable>(rng: &mut Rng, r: &Ray<f64>, world: &T, depth: u8) -> Vec3<f64>
{
    // Use 0.0001 to ignore hits very near zero (the ray should travel at
    // least some distance).
    if let Some(h) = world.hit(r, 0.0001, std::f64::MAX) {
        if depth < 50 {
            if let Some((scattered, attenuation)) = h.material.scatter(rng, r, &h) {
                return attenuation * color(rng, &scattered, world, depth+1);
            }
        }
        return Vec3::zero();
    } else {
        // Hit background.
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
    let nx = 400;
    let ny = 200;
    let ns = 100;
    println!("P3\n{} {}\n255", nx, ny);
    let cam = Camera::new();
    let mut world = HitableList::new();
    // Small sphere.
    world.add_hitable(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Rc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)))));
    // Really huge sphere (covers bottom of screen).
    world.add_hitable(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)))));
    world.add_hitable(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.2))));
    world.add_hitable(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Rc::new(Dielectric::new(1.5))));
    let mut rng = Rng::new();
    for j in (0..ny - 1).rev() {
        for i in 0..nx {
            let mut col = Vec3::<f64>::zero();
            for _ in 0..ns {
                let u = (i as f64 + rng.rand64()) / nx as f64;
                let v = (j as f64 + rng.rand64()) / ny as f64;
                let r = cam.get_ray(u, v);
                col += color(&mut rng, &r, &world, 0);
            }
            col /= ns as f64;
            // Poor-man's gamma correction.
            col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
            let ir = (255.99 * col.x) as u32;
            let ig = (255.99 * col.y) as u32;
            let ib = (255.99 * col.z) as u32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
