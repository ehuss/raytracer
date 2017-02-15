use ray::*;
use hitable::*;
use vec3::*;
use util::*;
use texture::*;
use onb::*;
use pdf::*;

/// Reflect a vector from a surface.
/// v is the incoming vector, n is the normal of the surface.
fn reflect(v: &Vec3<f64>, n: &Vec3<f64>) -> Vec3<f64> {
    v - (2.0 * dot(v, n) * n)
}

/// Refract a vector from a surface (Snell's law).
///
/// * `v`: Incoming vector.
/// * `n`: Surface normal.
/// * `ni_over_nt`: Ratio of refractive indices.
///
/// Returns the Refracted vector (or None for no refraction).
fn refract(v: &Vec3<f64>, n: &Vec3<f64>, ni_over_nt: f64) -> Option<Vec3<f64>> {
    let uv = v.unit_vector();
    let dt = dot(&uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        return Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt());
    } else {
        return None;
    }
}
/// Approximation to vary reflectivity with angle (Christophe Schlick).
fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}

pub trait Material: fmt::Debug {
    /// Return is (scattered, abledo, pdf) where scattered is the direction
    /// the ray should scatter in.  Albedo is the attenuation of the
    /// color.  pdf is f64.  Return None if there is no scatter.
    #[allow(unused)]
    fn scatter(&self,
               rng: &mut Rng,
               r_in: &Ray<f64>,
               rec: &HitRecord)
               -> Option<(Ray<f64>, Vec3<f64>, f64)> {
        None
    }

    fn scattering_pdf(&self, r_in: &Ray<f64>, rec: &HitRecord, scattered: &Ray<f64>) -> f64 {
        0.
    }

    #[allow(unused)]
    fn emitted(&self, r_in: &Ray<f64>, rec: &HitRecord, u: f64, v: f64, p: &Vec3<f64>) -> Vec3<f64> {
        Vec3::zero()
    }
}

#[derive(Debug)]
pub struct Lambertian {
    albedo: Box<Texture>,
}

impl Lambertian {
    pub fn new(a: Box<Texture>) -> Lambertian {
        Lambertian { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(&self,
               rng: &mut Rng,
               r_in: &Ray<f64>,
               rec: &HitRecord)
               -> Option<(Ray<f64>, Vec3<f64>, f64)> {
        let mut uvw = Onb::new();
        uvw.build_from_w(&rec.normal);
        let direction = uvw.local_vec(&random_cosine_direction(rng));
        let scattered = Ray::new_time(rec.p, direction.unit_vector(), r_in.time());
        let alb = self.albedo.value(rec.u, rec.v, &rec.p);
        let pdf = dot(&uvw.w(), &scattered.direction()) / PI;
        return Some((scattered, alb, pdf));
    }
    fn scattering_pdf(&self, r_in: &Ray<f64>, rec: &HitRecord, scattered: &Ray<f64>) -> f64 {
        let cosine = dot(&rec.normal, &scattered.direction().unit_vector());
        if cosine < 0. {
            return 0.;
        }
        return cosine / PI;
    }

}

/// Get a random point on a unit sphere.
fn random_in_unit_sphere(rng: &mut Rng) -> Vec3<f64> {
    // Simple algorithm, pick a random point in a unit cube (range -1..1).
    // Repeat if the point is outside the sphere.
    loop {
        let p = 2.0 * Vec3::new(rng.rand64(), rng.rand64(), rng.rand64()) -
                Vec3::new(1.0, 1.0, 1.0);
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
        Metal {
            albedo: albeda,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self,
               rng: &mut Rng,
               r_in: &Ray<f64>,
               rec: &HitRecord)
               -> Option<(Ray<f64>, Vec3<f64>, f64)> {
        let reflected = reflect(&r_in.direction().unit_vector(), &rec.normal);
        // Randomly adjust the reflection to create a rougher surface.
        let scattered = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere(rng));
        // Limit scatter rays to those that are <90° from the normal.
        if dot(&scattered.direction(), &rec.normal) > 0.0 {
            return Some((scattered, self.albedo, 0.));
        } else {
            return None;
        }
    }
}


#[derive(Debug)]
pub struct Dielectric {
    /// Refractive index.
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Dielectric {
        Dielectric { ref_idx: ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(&self,
               rng: &mut Rng,
               r_in: &Ray<f64>,
               rec: &HitRecord)
               -> Option<(Ray<f64>, Vec3<f64>, f64)> {
        let reflected = reflect(&r_in.direction(), &rec.normal);
        let outward_normal;
        let ni_over_nt;
        let cosine;
        if dot(&r_in.direction(), &rec.normal) > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = self.ref_idx;
            let c = dot(&r_in.direction(), &rec.normal) / r_in.direction().length();
            cosine = (1.0 - self.ref_idx * self.ref_idx * (1.0 - c * c)).sqrt();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -dot(&r_in.direction(), &rec.normal) / r_in.direction().length();
        }
        let reflect_prob;
        let scattered;
        let refracted = refract(&r_in.direction(), &outward_normal, ni_over_nt);
        if refracted.is_some() {
            reflect_prob = schlick(cosine, self.ref_idx);
        } else {
            reflect_prob = 1.0;
        }
        if rng.rand64() < reflect_prob {
            scattered = Ray::new(rec.p, reflected);
        } else {
            scattered = Ray::new(rec.p, refracted.unwrap());
        }
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        return Some((scattered, attenuation, 0.));
    }
}

#[derive(Debug, new)]
pub struct DiffuseLight {
    emit: Box<Texture>
}


impl Material for DiffuseLight {

    fn emitted(&self, r_in: &Ray<f64>, rec: &HitRecord, u: f64, v: f64, p: &Vec3<f64>) -> Vec3<f64> {
        // Only emit in one direction.
        if dot(&rec.normal, &r_in.direction()) < 0. {
            return self.emit.value(u, v, p);
        } else {
            return Vec3::zero();
        }
    }
}

#[derive(Debug, new)]
pub struct Isotropic {
    albedo: Box<Texture>
}

impl Material for Isotropic {
    #[allow(unused)]
    fn scatter(&self,
               rng: &mut Rng,
               r_in: &Ray<f64>,
               rec: &HitRecord)
               -> Option<(Ray<f64>, Vec3<f64>, f64)> {
        let scattered = Ray::new(rec.p, random_in_unit_sphere(rng));
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        return Some((scattered, attenuation, 0.));
    }
}
