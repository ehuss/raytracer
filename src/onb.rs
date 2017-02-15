use vec3::*;

#[derive(Debug)]
pub struct Onb {
    axis: [Vec3<f64>; 3]
}

impl Onb {
    pub fn new() -> Onb {
        Onb {
            axis: [Vec3::zero(), Vec3::zero(), Vec3::zero()]
        }
    }
    pub fn new_from_w(n: &Vec3<f64>) -> Onb {
        let w = n.unit_vector();
        let a;
        if w.x.abs() > 0.9 {
            a = Vec3::new(0., 1., 0.);
        } else {
            a = Vec3::new(1., 0., 0.);
        }
        let v = cross(&w, &a).unit_vector();
        let u = cross(&w, &v);
        return Onb {axis: [u, v, w]};
    }

    pub fn u(&self) -> Vec3<f64> { self.axis[0] }
    pub fn v(&self) -> Vec3<f64> { self.axis[1] }
    pub fn w(&self) -> Vec3<f64> { self.axis[2] }
    pub fn local_scalar(&self, a: f64, b: f64, c: f64) -> Vec3<f64> {
        a*self.u() + b*self.v() + c*self.w()
    }
    pub fn local_vec(&self, a: &Vec3<f64>) -> Vec3<f64> {
        a.x*self.u() + a.y*self.v() + a.z*self.w()
    }
}
