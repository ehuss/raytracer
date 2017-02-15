use vec3::Vec3;
use ray::Ray;
use material::Material;
use util::*;
use aabb::*;

/// A ray hit on a surface.
#[derive(Debug, Clone, new)]
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

/// Used for surfaces/objects that can be "hit" by a ray.
pub trait Hitable: fmt::Debug {
    /// Test for hit against surface.
    /// Returns None if no hit.
    fn hit(&self, rng: &mut Rng, r: &Ray<f64>, t_min: f64, t_max: f64) -> Option<HitRecord>;

    /// Generate a bounding box for this hitable object.
    ///
    /// t0/t1 is the time range, to accommodate moving objects (box should
    /// encompass the entire region the object occupies during that time).
    ///
    /// Returns None if there is no valid bounding box (like an infinite
    /// plane).
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;

    fn pdf_value(&self, rng: &mut Rng, o: &Vec3<f64>, v: &Vec3<f64>) -> f64 { 0. }
    fn random(&self, rng: &mut Rng, o: &Vec3<f64>) -> Vec3<f64> { Vec3::new(1., 0., 0.) }
}

#[derive(Debug, new)]
pub struct FlipNormals {
    hitable: Box<Hitable>
}

impl Hitable for FlipNormals {
    fn hit(&self, rng: &mut Rng, r: &Ray<f64>, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(mut h) = self.hitable.hit(rng, r, t_min, t_max) {
            h.normal = -h.normal;
            return Some(h);
        } else {
            return None;
        }
    }
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        self.hitable.bounding_box(t0, t1)
    }
}

#[derive(Debug, new)]
pub struct Translate {
    hitable: Box<Hitable>,
    offset: Vec3<f64>,
}

impl Hitable for Translate {
    fn hit(&self, rng: &mut Rng, r: &Ray<f64>, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new_time(r.origin() - self.offset, r.direction(), r.time());
        match self.hitable.hit(rng, &moved_r, t_min, t_max) {
            Some(mut h) => {
                h.p += self.offset;
                Some(h)
            },
            None => None,
        }
    }
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        match self.hitable.bounding_box(t0, t1) {
            Some(bb) => {
                Some(AABB::new(bb.min() + self.offset, bb.max() + self.offset))
            },
            None => None,
        }
    }
}

#[derive(Debug)]
pub struct RotateY {
    hitable: Box<Hitable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<AABB>,
}

impl RotateY {
    pub fn new(hitable: Box<Hitable>, angle: f64) -> RotateY {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = hitable.bounding_box(0., 1.).map(|bbox| {
            let mut min = Vec3::new(f64::MAX, f64::MAX, f64::MAX);
            let mut max = Vec3::new(-f64::MAX, -f64::MAX, -f64::MAX);
            for i in 0..2u8 {
                let fi = i as f64;
                for j in 0..2u8 {
                    let fj = j as f64;
                    for k in 0..2u8 {
                        let fk = k as f64;
                        let x = fi * bbox.max().x + (1.-fi)*bbox.min().x;
                        let y = fj * bbox.max().y + (1.-fj)*bbox.min().y;
                        let z = fk * bbox.max().z + (1.-fk)*bbox.min().z;
                        let newx = cos_theta*x + sin_theta*z;
                        let newz = -sin_theta*x + cos_theta*z;
                        let tester = Vec3::new(newx, y, newz);
                        for c in 0..3 {
                            if tester[c] > max[c] {
                                max[c] = tester[c];
                            }
                            if tester[c] < min[c] {
                                min[c] = tester[c];
                            }
                        }
                    }
                }
            }
            AABB::new(min, max)
        });
        RotateY {
            hitable: hitable,
            sin_theta: sin_theta,
            cos_theta: cos_theta,
            bbox: bbox
        }
    }
}

impl Hitable for RotateY {
    fn hit(&self, rng: &mut Rng, r: &Ray<f64>, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = r.origin().clone();
        let mut direction = r.direction().clone();
        origin[0] = self.cos_theta*r.origin()[0] - self.sin_theta*r.origin()[2];
        origin[2] = self.sin_theta*r.origin()[0] + self.cos_theta*r.origin()[2];
        direction[0] = self.cos_theta*r.direction()[0] - self.sin_theta*r.direction()[2];
        direction[2] = self.sin_theta*r.direction()[0] + self.cos_theta*r.direction()[2];
        let rotated_r = Ray::new_time(origin, direction, r.time());
        return self.hitable.hit(rng, &rotated_r, t_min, t_max).map(|mut rec| {
            let mut p = rec.p.clone();
            let mut normal = rec.normal.clone();
            p[0] = self.cos_theta*rec.p[0] + self.sin_theta*rec.p[2];
            p[2] = -self.sin_theta*rec.p[0] + self.cos_theta*rec.p[2];
            normal[0] = self.cos_theta*rec.normal[0] + self.sin_theta*rec.normal[2];
            normal[2] = -self.sin_theta*rec.normal[0] + self.cos_theta*rec.normal[2];
            rec.p = p;
            rec.normal = normal;
            rec
        });
    }
    #[allow(unused)]
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        self.bbox.clone()
    }
}


