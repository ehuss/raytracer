use vec3::*;
use std::fmt;
use perlin::*;

pub trait Texture: fmt::Debug {
    fn value(&self, u: f64, v: f64, p: &Vec3<f64>) -> Vec3<f64>;
}

#[derive(Debug)]
pub struct ConstantTexture {
    color: Vec3<f64>,
}

impl ConstantTexture {
    pub fn new(c: Vec3<f64>) -> ConstantTexture {
        ConstantTexture { color: c }
    }
}

impl Texture for ConstantTexture {
    #[allow(unused_variables)]
    fn value(&self, u: f64, v: f64, p: &Vec3<f64>) -> Vec3<f64> {
        self.color
    }
}

#[derive(Debug)]
pub struct CheckerTexture {
    odd: Box<Texture>,
    even: Box<Texture>,
}

impl CheckerTexture {
    pub fn new(t0: Box<Texture>, t1: Box<Texture>) -> CheckerTexture {
        CheckerTexture {
            odd: t0,
            even: t1,
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3<f64>) -> Vec3<f64> {
        let sines = (10. * p.x).sin() * (10. * p.y).sin() * (10. * p.z).sin();
        if sines < 0. {
            return self.odd.value(u, v, p);
        } else {
            return self.even.value(u, v, p);
        }
    }
}

#[derive(Debug)]
pub struct NoiseTexture {
    scale: f64
}

impl NoiseTexture {
    pub fn new(scale: f64) -> NoiseTexture {
        NoiseTexture{scale: scale}
    }
}

impl Texture for NoiseTexture {
    #[allow(unused)]
    fn value(&self, u: f64, v: f64, p: &Vec3<f64>) -> Vec3<f64> {
        // Vec3::new(1.,1.,1.)*old_noise3(&(self.scale * p))
        // Vec3::new(1.,1.,1.)*0.5 * (1.+perlin_noise(&(self.scale * p)))
        Vec3::new(1.,1.,1.)*0.5 * (1.+(self.scale*p.z + 10.*turb_noise(p, 7)).sin())
    }
}
