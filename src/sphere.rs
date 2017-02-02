use vec3::*;
use ray::Ray;
use hitable::*;
use material::*;
use util::*;

pub struct Sphere {
    center: Vec3<f64>,
    radius: f64,
    material: Rc<Material>,
}

impl Sphere {
    pub fn new(cen: Vec3<f64>, r: f64, material: Rc<Material>) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
            material: material
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray<f64>, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = dot(&r.direction(), &r.direction());
        let b = dot(&oc, &r.direction());
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                let h = HitRecord::new(temp, p, (p - self.center) / self.radius, self.material.clone());
                return Some(h);
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                let h = HitRecord::new(temp, p, (p - self.center) / self.radius, self.material.clone());
                return Some(h);
            }
        }
        return None;
    }
}
