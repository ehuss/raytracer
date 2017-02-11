#![feature(test)]

extern crate test;
extern crate raytracer;
use test::Bencher;
use raytracer::*;

#[bench]
fn bench_perlin(b: &mut Bencher) {
    let v = Vec3::new(0.5, 0.6, 0.7);
    b.iter(|| perlin_noise(&v))
}
