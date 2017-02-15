use vec3::*;
use util::*;

pub fn random_cosine_direction(rng: &mut Rng) -> Vec3<f64> {
    let r1 = rng.rand64();
    let r2 = rng.rand64();
    let z = (1.-r2).sqrt();
    let phi = 2.*PI*r1;
    let x = phi.cos()*2.*r2.sqrt();
    let y = phi.sin()*2.*r2.sqrt();
    return Vec3::new(x,y,z);
}
