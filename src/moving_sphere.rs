use vec3::*;
use ray::Ray;
use hitable::*;
use material::*;
use util::*;

pub struct MovingSphere {
    /// Position at time0.
    center0: Vec3<f64>,
    /// Position at time1.
    center1: Vec3<f64>,
    /// Timestamp where it is positioned at center0.
    time0: f64,
    /// Timestamp where it is positioned at center1.
    time1: f64,
    radius: f64,
    material: Rc<Material>,
}

impl MovingSphere {
    pub fn new(cen0: Vec3<f64>,
               cen1: Vec3<f64>,
               t0: f64,
               t1: f64,
               r: f64,
               material: Rc<Material>)
               -> MovingSphere {
        MovingSphere {
            center0: cen0,
            center1: cen1,
            time0: t0,
            time1: t1,
            radius: r,
            material: material,
        }
    }

    /// Compute the sphere's position at the given time.
    fn center(&self, time: f64) -> Vec3<f64> {
        self.center0 +
        ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, r: &Ray<f64>, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center(r.time());
        let a = dot(&r.direction(), &r.direction());
        let b = dot(&oc, &r.direction());
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                let h = HitRecord::new(temp,
                                       p,
                                       (p - self.center(r.time())) / self.radius,
                                       self.material.clone());
                return Some(h);
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                let h = HitRecord::new(temp,
                                       p,
                                       (p - self.center(r.time())) / self.radius,
                                       self.material.clone());
                return Some(h);
            }
        }
        return None;
    }
}
