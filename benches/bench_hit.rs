#![feature(test)]

extern crate test;
extern crate raytracer;
use test::Bencher;
use raytracer::*;
use std::rc::Rc;

#[bench]
fn bench_hit_sphere(b: &mut Bencher) {
    let mat = Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(1.0, 0.0, 0.0))));
    let mut rng = Rng::new();
    let s = Sphere::new(Vec3::new(3.0, 0.2, 4.0), 0.2, Rc::new(mat));
    let r = Ray::new(Vec3::new(13.,2.,3.),
                     Vec3::new(-10., 0.0, -2.0));
    b.iter(|| s.hit(&mut rng, &r, 0.0, 1.0));
}

#[bench]
fn bench_hit_aabb(b: &mut Bencher) {
    let mat = Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(1.0, 0.0, 0.0))));
    let mut rng = Rng::new();
    let s = Sphere::new(Vec3::new(3.0, 0.2, 4.0), 0.2, Rc::new(mat));
    let r = Ray::new(Vec3::new(13.,2.,3.),
                     Vec3::new(-10., 0.0, -2.0));
    let bbox = s.bounding_box(0.0, 1.0).unwrap();
    b.iter(|| bbox.hit(&mut rng, &r, 0.0, 1.0));

}
