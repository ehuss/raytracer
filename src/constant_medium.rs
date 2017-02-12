use hitable::*;
use material::*;
use std::f64;
use util::*;
use texture::*;
use ray::*;
use vec3::*;
use aabb::*;

#[derive(Debug)]
pub struct ConstantMedium {
    boundary: Box<Hitable>,
    density: f64,
    phase_function: Rc<Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Box<Hitable>, density: f64, albedo: Box<Texture>) -> ConstantMedium {
        ConstantMedium {
            boundary: boundary,
            density: density,
            phase_function: Rc::new(Isotropic::new(albedo))
        }
    }
}

impl Hitable for ConstantMedium {
    fn hit(&self, rng: &mut Rng, r: &Ray<f64>, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(mut rec1) = self.boundary.hit(rng, r, -f64::MAX, f64::MAX) {
            if let Some(mut rec2) = self.boundary.hit(rng, r, rec1.t+0.0001, f64::MAX) {
                if rec1.t < t_min {
                    rec1.t = t_min;
                }
                if rec2.t > t_max {
                    rec2.t = t_max;
                }
                if rec1.t >= rec2.t {
                    return None;
                }
                if rec1.t < 0. {
                    rec1.t = 0.;
                }
                let distance_inside_boundary = (rec2.t - rec1.t)*r.direction().length();
                let hit_distance = -(1./self.density)*rng.rand64().ln();
                if hit_distance < distance_inside_boundary {
                    let t = rec1.t + hit_distance / r.direction().length();
                    let p = r.point_at_parameter(t);
                    let normal = Vec3::new(1., 0., 0.); // arbitrary
                    return Some(HitRecord::new(t, 0., 0., p, normal, self.phase_function.clone()));
                }
            }
        }
        return None;
    }
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        self.boundary.bounding_box(t0, t1)
    }
}
