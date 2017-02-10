use std::mem;
use std::ops::*;
use util::*;
use num_traits::Float;
pub use num_traits::Zero;

// TODO:
// - Hash?
//
// ○ Index, IndexMut
//
// • std::convert:
// From/Into, AsRef/AsMut
// [S;3]
// (S, S, S)
//

pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Vec3<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float> Vec3<T> {
    #[inline]
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x: x, y: y, z: z }
    }
    pub fn length(&self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn squared_length(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    // AKA normalize
    pub fn make_unit_vector(&mut self) {
        let k = self.length().recip();
        self.x = self.x * k;
        self.y = self.y * k;
        self.z = self.z * k;
    }
    pub fn unit_vector(&self) -> Vec3<T> {
        self / self.length()
    }
}

impl<T: Float> From<T> for Vec3<T> {
    fn from(f: T) -> Vec3<T> {
        Vec3::new(f, f, f)
    }
}

impl<T: Float> Zero for Vec3<T> {
    fn zero() -> Vec3<T> {
        Vec3::from(T::zero())
    }
    fn is_zero(&self) -> bool {
        *self == Vec3::zero()
    }
}

impl<T: Float> Neg for Vec3<T> {
    type Output = Vec3<T>;

    fn neg(self) -> Vec3<T> {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}
impl<'a, T: Float> Neg for &'a Vec3<T> {
    type Output = Vec3<T>;

    fn neg(self) -> Vec3<T> {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl<T: Float> Add for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl<'a, 'b, T: Float> Add<&'a Vec3<T>> for &'b Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: &'a Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: Float> Sub for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl<'a, T: Float> Sub<Vec3<T>> for &'a Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<'a, 'b, T: Float> Sub<&'a Vec3<T>> for &'b Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: &'a Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: Float> Mul<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: T) -> Vec3<T> {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl<'a, T: Float> Mul<T> for &'a Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: T) -> Vec3<T> {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl<T: Float> Mul<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}
impl Mul<Vec3<f64>> for f64 {
    type Output = Vec3<f64>;

    fn mul(self, rhs: Vec3<f64>) -> Vec3<f64> {
        Vec3::new(rhs.x * self, rhs.y * self, rhs.z * self)
    }
}
impl<'a> Mul<&'a Vec3<f64>> for f64 {
    type Output = Vec3<f64>;

    fn mul(self, rhs: &Vec3<f64>) -> Vec3<f64> {
        Vec3::new(rhs.x * self, rhs.y * self, rhs.z * self)
    }
}

impl<T: Float> Div<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn div(self, rhs: T) -> Vec3<T> {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}
impl<'a, T: Float> Div<T> for &'a Vec3<T> {
    type Output = Vec3<T>;

    fn div(self, rhs: T) -> Vec3<T> {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl<T: Float + AddAssign<T>> AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Vec3<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Float + SubAssign<T>> SubAssign for Vec3<T> {
    fn sub_assign(&mut self, rhs: Vec3<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: Float + MulAssign<T>> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
impl<T: Float + MulAssign<T>> MulAssign<Vec3<T>> for Vec3<T> {
    fn mul_assign(&mut self, rhs: Vec3<T>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl<T: Float + DivAssign<T>> DivAssign<T> for Vec3<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}
impl<T: Float + DivAssign<T>> DivAssign<Vec3<T>> for Vec3<T> {
    fn div_assign(&mut self, rhs: Vec3<T>) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl<N, T: Float> Index<N> for Vec3<T> where [T]: Index<N> {
    type Output = <[T] as Index<N>>::Output;
    #[inline(always)]
    fn index(&self, i: N) -> &<[T] as Index<N>>::Output {
        &self.as_ref()[i]
    }
}

impl <T: Float> AsRef<[T; 3]> for Vec3<T> {
    #[inline(always)]
    fn as_ref(&self) -> &[T; 3] { unsafe { mem::transmute(self) } }
}

impl <T: Float + fmt::Display> fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p = f.precision().unwrap_or(3);
        write!(f, "Vec3({:.*}, {:.*}, {:.*})", p, self.x, p, self.y, p, self.z)
    }
}


pub fn dot<T: Float>(v1: &Vec3<T>, v2: &Vec3<T>) -> T {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn cross<T: Float>(v1: &Vec3<T>, v2: &Vec3<T>) -> Vec3<T> {
    Vec3::new((v1.y * v2.z - v1.z * v2.y),
              (-(v1.x * v2.z - v1.z * v2.x)),
              (v1.x * v2.y - v1.y * v2.x))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_math() {
        // Neg
        let a: Vec3<f64> = Default::default();
        let b = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(-&b, Vec3::new(-1.0, -2.0, -3.0));
        assert_eq!(-b, Vec3::new(-1.0, -2.0, -3.0));

        // Zero
        assert_eq!(Vec3::zero(), Vec3::new(0.0, 0.0, 0.0));

        // Add
        let b = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(a + b, Vec3::new(1.0, 2.0, 3.0));

        // Add &
        let a: Vec3<f64> = Default::default();
        let b = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(&a + &b, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(&b + &b, Vec3::new(2.0, 4.0, 6.0));

        // AddAssign
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        a += b;
        assert_eq!(a, Vec3::new(5.0, 7.0, 9.0));

        // Sub
        let a = Vec3::new(1.0, 1.0, 1.0);
        let b = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(b - a, Vec3::new(0.0, 1.0, 2.0));

        // Sub &
        let a = Vec3::new(1.0, 1.0, 1.0);
        let b = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(&b - &a, Vec3::new(0.0, 1.0, 2.0));
        assert_eq!(&b - &b, Vec3::new(0.0, 0.0, 0.0));

        // SubAssign
        let mut a = Vec3::new(4.0, 5.0, 6.0);
        let b = Vec3::new(1.0, 2.0, 3.0);
        a -= b;
        assert_eq!(a, Vec3::new(3.0, 3.0, 3.0));

        // Mul
        let a = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(a * 4.0, Vec3::new(4.0, 8.0, 12.0));
        // Mul &
        let a = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(&a * 4.0, Vec3::new(4.0, 8.0, 12.0));

        // MulAssign
        let mut a = Vec3::new(4.0, 5.0, 6.0);
        a *= 3.0;
        assert_eq!(a, Vec3::new(12.0, 15.0, 18.0));
        a /= Vec3::new(2.0, 5.0, 6.0);
        assert_eq!(a, Vec3::new(6.0, 3.0, 3.0));


        // Div
        let a = Vec3::new(5.0, 10.0, 15.0);
        assert_eq!(a / 5.0, Vec3::new(1.0, 2.0, 3.0));
        // Div &
        let a = Vec3::new(5.0, 10.0, 15.0);
        assert_eq!(&a / 5.0, Vec3::new(1.0, 2.0, 3.0));

        // DivAssign
        let mut a = Vec3::new(5.0, 10.0, 15.0);
        a /= 5.0;
        assert_eq!(a, Vec3::new(1.0, 2.0, 3.0));
        let mut a = Vec3::new(5.0, 10.0, 15.0);
        a /= Vec3::new(5.0, 2.0, 3.0);
        assert_eq!(a, Vec3::new(1.0, 5.0, 5.0));
    }

    #[test]
    fn test_dot() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(dot(&a, &b), 32.0);
    }

    #[test]
    fn test_cross() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(cross(&a, &b), Vec3::new(-3.0, 6.0, -3.0));
    }

    #[test]
    fn test_unit_vector() {
        let a = Vec3::new(2.0, 3.0, 6.0);
        assert_eq!(a.unit_vector(), Vec3::new(2.0 / 7.0, 3.0 / 7.0, 6.0 / 7.0));
        let mut a = Vec3::new(2.0, 3.0, 6.0);
        a.make_unit_vector();
        assert_eq!(a, Vec3::new(2.0 / 7.0, 3.0 / 7.0, 6.0 / 7.0));
    }

}
