use hitable::*;
use ray::Ray;
use aabb::*;
use util::*;
use vec3::*;

#[derive(Debug)]
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
    fn hit(&self, rng: &mut Rng, r: &Ray<f64>, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut result: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for h in &self.list {
            if let Some(hr) = h.hit(rng, r, t_min, closest_so_far) {
                closest_so_far = hr.t;
                result = Some(hr);
            }
        }
        return result;
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        if self.list.len() == 0 {
            return None;
        }
        let bb1 = self.list[0].bounding_box(t0, t1);
        if let Some(bb1) = bb1 {
            let mut bb = bb1;
            for h in self.list.iter().skip(1) {
                if let Some(temp_box) = h.bounding_box(t0, t1) {
                    bb = surrounding_box(&bb, &temp_box);
                } else {
                    // One of our items is infinite.
                    return None;
                }
            }
            return Some(bb);
        } else {
            return None;
        }
    }

    fn pdf_value(&self, rng: &mut Rng, o: &Vec3<f64>, v: &Vec3<f64>) -> f64 {
        let weight = 1./self.list.len() as f64;
        let mut sum = 0.;
        for i in 0..self.list.len() {
            sum += weight * self.list[i].pdf_value(rng, o, v);
        }
        return sum;
    }

    fn random(&self, rng: &mut Rng, o: &Vec3<f64>) -> Vec3<f64> {
        let index = (rng.rand64() * self.list.len() as f64) as usize;
        return self.list[index].random(rng, o);
    }

}

