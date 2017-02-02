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

fn random_scene<'a>(rng: &mut Rng) -> HitableList<'a> {
    let mut list = HitableList::new();
    list.add_hitable(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)))));
    // XXX: Not sure why explicit u8 is required here to cast to f64.
    for a in -11..11i8 {
        for b in -11..11i8 {
            let choose_mat = rng.rand64();
            let center = Vec3::new(a as f64 + 0.9 * rng.rand64(), 0.2, b as f64+0.9*rng.rand64());
            if (center-Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 { //diffuse
                    list.add_hitable(
                        Sphere::new(center, 0.2, Rc::new(Lambertian::new(
                            Vec3::new(rng.rand64()*rng.rand64(), rng.rand64()*rng.rand64(), rng.rand64()*rng.rand64())))));
                } else if choose_mat < 0.95 { //metal
                    list.add_hitable(
                        Sphere::new(center, 0.2, Rc::new(Metal::new(
                            Vec3::new(0.5*(1.0 + rng.rand64()),
                                      0.5*(1.0 + rng.rand64()),
                                      0.5*(1.0 + rng.rand64())), 0.5*rng.rand64()))));
                } else { //glass
                    list.add_hitable(Sphere::new(center, 0.2, Rc::new(Dielectric::new(1.5))));
                }
            }
        }
    }
    list.add_hitable(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Rc::new(Dielectric::new(1.5))));
    list.add_hitable(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Rc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)))));
    list.add_hitable(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0))));
    return list;
}


// y-up
// into screen is -z
// Traverse from lower-left corner (-2, -1) to upper-right +(+2,+2)

fn main() {
    let nx = 400;
    let ny = 200;
    let ns = 100;
    println!("P3\n{} {}\n255", nx, ny);
    let mut rng = Rng::new();
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;//(lookfrom-lookat).length();
    let aperture = 0.1;
    let cam = Camera::new(lookfrom, lookat, Vec3::new(0.0, 1.0, 0.0), 20.0, nx as f64/ny as f64, aperture, dist_to_focus);
    let world = random_scene(&mut rng);
    for j in (0..ny - 1).rev() {
        for i in 0..nx {
            let mut col = Vec3::<f64>::zero();
            for _ in 0..ns {
                let u = (i as f64 + rng.rand64()) / nx as f64;
                let v = (j as f64 + rng.rand64()) / ny as f64;
                let r = cam.get_ray(&mut rng, u, v);
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
