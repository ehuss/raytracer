use vec3::*;
use ray::Ray;
use aabb::*;
use hitable::*;
use hitable_list::*;
use material::*;
use util::*;
use aarect::*;

#[derive(Debug)]
pub struct HBox<'a> {
    pmin: Vec3<f64>,
    pmax: Vec3<f64>,
    hlist: HitableList<'a>,
}

impl<'a> HBox<'a> {
    pub fn new(p0: Vec3<f64>, p1: Vec3<f64>, material: Rc<Material>) -> HBox<'a> {
        let mut hlist = HitableList::new();
        hlist.add_hitable(XYRect::new(p0.x, p1.x, p0.y, p1.y, p1.z, material.clone()));
        hlist.add_hitable(FlipNormals::new(Box::new(XYRect::new(p0.x, p1.x, p0.y, p1.y, p0.z, material.clone()))));
        hlist.add_hitable(XZRect::new(p0.x, p1.x, p0.z, p1.z, p1.y, material.clone()));
        hlist.add_hitable(FlipNormals::new(Box::new(XZRect::new(p0.x, p1.x, p0.z, p1.z, p0.y, material.clone()))));
        hlist.add_hitable(YZRect::new(p0.y, p1.y, p0.z, p1.z, p1.x, material.clone()));
        hlist.add_hitable(FlipNormals::new(Box::new(YZRect::new(p0.y, p1.y, p0.z, p1.z, p0.x, material.clone()))));
        HBox {
            pmin: p0,
            pmax: p1,
            hlist: hlist
        }
    }
}

impl<'a> Hitable for HBox<'a> {
    fn hit(&self, r: &Ray<f64>, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.hlist.hit(r, t_min, t_max)
    }
    #[allow(unused)]
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        Some(AABB::new(self.pmin, self.pmax))
    }
}
