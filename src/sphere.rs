use vec3::*;
use ray::Ray;
use hitable::*;
use material::*;
use util::*;
use aabb::*;
use std::f64::consts::*;

#[derive(Debug)]
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
            material: material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, _: &mut Rng, r: &Ray<f64>, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = dot(&r.direction(), &r.direction());
        let b = dot(&oc, &r.direction());
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                let (u, v) = get_sphere_uv(&((p-self.center)/self.radius));
                let h = HitRecord::new(temp,
                                       u, v,
                                       p,
                                       (p - self.center) / self.radius,
                                       self.material.clone());
                return Some(h);
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                let (u, v) = get_sphere_uv(&((p-self.center)/self.radius));
                let h = HitRecord::new(temp,
                                       u, v,
                                       p,
                                       (p - self.center) / self.radius,
                                       self.material.clone());
                return Some(h);
            }
        }
        return None;
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(AABB::new(self.center - Vec3::new(self.radius, self.radius, self.radius),
                       self.center + Vec3::new(self.radius, self.radius, self.radius)))
    }
}

fn get_sphere_uv(p: &Vec3<f64>) -> (f64, f64) {
    let phi = p.z.atan2(p.x);
    let theta = p.y.asin();
    let u = 1.-(phi + PI) / (2.*PI);
    let v = (theta + PI/2.) / PI;
    return (u,v);
}
