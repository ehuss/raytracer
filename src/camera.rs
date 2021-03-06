use vec3::*;
use ray::*;
use util::*;

fn random_in_unit_disk(rng: &mut Rng) -> Vec3<f64> {
    loop {
        let p = 2.0 * Vec3::new(rng.rand64(), rng.rand64(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        if dot(&p, &p) < 1.0 {
            return p;
        }
    }
}

#[derive(Debug)]
pub struct Camera {
    /// Position of the camera.
    origin: Vec3<f64>,
    /// Lower-left corner of the focus plane in world coordinates.
    lower_left_corner: Vec3<f64>,
    /// Vector going horizontally (left-to-right) across the focus plane.
    horizontal: Vec3<f64>,
    /// Vector going vertically (bottom-to-top) across the focus plane.
    vertical: Vec3<f64>,
    /// ONB for the camera.
    u: Vec3<f64>,
    v: Vec3<f64>,
    w: Vec3<f64>,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

impl Camera {
    /// Create a new camera.
    /// vfov is field of view vertically in degrees.
    pub fn new(lookfrom: Vec3<f64>,
               lookat: Vec3<f64>,
               vup: Vec3<f64>,
               vfov: f64,
               aspect: f64,
               aperture: f64,
               focus_dist: f64,
               t0: f64,
               t1: f64)
               -> Camera {
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (lookfrom - lookat).unit_vector();
        let u = cross(&vup, &w).unit_vector();
        let v = cross(&w, &u);
        let lower_left_corner = lookfrom - half_width * focus_dist * u -
                                half_height * focus_dist * v -
                                focus_dist * w;
        let horizontal = 2.0 * half_width * focus_dist * u;
        let vertical = 2.0 * half_height * focus_dist * v;

        Camera {
            origin: lookfrom,
            lower_left_corner: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical,
            u: u,
            v: v,
            w: w,
            lens_radius: aperture / 2.0,
            time0: t0,
            time1: t1,
        }
    }

    pub fn get_ray(&self, rng: &mut Rng, s: f64, t: f64) -> Ray<f64> {
        let rd = self.lens_radius * random_in_unit_disk(rng);
        let offset = self.u * rd.x + self.v * rd.y;
        // Emit the ray at some random time while the shutter is open.
        let time = self.time0 + rng.rand64() * (self.time1 - self.time0);
        Ray::new_time(self.origin + offset,
                      self.lower_left_corner + s * self.horizontal + t * self.vertical -
                      self.origin - offset,
                      time)
    }
}
