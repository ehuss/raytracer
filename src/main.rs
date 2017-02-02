extern crate raytracer;
extern crate rand;

use raytracer::*;

/// Get a random point on a unit sphere.
fn random_in_unit_sphere(rng: &mut Rng) -> Vec3<f64>
{
    // Simple algorithm, pick a random point in a unit cube (range -1..1).
    // Repeat if the point is outside the sphere.
    loop {
        let p = 2.0 * Vec3::new(rng.rand64(), rng.rand64(), rng.rand64()) - Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}


/// Get color for ray r cast into scene.
///
/// A miss into the background is a linear gradient from white to blue.
fn color<T: Hitable>(rng: &mut Rng, r: &Ray<f64>, world: &T) -> Vec3<f64>
{
    // Use 0.0001 to ignore hits very near zero (the ray should travel at
    // least some distance).
    if let Some(h) = world.hit(r, 0.0001, std::f64::MAX) {
        let target = h.p + h.normal + random_in_unit_sphere(rng);
        // Takes half the color from a random ray from the surface.
        return 0.5 * color(rng, &Ray::new(h.p, target-h.p), world);
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
    world.add_hitable(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    // Really huge sphere (covers bottom of screen).
    world.add_hitable(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));
    let mut rng = Rng::new();
    for j in (0..ny - 1).rev() {
        for i in 0..nx {
            let mut col = Vec3::<f64>::zero();
            for _ in 0..ns {
                let u = (i as f64 + rng.rand64()) / nx as f64;
                let v = (j as f64 + rng.rand64()) / ny as f64;
                let r = cam.get_ray(u, v);
                col += color(&mut rng, &r, &world);
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
