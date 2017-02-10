use hitable::*;
use aabb::*;
use util::*;
use ray::*;
use vec3::*;
use std::cmp::Ordering;
use std::f64;


/*
Bounding Volume Hierarchy using AABB.

Surface-Area-Heuristic (SAH)
    When examining the bounding box to split, cut along the longest axis.
    See http://psgraphics.blogspot.com/2016/03/a-simple-sah-bvh-build.html

Consider trying a stackless algorithm:
http://dcgi.felk.cvut.cz/publications/2011/hapala-sccg-esta

Other things of interest:
https://github.com/Twinklebear/tray_rust/blob/master/src/geometry/bvh.rs
https://github.com/svenstaro/bvh/blob/master/src/bvh.rs
https://github.com/sondrele/rust-raytracer/blob/master/src/scene/bvh.rs
http://www.sci.utah.edu/~wald/Publications/2007/ParallelBVHBuild/fastbuild.pdf
http://cseweb.ucsd.edu/~ravir/274/15/papers/drst.pdf
https://github.com/szellmann/visionaray/wiki/Acceleration-data-structures-and-traversal


*/

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

// Comparison function used for sorting elements along the longest axis.
macro_rules! box_a_compare {
    ($f:ident, $a:ident) => {
        fn $f(a: &Box<Hitable>, b: &Box<Hitable>) -> Ordering {
            // XXX: Should we pass in the time?
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
        let n = l.len();
        assert_ne!(n, 0);

        // Build a bounding box around all the elements.
        let main_box = l.iter().fold(AABB::zero(), |bbox, ref x| {
            let xbbox = x.bounding_box(time0, time1).unwrap();
            surrounding_box(&xbbox, &bbox)
        });

        // Sort the elements across the longest axis of the bounding box.
        let axis = main_box.longest_axis();
        match axis {
            Axis::X => l.sort_by(box_x_compare),
            Axis::Y => l.sort_by(box_y_compare),
            Axis::Z => l.sort_by(box_z_compare),
        }

        // Cumulatively build the surface area of the bounding boxes.
        let boxes: Vec<AABB> = l.iter().map(|h| h.bounding_box(time0, time1).unwrap()).collect();
        let mut left_area = Vec::with_capacity(n);
        let mut right_area = Vec::with_capacity(n);
        left_area.push(boxes[0].area());
        let mut left_box = boxes[0].clone();
        for i in 1..n-1 {
            left_box = surrounding_box(&left_box, &boxes[i]);
            left_area.push(left_box.area());
        }
        // Technically, left_area[n-1] == 0, but it is not needed.

        // Cumulatively build the surface area in reverse order.
        // XXX: Is there a way to build this without needing to manually reverse at the end?
        right_area.push(boxes[n-1].area());
        let mut right_box = boxes[n-1].clone();
        for i in (1..n-1).rev() {
            right_box = surrounding_box(&right_box, &boxes[i]);
            right_area.push(right_box.area());
        }
        right_area.push(0.0);
        right_area.reverse();

        // Find the index where to split the left/right trees. This is done by
        // finding the minimum of the number_of_children*surface_area.
        let mut min_sah = f64::MAX;
        let mut min_sah_idx = 0;
        for i in 0..n-1 {
            let sah = i as f64*left_area[i] + (n-i-1) as f64*right_area[i+1];
            if sah < min_sah {
                min_sah_idx = i;
                min_sah = sah;
            }
        }

        // Build the left tree.
        let mut rest = l.split_off(min_sah_idx+1);
        let left;
        if min_sah_idx == 0 {
            left = l.remove(0);
        } else {
            left = Box::new(BVHNode::new(rng, l, time0, time1));
        }
        // Build the right tree.
        let right;
        if rest.len() == 1 {
            right = rest.remove(0);
        } else {
            right = Box::new(BVHNode::new(rng, rest, time0, time1));
        }
        BVHNode{
            left: left,
            right: right,
            bbox: main_box,
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

