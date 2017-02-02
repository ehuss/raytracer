use vec3::*;
use ray::*;
use std;

pub struct Camera {
    origin: Vec3<f64>,
    lower_left_corner: Vec3<f64>,
    horizontal: Vec3<f64>,
    vertical: Vec3<f64>,
}

impl Camera {
    /// Create a new camera.
    /// vfov is field of view vertically in degrees.
    pub fn new(lookfrom: Vec3<f64>, lookat: Vec3<f64>, vup: Vec3<f64>, vfov: f64, aspect: f64) -> Camera {
        let theta = vfov * std::f64::consts::PI / 180.0;
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;
        let w = (lookfrom - lookat).unit_vector();
        let u = cross(&vup, &w).unit_vector();
        let v = cross(&w, &u);

        Camera {
            origin: lookfrom,
            lower_left_corner: lookfrom - half_width*u - half_height*v - w,
            horizontal: 2.0*half_width*u,
            vertical: 2.0*half_height*v,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray<f64> {
        Ray::new(self.origin,
                 self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin)
    }
}
