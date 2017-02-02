use vec3::*;
use ray::*;

pub struct Camera {
    origin: Vec3<f64>,
    lower_left_corner: Vec3<f64>,
    horizontal: Vec3<f64>,
    vertical: Vec3<f64>,
}

impl Camera {
    /// Create a new camera.
    pub fn new() -> Camera {
        Camera {
            origin: Vec3::zero(),
            lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray<f64> {
        Ray::new(self.origin,
                 self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin)
    }
}
