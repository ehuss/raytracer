use vec3::*;
use std::fmt;
use perlin::*;
use image;
use image::{GenericImage, Pixel};

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


pub struct ImageTexture {
    img: image::DynamicImage,
    nx: u32,
    ny: u32,
}

impl ImageTexture {
    pub fn new(img: image::DynamicImage) -> ImageTexture {
        let (x, y) = img.dimensions();
        ImageTexture{img: img, nx: x, ny: y}
    }
}

impl fmt::Debug for ImageTexture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ImageTexture{{ nx: {}, ny: {}}}", self.nx, self.ny)
    }
}

impl Texture for ImageTexture {
    #[allow(unused)]
    fn value(&self, u: f64, v: f64, p: &Vec3<f64>) -> Vec3<f64> {
        let mut i = (u*self.nx as f64) as i32;
        let mut j = ((1.-v)*self.ny as f64-0.001) as i32;
        let nxi = self.nx as i32;
        let nyi = self.ny as i32;
        if i < 0 { i = 0; }
        if j < 0 { j = 0; }
        if i > nxi-1 {i = nxi-1;}
        if j > nyi-1 {j = nyi-1;}
        let p = self.img.get_pixel(i as u32, j as u32).to_rgb();
        return Vec3::new(p[0] as f64/255., p[1] as f64/255., p[2] as f64/255.);
    }
}
