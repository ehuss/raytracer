use vec3::*;
use util::*;
use onb::*;
use hitable::*;

pub fn random_cosine_direction(rng: &mut Rng) -> Vec3<f64> {
    let r1 = rng.rand64();
    let r2 = rng.rand64();
    let z = (1.-r2).sqrt();
    let phi = 2.*PI*r1;
    let x = phi.cos()*2.*r2.sqrt();
    let y = phi.sin()*2.*r2.sqrt();
    return Vec3::new(x,y,z);
}

#[inline]
pub fn random_to_sphere(rng: &mut Rng, radius: f64, distance_squared: f64) -> Vec3<f64> {
    let r1 = rng.rand64();
    let r2 = rng.rand64();
    let z = 1. + r2*((1.-radius*radius/distance_squared).sqrt() - 1.);
    let phi = 2.*PI*r1;
    let x = phi.cos() * (1.-z*z).sqrt();
    let y = phi.sin() * (1.-z*z).sqrt();
    return Vec3::new(x, y, z);
}

pub trait Pdf: fmt::Debug {
    fn value(&self, rng: &mut Rng, direction: &Vec3<f64>) -> f64;
    fn generate(&self, rng: &mut Rng) -> Vec3<f64>;
}

#[derive(Debug)]
pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    pub fn new(w: &Vec3<f64>) -> CosinePdf {
        CosinePdf {
            uvw: Onb::new_from_w(w)
        }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, rng: &mut Rng, direction: &Vec3<f64>) -> f64 {
        let cosine = dot(&direction.unit_vector(), &self.uvw.w());
        if cosine > 0. {
            return cosine/PI;
        } else {
            return 0.
        }
    }

    fn generate(&self, rng: &mut Rng) -> Vec3<f64> {
        return self.uvw.local_vec(&random_cosine_direction(rng));
    }
}

#[derive(Debug, new)]
pub struct HitablePdf<'a> {
    o: Vec3<f64>,
    hitable: &'a Hitable,
}

impl<'a> Pdf for HitablePdf<'a> {
    fn value(&self, rng: &mut Rng, direction: &Vec3<f64>) -> f64 {
        return self.hitable.pdf_value(rng, &self.o, direction);
    }
    fn generate(&self, rng: &mut Rng) -> Vec3<f64> {
        return self.hitable.random(rng, &self.o);
    }
}

#[derive(Debug, new)]
pub struct MixturePdf<'a, 'b> {
    pdf0: &'a Pdf,
    pdf1: &'b Pdf,
}

impl<'a, 'b> Pdf for MixturePdf<'a, 'b> {
    fn value(&self, rng: &mut Rng, direction: &Vec3<f64>) -> f64 {
        return 0.5 * self.pdf0.value(rng, direction) + 0.5 * self.pdf1.value(rng, direction);
    }
    fn generate(&self, rng: &mut Rng) -> Vec3<f64> {
        if rng.rand64() < 0.5 {
            return self.pdf0.generate(rng);
        } else {
            return self.pdf1.generate(rng);
        }
    }
}
