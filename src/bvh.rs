use hitable::*;
use aabb::*;
use util::*;
use ray::*;
use std::cmp::Ordering;

/// Dummy node is used in the case where the BVHNode has only 1 child (to
/// avoid having to Clone the Hitable or using Option or Enum nodes).
#[derive(Debug)]
struct DummyNode {
    bbox: AABB
}
impl Hitable for DummyNode {
    #![allow(unused)]
    fn hit(&self, r: &Ray<f64>, t_min: f64, t_max: f64) -> Option<HitRecord> { None }
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> { Some(self.bbox.clone()) }
}
impl fmt::Display for DummyNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DummyNode()")
    }
}


#[derive(Debug)]
pub struct BVHNode {
    left: Box<Hitable>,
    right: Box<Hitable>,
    bbox: AABB,
}

macro_rules! box_a_compare {
    ($f:ident, $a:ident) => {
        fn $f(a: &Box<Hitable>, b: &Box<Hitable>) -> Ordering {
            let box_left = a.bounding_box(0.0, 0.0).unwrap();
            let box_right = b.bounding_box(0.0, 0.0).unwrap();
            if box_left.min().$a - box_right.min().$a < 0.0 {
                return Ordering::Less;
            } else {
                return Ordering::Greater;
            }
        }
    }
}

box_a_compare!(box_x_compare, x);
box_a_compare!(box_y_compare, y);
box_a_compare!(box_z_compare, z);



impl BVHNode {
    pub fn new(rng: &mut Rng, mut l: Vec<Box<Hitable>>, time0: f64, time1: f64) -> BVHNode
    {
        assert_ne!(l.len(), 0);
        let axis = (3.0 * rng.rand64()) as u8;
        if axis == 0 {
            l.sort_by(box_x_compare);
        } else if axis == 1 {
            l.sort_by(box_y_compare);
        } else {
            l.sort_by(box_z_compare);
        }
        let left: Box<Hitable>;
        let right: Box<Hitable>;
        let llen = l.len();
        if llen == 1 {
            left = l.remove(0);
            right = Box::new(DummyNode{bbox: left.bounding_box(time0, time1).unwrap()});
        } else if llen == 2 {
            left = l.remove(0);
            right = l.remove(0);
        } else {
            let rest = l.split_off(llen/2);
            left = Box::new(BVHNode::new(rng, l, time0, time1));
            right = Box::new(BVHNode::new(rng, rest, time0, time1));
        }
        let box_left = left.bounding_box(time0, time1).unwrap();
        let box_right = right.bounding_box(time0, time1).unwrap();
        BVHNode{
            left: left,
            right: right,
            bbox: surrounding_box(&box_left, &box_right)


        }
    }
}

impl Hitable for BVHNode {
    fn hit(&self, r: &Ray<f64>, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if self.bbox.hit(r, t_min, t_max) {
            let hit_left = self.left.hit(r, t_min, t_max);
            let hit_right = self.right.hit(r, t_min, t_max);
            match (hit_left, hit_right) {
                (Some(left), Some(right)) => {
                    if left.t < right.t {
                        return Some(left);
                    } else {
                        return Some(right);
                    }
                },
                (Some(left), None) => { return Some(left); },
                (None, Some(right)) => { return Some(right); },
                _ => { return None; }
            }
        }
        return None;
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        Some(self.bbox.clone())
    }
}

