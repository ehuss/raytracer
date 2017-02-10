use vec3::Vec3;
use num_traits::Float;
use util::*;

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Ray<T: Float> {
    A: Vec3<T>,
    B: Vec3<T>,
    /// Timestamp of when this ray was fired.
    t: f64,
}

impl<T: Float> Ray<T> {
    pub fn new(a: Vec3<T>, b: Vec3<T>) -> Ray<T> {
        Ray {
            A: a,
            B: b,
            t: 0.0,
        }
    }
    pub fn new_time(a: Vec3<T>, b: Vec3<T>, time: f64) -> Ray<T> {
        Ray {
            A: a,
            B: b,
            t: time,
        }
    }
    #[inline(always)]
    pub fn origin(&self) -> Vec3<T> {
        self.A
    }
    #[inline(always)]
    pub fn direction(&self) -> Vec3<T> {
        self.B
    }
    #[inline(always)]
    pub fn time(&self) -> f64 {
        self.t
    }
    pub fn point_at_parameter(&self, t: T) -> Vec3<T> {
        self.A + self.B * t
    }
}

impl <T: Float + fmt::Display> fmt::Display for Ray<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p = f.precision().unwrap_or(3);
        write!(f, "Ray({:.*}, {:.*}, {:.*})", p, self.A, p, self.B, p, self.t)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use vec3::Vec3;

    #[test]
    fn test_ray() {
        let a = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0));
        assert_eq!(a.origin(), Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(a.direction(), Vec3::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn test_p_at_p() {
        let a = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0));
        assert_eq!(a.point_at_parameter(0.5), Vec3::new(3.0, 4.5, 6.0));
    }
}
