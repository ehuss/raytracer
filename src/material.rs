use ray::*;
use hitable::*;
use vec3::*;
use util::*;

/// Reflect a vector from a surface.
/// v is the incoming vector, n is the normal of the surface.
fn reflect(v: &Vec3<f64>, n: &Vec3<f64>) -> Vec3<f64> {
    v - (2.0 * dot(v, n) * n)
}

pub trait Material {
    /// Return is (scattered, attenuation) where scattered is the direction
    /// the ray should scatter in.  Attenuation is the attenuation of the
    /// color.  Return None if there is no scatter.
    fn scatter(&self, rng: &mut Rng, r_in: &Ray<f64>, rec: &HitRecord) -> Option<(Ray<f64>, Vec3<f64>)>;
}

pub struct Lambertian {
    albedo: Vec3<f64>
}

impl Lambertian {
    pub fn new(a: Vec3<f64>) -> Lambertian {
        Lambertian{albedo: a}
    }
}

impl Material for Lambertian {
    fn scatter(&self, rng: &mut Rng, _: &Ray<f64>, rec: &HitRecord) -> Option<(Ray<f64>, Vec3<f64>)> {
        let target = rec.p + rec.normal + random_in_unit_sphere(rng);
        Some((
            Ray::new(rec.p, target-rec.p),
            self.albedo
            ))
    }
}

/// Get a random point on a unit sphere.
fn random_in_unit_sphere(rng: &mut Rng) -> Vec3<f64>
{
    // Simple algorithm, pick a random point in a unit cube (range -1..1).
    // Repeat if the point is outside the sphere.
    loop {
        let p = 2.0 * Vec3::new(rng.rand64(), rng.rand64(), rng.rand64()) - Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

#[derive(Debug)]
pub struct Metal {
    albedo: Vec3<f64>,
    fuzz: f64,
}

impl Metal {
    /// Create a new Metal material.
    ///
    /// * `albedo`: The albedo/color.
    /// * `fuzz`: The amount of scattering, creating a rough surface (0=mirror finish).
    pub fn new(albeda: Vec3<f64>, fuzz: f64) -> Metal {
        Metal{albedo: albeda, fuzz: fuzz.min(1.0)}
    }
}

impl Material for Metal {
    fn scatter(&self, rng: &mut Rng, r_in: &Ray<f64>, rec: &HitRecord) -> Option<(Ray<f64>, Vec3<f64>)> {
        let reflected = reflect(&r_in.direction().unit_vector(), &rec.normal);
        // Randomly adjust the reflection to create a rougher surface.
        let scattered = Ray::new(rec.p, reflected + self.fuzz*random_in_unit_sphere(rng));
        // Limit scatter rays to those that are <90° from the normal.
        if dot(&scattered.direction(), &rec.normal) > 0.0 {
            return Some((scattered, self.albedo))
        } else {
            return None
        }
    }
}
