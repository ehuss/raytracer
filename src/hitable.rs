use vec3::Vec3;
use ray::Ray;
use material::Material;
use util::*;
use aabb::*;

/// A ray hit on a surface.
#[derive(Debug)]
pub struct HitRecord {
    /// Point along ray.
    pub t: f64,
    /// Surface UV.
    pub u: f64,
    /// Surface UV.
    pub v: f64,
    /// Hit location.
    pub p: Vec3<f64>,
    /// Normal vector from surface.
    pub normal: Vec3<f64>,
    /// Material of the surface where it hit.
    pub material: Rc<Material>,
}

impl HitRecord {
    pub fn new(t: f64, u: f64, v: f64, p: Vec3<f64>, normal: Vec3<f64>, m: Rc<Material>) -> HitRecord {
        HitRecord {
            t: t,
            u: u,
            v: v,
            p: p,
            normal: normal,
            material: m,
        }
    }
}

/// Used for surfaces/objects that can be "hit" by a ray.
pub trait Hitable: fmt::Debug {
    /// Test for hit against surface.
    /// Returns None if no hit.
    fn hit(&self, r: &Ray<f64>, t_min: f64, t_max: f64) -> Option<HitRecord>;

    /// Generate a bounding box for this hitable object.
    ///
    /// t0/t1 is the time range, to accommodate moving objects (box should
    /// encompass the entire region the object occupies during that time).
    ///
    /// Returns None if there is no valid bounding box (like an infinite
    /// plane).
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;
}
