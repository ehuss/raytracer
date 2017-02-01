extern crate raytracer;

use raytracer::*;

fn hit_sphere(center: Vec3<f64>, radius: f64, r: &Ray<f64>) -> bool {
    // Equation to test if ray hits sphere:
    //    ray = p(t) = A + t*B
    //    dot((p(t)-c), (p(t)-c)) = radius*radius
    // Replace and expand:
    //    t*t*dot(B, B) + 2*t*dot(B, A-C) + dot(A-C, A-C) - R*R = 0
    // Solve for t.
    //    If the sqrt is >0, 2 solutions (hit both sides of sphere).
    //                 ==0, one solution, tangent to the surface.
    //                 <0, no solutions (didn't hit)
    let oc = r.origin() - center;
    let a = dot(&r.direction(), &r.direction());
    let b = 2.0 * dot(&oc, &r.direction());
    let c = dot(&oc, &oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    discriminant > 0.0
}

/// Get color for ray r cast into scene.
///
/// Linear blend blue-to-while vertically.
/// Red sphere in the middle.
fn color(r: &Ray<f64>) -> Vec3<f64> {
    if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, &r) {
        return Vec3::new(1.0, 0.0, 0.0);  // Red
    }
    // y will transition from -1 to +1
    let unit_direction = r.direction().unit_vector();
    // translate y to range 0..1
    let t = 0.5 * (unit_direction.y + 1.0);
    // Linear blend white to blue.
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
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
    let origin = Vec3::new(0.0, 0.0, 0.0);
    for j in (0..ny - 1).rev() {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;
            let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);
            let col = color(&r);
            let ir = (255.99 * col.x) as u32;
            let ig = (255.99 * col.y) as u32;
            let ib = (255.99 * col.z) as u32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
