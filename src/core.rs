use vec3::*;
use ray::*;
use scene::*;
use util::*;
use pdf::*;
use output::*;
use output;
// use std::error::Error;
use perlin::*;

#[inline(always)]
fn de_nan(c: &Vec3<f64>) -> Vec3<f64> {
    let x = if c.x.is_nan() {0.} else {c.x};
    let y = if c.y.is_nan() {0.} else {c.y};
    let z = if c.z.is_nan() {0.} else {c.z};
    return Vec3::new(x, y, z);
}

/// Get color for ray r cast into scene.
///
/// A miss into the background is a linear gradient from white to blue.
fn color(rng: &mut Rng, r: &Ray<f64>, scene: &Scene, depth: u8) -> Vec3<f64> {
    // Use 0.0001 to ignore hits very near zero (the ray should travel at
    // least some distance).
    if let Some(hrec) = scene.world.hit(rng, r, 0.0001, f64::MAX) {
        let emitted = hrec.material.emitted(r, &hrec, hrec.u, hrec.v, &hrec.p);
        if depth < 50 {
            if let Some(srec) = hrec.material.scatter(rng, r, &hrec) {
                if let Some(specular_ray) = srec.specular_ray {
                    return srec.attenuation * color(rng, &specular_ray, scene, depth+1);
                } else {
                    let plight = HitablePdf::new(hrec.p, &*scene.light_shapes);
                    let spdf = srec.pdf.unwrap();
                    let p = MixturePdf::new(&plight, &*spdf);
                    let scattered = Ray::new_time(hrec.p, p.generate(rng), r.time());
                    let pdf_val = p.value(rng, &scattered.direction());
                    return emitted + srec.attenuation*hrec.material.scattering_pdf(r, &hrec, &scattered)*
                        color(rng, &scattered, scene, depth + 1) / pdf_val;
                }
            } else {
                return emitted;
            }
        }
        return emitted;
    } else {
        // Hit background.
        // let unit_direction = r.direction().unit_vector();
        // let t = 0.5 * (unit_direction.y + 1.0);
        // return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
        return Vec3::zero();
    }
}

pub fn render(scene: &Scene, output_settings: &OutputSettings, output: &mut Box<Output>) -> output::Result<()> {
    perlin_init();
    output.begin()?;
    let mut rng = Rng::new();
    let width = output_settings.width;
    let height = output_settings.height;
    let ns = scene.num_samples;
    let buckets = output::create_buckets(width, height, output::BucketStrategy::Spiral, 64, 64);
    for bucket in buckets {
        output.begin_bucket(&bucket)?;
        render_bucket(&mut rng, scene, output, ns, width, height, bucket)?;
    }

    output.end()?;
    Ok(())
}

fn render_bucket(rng: &mut Rng,
                 scene: &Scene,
                 output: &mut Box<Output>,
                 ns: u32,
                 width: u32,
                 height: u32,
                 bucket: output::Bucket)
                 -> output::Result<()>
{
    let mut pixels = Vec::with_capacity(bucket.height as usize);
    for bucket_j in 0..bucket.height {
        let mut pixel_row = Vec::with_capacity(bucket.width as usize);
        // Reverse since camera rays go from bottom-left corner, but
        // the output API sets the origin the upper-left corner.
        let j = height - (bucket_j + bucket.y);
        for bucket_i in 0..bucket.width {
            let i = bucket_i + bucket.x;
            let mut col = Vec3::<f64>::zero();
            for _ in 0..ns {
                let u = (i as f64 + rng.rand64()) / width as f64;
                let v = (j as f64 + rng.rand64()) / height as f64;
                let r = scene.camera.get_ray(rng, u, v);

                col += de_nan(&color(rng, &r, scene, 0));
            }
            col /= ns as f64;
            // Poor-man's gamma correction.
            col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
            pixel_row.push(col);
            // output.put_pixel(i, height-j-1, &col)?;
        }
        pixels.push(pixel_row);
    }
    output.put_bucket(&bucket, &pixels)?;

    Ok(())
}
