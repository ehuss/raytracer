use vec3::*;
use ray::*;
use util::*;

// ffmin/ffmax are faster because they do not worry about NaN and other issues.

#[inline(always)]
fn ffmin(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

#[inline(always)]
fn ffmax(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

#[derive(Clone, Debug)]
pub struct AABB {
    _min: Vec3<f64>,
    _max: Vec3<f64>,
}

impl AABB {
    pub fn new(a: Vec3<f64>, b: Vec3<f64>) -> AABB {
        AABB {
            _min: a,
            _max: b,
        }
    }
    #[inline(always)]
    pub fn min(&self) -> Vec3<f64> { self._min }
    #[inline(always)]
    pub fn max(&self) -> Vec3<f64> { self._max }

    /// Check if the given ray hits the bounding box.
    pub fn hit(&self, r: &Ray<f64>, tmin: f64, tmax: f64) -> bool {
        for a in 0..3 {
            let t0 = ffmin((self._min[a] - r.origin()[a]) / r.direction()[a],
                           (self._max[a] - r.origin()[a]) / r.direction()[a]);
            let t1 = ffmax((self._min[a] - r.origin()[a]) / r.direction()[a],
                           (self._max[a] - r.origin()[a]) / r.direction()[a]);
            let tmin = ffmax(t0, tmin);
            let tmax = ffmin(t1, tmax);
            if tmax <= tmin {
                return false;
            }
        }
        return true;
    }
}

impl fmt::Display for AABB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AABB({}, {})", self._min, self._max)
    }
}


/// Compute the AABB that surrounds the two given boxes.
pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let small = Vec3::new(box0.min().x.min(box1.min().x),
                          box0.min().y.min(box1.min().y),
                          box0.min().z.min(box1.min().z));
    let big = Vec3::new(box0.max().x.max(box1.max().x),
                        box0.max().y.max(box1.max().y),
                        box0.max().z.max(box1.max().z));
    return AABB::new(small, big);
}
