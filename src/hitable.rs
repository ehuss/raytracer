use vec3::Vec3;
use ray::Ray;

/// A ray hit on a surface.
pub struct HitRecord {
    /// Point along ray.
    pub t: f64,
    /// Hit location.
    pub p: Vec3<f64>,
    /// Normal vector from surface.
    pub normal: Vec3<f64>,
}

impl HitRecord {
    pub fn new(t: f64, p: Vec3<f64>, normal: Vec3<f64>) -> HitRecord {
        HitRecord {
            t: t,
            p: p,
            normal: normal,
        }
    }
}

/// Used for surfaces/objects that can be "hit" by a ray.
pub trait Hitable {
    /// Test for hit against surface.
    /// Returns None if no hit.
    fn hit(&self, r: &Ray<f64>, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
