use hitable::*;
use ray::Ray;

pub struct HitableList<'a> {
    list: Vec<Box<Hitable + 'a>>,
}


impl<'a> HitableList<'a> {
    pub fn new() -> HitableList<'a> {
        HitableList { list: Vec::new() }
    }
    pub fn add_hitable<T: Hitable + 'a>(&mut self, h: T) {
        self.list.push(Box::new(h));
    }
}

impl<'a> Hitable for HitableList<'a> {
    fn hit(&self, r: &Ray<f64>, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut result: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for h in &self.list {
            if let Some(hr) = h.hit(r, t_min, closest_so_far) {
                closest_so_far = hr.t;
                result = Some(hr);
            }
        }
        return result;
    }
}
