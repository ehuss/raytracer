#![allow(non_snake_case)]
extern crate rand;

use vec3::*;
use rand::Rng;
use std::sync;

static mut PERM_X: [u8; 256] = [0; 256];
static mut PERM_Y: [u8; 256] = [0; 256];
static mut PERM_Z: [u8; 256] = [0; 256];
static mut RANFLOAT: [f64; 256] = [0.; 256];
static mut RANVEC: [Vec3<f64>; 256] = [Vec3{x:0.,y:0.,z:0.}; 256];

// Unfortunately, the lazy_static crate adds a little overhead to read from
// the static, which I find unacceptable.  Instead, just use unsafe access,
// which should be fine for read-only data as long as it is initialized
// correctly.  Alternatively, could package the data into a struct, or just
// precompute the values and place the literals directly in this rs file.

/// Initialize noise.  Must be called once (in main).
pub fn perlin_init() {
    unsafe {
        static mut ONCE: sync::Once = sync::ONCE_INIT;
        ONCE.call_once(|| {
            PERM_X = perlin_generate_perm();
            PERM_Y = perlin_generate_perm();
            PERM_Z = perlin_generate_perm();
            RANFLOAT = perlin_generate_float();
            RANVEC = perlin_generate();
        });
    }
}

// Accessors to hide the unsafe blocks.
fn get_perm_x() -> &'static [u8; 256] {unsafe { &PERM_X }}
fn get_perm_y() -> &'static [u8; 256] {unsafe { &PERM_Y }}
fn get_perm_z() -> &'static [u8; 256] {unsafe { &PERM_Z }}
fn get_ranfloat() -> &'static [f64; 256] {unsafe { &RANFLOAT }}
fn get_ranvec() -> &'static [Vec3<f64>; 256] {unsafe { &RANVEC }}

fn perlin_generate_perm() -> [u8; 256] {
    let mut result = [0u8; 256];
    for i in 0..256 {
        result[i] = i as u8;
    }
    // rand::thread_rng().shuffle(result)
    permute(&mut result);
    return result;
}

fn permute(p: &mut [u8; 256]) {
    let mut rng = rand::thread_rng();
    for i in (1..256).rev() {
        let target: usize = rng.gen_range(0, i + 1);
        let tmp = p[i];
        p[i] = p[target];
        p[target] = tmp;
    }
}

fn perlin_generate() -> [Vec3<f64>; 256] {
    let mut rng = rand::thread_rng();
    let mut result = [Vec3::zero(); 256];
    for i in 0..256 {
        let mut v = Vec3::new(-1. + 2. * rng.gen::<f64>(),
                              -1. + 2. * rng.gen::<f64>(),
                              -1. + 2. * rng.gen::<f64>());
        v.make_unit_vector();
        result[i] = v;
    }
    return result;
}

fn perlin_generate_float() -> [f64; 256] {
    let mut rng = rand::thread_rng();
    let mut result = [0.; 256];
    for i in 0..256 {
        result[i] = rng.gen::<f64>();
    }
    return result;
}

pub fn old_noise1(p: &Vec3<f64>) -> f64 {
    let PX = get_perm_x();
    let PY = get_perm_y();
    let PZ = get_perm_z();
    let RF = get_ranfloat();
    let i = (4. * p.x) as usize & 255;
    let j = (4. * p.y) as usize & 255;
    let k = (4. * p.z) as usize & 255;
    return RF[(PX[i] ^ PY[j] ^ PZ[k]) as usize];
}


pub fn old_noise2(p: &Vec3<f64>) -> f64 {
    let PX = get_perm_x();
    let PY = get_perm_y();
    let PZ = get_perm_z();
    let RF = get_ranfloat();
    let u = p.x - p.x.floor();
    let v = p.y - p.y.floor();
    let w = p.z - p.z.floor();
    let i = p.x.floor() as usize;
    let j = p.y.floor() as usize;
    let k = p.z.floor() as usize;
    let mut c = [0.; 8];
    for di in 0..2 {
        for dj in 0..2 {
            for dk in 0..2 {
                let index = PX[(i + di) & 255] ^
                            PY[(j + dj) & 255] ^
                            PZ[(k + dk) & 255];
                c[di * 4 + dj * 2 + dk] = RF[index as usize];
            }
        }
    }
    let mut accum = 0.;
    for i in 0..2 {
        let fi = i as f64;
        for j in 0..2 {
            let fj = j as f64;
            for k in 0..2 {
                let fk = k as f64;
                accum += (fi * u + (1. - fi) * (1. - u)) *
                         (fj * v + (1. - fj) * (1. - v)) *
                         (fk * w + (1. - fk) * (1. - w)) *
                         c[i * 4 + j * 2 + k];

            }
        }
    }
    return accum;
}

pub fn old_noise3(p: &Vec3<f64>) -> f64 {
    let PX = get_perm_x();
    let PY = get_perm_y();
    let PZ = get_perm_z();
    let RF = get_ranfloat();
    let u = p.x - p.x.floor();
    let v = p.y - p.y.floor();
    let w = p.z - p.z.floor();
    let u = u * u * (3. - 2. * u);
    let v = v * v * (3. - 2. * v);
    let w = w * w * (3. - 2. * w);
    let i = p.x.floor() as usize;
    let j = p.y.floor() as usize;
    let k = p.z.floor() as usize;
    let mut c = [0.; 8];
    for di in 0..2 {
        for dj in 0..2 {
            for dk in 0..2 {
                let index = PX[(i + di) & 255] ^
                            PY[(j + dj) & 255] ^
                            PZ[(k + dk) & 255];
                c[di * 4 + dj * 2 + dk] = RF[index as usize];
            }
        }
    }
    let mut accum = 0.;
    for i in 0..2 {
        let fi = i as f64;
        for j in 0..2 {
            let fj = j as f64;
            for k in 0..2 {
                let fk = k as f64;
                accum += (fi * u + (1. - fi) * (1. - u)) *
                         (fj * v + (1. - fj) * (1. - v)) *
                         (fk * w + (1. - fk) * (1. - w)) *
                         c[i * 4 + j * 2 + k];

            }
        }
    }
    return accum;
}

pub fn perlin_noise(p: &Vec3<f64>) -> f64 {
    let PX = get_perm_x();
    let PY = get_perm_y();
    let PZ = get_perm_z();
    let RV = get_ranvec();
    let u = p.x - p.x.floor();
    let v = p.y - p.y.floor();
    let w = p.z - p.z.floor();
    let i = p.x.floor() as usize;
    let j = p.y.floor() as usize;
    let k = p.z.floor() as usize;
    let mut c = [Vec3::zero(); 8];
    for di in 0..2 {
        for dj in 0..2 {
            for dk in 0..2 {
                let index = PX[(i + di) & 255] ^
                            PY[(j + dj) & 255] ^
                            PZ[(k + dk) & 255];
                c[di * 4 + dj * 2 + dk] = RV[index as usize];
            }
        }
    }
    // Add some interpolation to smooth the pattern using a hermite cube.
    let uu = u * u * (3. - 2. * u);
    let vv = v * v * (3. - 2. * v);
    let ww = w * w * (3. - 2. * w);
    let mut accum = 0.;
    for i in 0..2 {
        let fi = i as f64;
        for j in 0..2 {
            let fj = j as f64;
            for k in 0..2 {
                let fk = k as f64;
                let weight_v = Vec3::new(u - fi, v - fj, w - fk);
                accum += (fi * uu + (1. - fi) * (1. - uu)) *
                         (fj * vv + (1. - fj) * (1. - vv)) *
                         (fk * ww + (1. - fk) * (1. - ww)) *
                         dot(&c[i * 4 + j * 2 + k], &weight_v);
            }
        }
    }
    return accum;
}

/// A marbled noise texture.
pub fn turb_noise(p: &Vec3<f64>, depth: usize) -> f64 {
    let mut accum = 0.;
    let mut temp_p = p.clone();
    let mut weight = 1.0;
    for _ in 0..depth {
        accum += weight * perlin_noise(&temp_p);
        weight *= 0.5;
        temp_p *= 2.;
    }
    return accum.abs();
}
