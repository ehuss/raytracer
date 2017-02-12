use vec3::*;
use ray::Ray;
use aabb::*;
use hitable::*;
use material::*;
use util::*;

#[derive(Debug, new)]
pub struct XYRect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    material: Rc<Material>
}


impl Hitable for XYRect {
    fn hit(&self, _: &mut Rng, r: &Ray<f64>, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k-r.origin().z) / r.direction().z;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin().x + t*r.direction().x;
        let y = r.origin().y + t*r.direction().y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let u = (x-self.x0)/(self.x1-self.x0);
        let v = (y-self.y0)/(self.y1-self.y0);
        let normal = Vec3::new(0., 0., 1.);
        return Some(HitRecord::new(t, u, v, r.point_at_parameter(t), normal, self.material.clone()));
    }

    #[allow(unused)]
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        Some(AABB::new(Vec3::new(self.x0, self.y0, self.k-0.0001),
                       Vec3::new(self.x1, self.y1, self.k+0.0001)))
    }
}

#[derive(Debug, new)]
pub struct XZRect {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: Rc<Material>
}


impl Hitable for XZRect {
    fn hit(&self, _: &mut Rng, r: &Ray<f64>, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k-r.origin().y) / r.direction().y;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin().x + t*r.direction().x;
        let z = r.origin().z + t*r.direction().z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let u = (x-self.x0)/(self.x1-self.x0);
        let v = (z-self.z0)/(self.z1-self.z0);
        let normal = Vec3::new(0., 1., 0.);
        return Some(HitRecord::new(t, u, v, r.point_at_parameter(t), normal, self.material.clone()));
    }

    #[allow(unused)]
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        Some(AABB::new(Vec3::new(self.x0, self.k-0.0001, self.z0),
                       Vec3::new(self.x1, self.k+0.0001, self.z1)))
    }
}

#[derive(Debug, new)]
pub struct YZRect {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: Rc<Material>
}


impl Hitable for YZRect {
    fn hit(&self, _: &mut Rng, r: &Ray<f64>, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k-r.origin().x) / r.direction().x;
        if t < t_min || t > t_max {
            return None;
        }
        let y = r.origin().y + t*r.direction().y;
        let z = r.origin().z + t*r.direction().z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let u = (y-self.y0)/(self.y1-self.y0);
        let v = (z-self.z0)/(self.z1-self.z0);
        let normal = Vec3::new(1., 0., 0.);
        return Some(HitRecord::new(t, u, v, r.point_at_parameter(t), normal, self.material.clone()));
    }

    #[allow(unused)]
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        Some(AABB::new(Vec3::new(self.k-0.0001, self.y0, self.z0),
                       Vec3::new(self.k+0.0001, self.y1, self.z1)))
    }
}
