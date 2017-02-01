use vec3::Vec3;
use num_traits::Float;

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Ray<T: Float> {
    A: Vec3<T>,
    B: Vec3<T>,
}

impl<T: Float> Ray<T> {
    pub fn new(a: Vec3<T>, b: Vec3<T>) -> Ray<T> {
        Ray { A: a, B: b }
    }

    pub fn origin(&self) -> Vec3<T> {
        self.A
    }
    pub fn direction(&self) -> Vec3<T> {
        self.B
    }
    pub fn point_at_parameter(&self, t: T) -> Vec3<T> {
        self.A + self.B * t
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
