#![allow(unused)]

extern crate raytracer;
extern crate image;

use raytracer::*;

/// Get color for ray r cast into scene.
///
/// A miss into the background is a linear gradient from white to blue.
fn color(rng: &mut Rng, r: &Ray<f64>, world: &Box<Hitable>, depth: u8) -> Vec3<f64> {
    // Use 0.0001 to ignore hits very near zero (the ray should travel at
    // least some distance).
    if let Some(h) = world.hit(rng, r, 0.0001, std::f64::MAX) {
        let emitted = h.material.emitted(r, &h, h.u, h.v, &h.p);
        if depth < 50 {
            if let Some((scattered, albedo, pdf)) = h.material.scatter(rng, r, &h) {
                let on_light = Vec3::new(213.+rng.rand64()*(343.-213.), 554., 227.+rng.rand64()*(332.-227.));
                let mut to_light = on_light - h.p;
                let distance_squared = to_light.squared_length();
                to_light.make_unit_vector();
                if dot(&to_light, &h.normal) < 0. {
                    return emitted;
                }
                let light_area = (343.-213.)*(332.-227.);
                let light_cosine = to_light.y.abs();
                if light_cosine < 0.000001 {
                    return emitted;
                }
                let pdf = distance_squared / (light_cosine * light_area);
                let scattered = Ray::new_time(h.p, to_light, r.time());
                return emitted + albedo * h.material.scattering_pdf(r, &h, &scattered)*color(rng, &scattered, world, depth + 1) / pdf;
            } else {
                return emitted;
            }
        }
        return Vec3::zero();
    } else {
        // Hit background.
        // let unit_direction = r.direction().unit_vector();
        // let t = 0.5 * (unit_direction.y + 1.0);
        // return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
        return Vec3::zero();
    }
}

fn two_spheres(rng: &mut Rng) -> Box<Hitable> {
    let checker = CheckerTexture::new(
        Box::new(ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1))),
        Box::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9)))
    );
    let mut list = HitableList::new();
    let mat = Rc::new(Lambertian::new(Box::new(checker)));
    list.add_hitable(Sphere::new(Vec3::new(0., -10., 0.), 10., mat.clone()));
    list.add_hitable(Sphere::new(Vec3::new(0.,  10., 0.), 10., mat.clone()));
    return Box::new(list);
}

fn two_perlin_spheres(rng: &mut Rng) -> Box<Hitable> {
    let pertext = NoiseTexture::new(4.);
    let mut list = HitableList::new();
    let mat = Rc::new(Lambertian::new(Box::new(pertext)));
    list.add_hitable(Sphere::new(Vec3::new(0., -1000., 0.), 1000., mat.clone()));
    list.add_hitable(Sphere::new(Vec3::new(0., 2., 0.), 2., mat.clone()));
    return Box::new(list);
}

fn earth() -> Box<Hitable> {
    let img = image::open("earthmap1k.jpg").unwrap();
    let mat = Lambertian::new(Box::new(ImageTexture::new(img)));
    return Box::new(Sphere::new(Vec3::zero(), 2., Rc::new(mat)));
}

fn simple_light() -> Box<Hitable> {
    let pertext = NoiseTexture::new(4.);
    let lamb_mat = Rc::new(Lambertian::new(Box::new(pertext)));
    let ctex = ConstantTexture::new(Vec3::new(4.,4.,4.));
    let lit_mat = Rc::new(DiffuseLight::new(Box::new(ctex)));
    let mut list = HitableList::new();
    list.add_hitable(Sphere::new(Vec3::new(0., -1000., 0.), 1000., lamb_mat.clone()));
    list.add_hitable(Sphere::new(Vec3::new(0., 2., 0.), 2., lamb_mat.clone()));
    list.add_hitable(Sphere::new(Vec3::new(0., 7., 0.), 2., lit_mat.clone()));
    list.add_hitable(XYRect::new(3., 5., 1., 3., -2., lit_mat.clone()));
    return Box::new(list);
}

fn cornell_box() -> Box<Hitable> {
    let mut list = HitableList::new();
    let red = Rc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(0.65, 0.05, 0.05)))));
    let white = Rc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73)))));
    let green = Rc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(0.12, 0.45, 0.15)))));
    let light = Rc::new(DiffuseLight::new(Box::new(ConstantTexture::new(Vec3::new(15., 15., 15.)))));
    list.add_hitable(FlipNormals::new(Box::new(YZRect::new(0., 555., 0., 555., 555., green.clone()))));
    list.add_hitable(YZRect::new(0., 555., 0., 555., 0., red.clone()));
    list.add_hitable(XZRect::new(213., 343., 227., 332., 554., light.clone()));
    list.add_hitable(FlipNormals::new(Box::new(XZRect::new(0., 555., 0., 555., 555., white.clone()))));
    list.add_hitable(XZRect::new(0., 555., 0., 555., 0., white.clone()));
    list.add_hitable(FlipNormals::new(Box::new(XYRect::new(0., 555., 0., 555., 555., white.clone()))));
    let b = Box::new(HBox::new(Vec3::new(0., 0., 0.), Vec3::new(165., 165., 165.), white.clone()));
    list.add_hitable(Translate::new(Box::new(
        RotateY::new(b, -18.)), Vec3::new(130., 0., 65.)));
    let b = Box::new(HBox::new(Vec3::new(0., 0., 0.), Vec3::new(165., 330., 165.), white.clone()));
    list.add_hitable(Translate::new(Box::new(RotateY::new(b, 15.)), Vec3::new(265., 0., 295.)));
    return Box::new(list);
}

fn cornell_smoke() -> Box<Hitable> {
    let mut list = HitableList::new();
    let red = Rc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(0.65, 0.05, 0.05)))));
    let white = Rc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73)))));
    let green = Rc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(0.12, 0.45, 0.15)))));
    let light = Rc::new(DiffuseLight::new(Box::new(ConstantTexture::new(Vec3::new(7., 7., 7.)))));
    list.add_hitable(FlipNormals::new(Box::new(YZRect::new(0., 555., 0., 555., 555., green.clone()))));
    list.add_hitable(YZRect::new(0., 555., 0., 555., 0., red.clone()));
    list.add_hitable(XZRect::new(113., 443., 127., 432., 554., light.clone()));
    list.add_hitable(FlipNormals::new(Box::new(XZRect::new(0., 555., 0., 555., 555., white.clone()))));
    list.add_hitable(XZRect::new(0., 555., 0., 555., 0., white.clone()));
    list.add_hitable(FlipNormals::new(Box::new(XYRect::new(0., 555., 0., 555., 555., white.clone()))));
    let b = Box::new(HBox::new(Vec3::new(0., 0., 0.), Vec3::new(165., 165., 165.), white.clone()));
    let b = Translate::new(Box::new(RotateY::new(b, -18.)), Vec3::new(130., 0., 65.));
    let b = ConstantMedium::new(Box::new(b), 0.01, Box::new(ConstantTexture::new(Vec3::new(1., 1., 1.))));
    list.add_hitable(b);
    let b = Box::new(HBox::new(Vec3::new(0., 0., 0.), Vec3::new(165., 330., 165.), white.clone()));
    let b = Translate::new(Box::new(RotateY::new(b, 15.)), Vec3::new(265., 0., 295.));
    let b = ConstantMedium::new(Box::new(b), 0.01, Box::new(ConstantTexture::new(Vec3::new(0., 0., 0.))));
    list.add_hitable(b);
    return Box::new(list);
}


fn random_scene(rng: &mut Rng) -> Box<Hitable> {
    let mut list: Vec<Box<Hitable>> = Vec::new();
    let checker = CheckerTexture::new(
        Box::new(ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1))),
        Box::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9)))
    );
    list.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0),
                                   1000.0,
                                   Rc::new(Lambertian::new(Box::new(checker))))));
    // XXX: Not sure why explicit i8 is required here to cast to f64.
    for a in -10..10i8 {
        for b in -10..10i8 {
            let choose_mat = rng.rand64();
            let center = Vec3::new(a as f64 + 0.9 * rng.rand64(),
                                   0.2,
                                   b as f64 + 0.9 * rng.rand64());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let texture = ConstantTexture::new(Vec3::new(rng.rand64()*rng.rand64(), rng.rand64()*rng.rand64(), rng.rand64()*rng.rand64()));
                    list.push(Box::new(
                        MovingSphere::new(center, center+Vec3::new(0.0,0.5*rng.rand64(), 0.0), 0.0, 1.0, 0.2, Rc::new(Lambertian::new(
                            Box::new(texture))))));
                } else if choose_mat < 0.95 {
                    // metal
                    let mat = Metal::new(Vec3::new(0.5 * (1.0 + rng.rand64()),
                                                   0.5 * (1.0 + rng.rand64()),
                                                   0.5 * (1.0 + rng.rand64())),
                                         0.5 * rng.rand64());
                    list.push(Box::new(Sphere::new(center, 0.2, Rc::new(mat))));
                } else {
                    // glass
                    list.push(Box::new(Sphere::new(center, 0.2, Rc::new(Dielectric::new(1.5)))));
                }
            }
        }
    }
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Rc::new(Dielectric::new(1.5)))));
    let texture = ConstantTexture::new(Vec3::new(0.4, 0.2, 0.1));
    list.push(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0),
                                   1.0,
                                   Rc::new(Lambertian::new(Box::new(texture))))));
    list.push(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0),
                                   1.0,
                                   Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)))));
    let bvh = BVHNode::new(rng, list, 0.0, 1.0);
    return Box::new(bvh);
}

fn final_scene() -> Box<Hitable> {
    let mut rng = Rng::new();
    let num_boxes = 20u8;
    let mut list = HitableList::new();
    let mut boxlist: Vec<Box<Hitable>> = Vec::new();
    let mut boxlist2: Vec<Box<Hitable>> = Vec::new();
    let white = Rc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73)))));
    let ground = Rc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(0.48, 0.83, 0.53)))));
    for i in 0..num_boxes {
        for j in 0..num_boxes {
            let w = 100.;
            let x0 = -1000. + i as f64*w;
            let z0 = -1000. + j as f64*w;
            let y0 = 0.;
            let x1 = x0 + w;
            let y1 = 100.*(rng.rand64() + 0.01);
            let z1 = z0 + w;
            let b = HBox::new(Vec3::new(x0, y0, z0), Vec3::new(x1, y1, x1), ground.clone());
            boxlist.push(Box::new(b));
        }
    }
    list.add_hitable(BVHNode::new(&mut rng, boxlist, 0., 1.));
    let light = Rc::new(DiffuseLight::new(Box::new(ConstantTexture::new(Vec3::new(7., 7., 7.)))));
    list.add_hitable(XZRect::new(123., 432., 147., 412., 554., light.clone()));
    let center = Vec3::new(400., 400., 200.);
    list.add_hitable(MovingSphere::new(center, center+Vec3::new(30., 0., 0.), 0., 1., 50., Rc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(0.7, 0.3, 0.1)))))));
    list.add_hitable(Sphere::new(Vec3::new(260., 150., 45.), 50., Rc::new(Dielectric::new(1.5))));
    list.add_hitable(Sphere::new(Vec3::new(0., 150., 145.), 50., Rc::new(Metal::new(Vec3::new(0.8, 0.8, 0.9), 10.))));
    let boundary = Sphere::new(Vec3::new(360., 150., 145.), 70., Rc::new(Dielectric::new(1.5)));
    let boundary2 = boundary.clone();
    list.add_hitable(boundary);
    list.add_hitable(ConstantMedium::new(Box::new(boundary2), 0.2, Box::new(ConstantTexture::new(Vec3::new(0.2, 0.4, 0.9)))));
    let boundary = Sphere::new(Vec3::zero(), 5000., Rc::new(Dielectric::new(1.5)));
    let img = image::open("earthmap1k.jpg").unwrap();
    let emat = Lambertian::new(Box::new(ImageTexture::new(img)));
    list.add_hitable(Sphere::new(Vec3::new(400., 200., 400.), 100., Rc::new(emat)));
    let pertext = NoiseTexture::new(0.1);
    list.add_hitable(Sphere::new(Vec3::new(220., 280., 300.), 80., Rc::new(Lambertian::new(Box::new(pertext)))));
    for j in 0..1000 {
        boxlist2.push(Box::new(Sphere::new(Vec3::new(165.*rng.rand64(), 165.*rng.rand64(), 165.*rng.rand64()), 10., white.clone())));
    }
    list.add_hitable(Translate::new(Box::new(RotateY::new(Box::new(BVHNode::new(&mut rng, boxlist2, 0., 1.)), 15.)), Vec3::new(-100., 270., 395.)));
    return Box::new(list);
}

// y-up
// into screen is -z
// Traverse from lower-left corner (-2, -1) to upper-right +(+2,+2)

fn main() {
    let nx = 400;
    let ny = 200;
    let ns = 100;
    perlin_init();
    println!("P3\n{} {}\n255", nx, ny);
    let mut rng = Rng::new();
    let lookfrom = Vec3::new(278., 278., -800.);
    let lookat = Vec3::new(278., 278., 0.);
    let dist_to_focus = 10.0;//(lookfrom-lookat).length();
    let aperture = 0.0;
    let cam = Camera::new(lookfrom,
                          lookat,
                          Vec3::new(0.0, 1.0, 0.0),
                          40.0,
                          nx as f64 / ny as f64,
                          aperture,
                          dist_to_focus,
                          0.0,
                          1.0);
    // let world = random_scene(&mut rng);
    // let world = two_spheres(&mut rng);
    // let world = two_perlin_spheres(&mut rng);
    // let world = final_scene();
    let world = cornell_box();
    for j in (0..ny - 1).rev() {
        for i in 0..nx {
            let mut col = Vec3::<f64>::zero();
            for _ in 0..ns {
                let u = (i as f64 + rng.rand64()) / nx as f64;
                let v = (j as f64 + rng.rand64()) / ny as f64;
                let r = cam.get_ray(&mut rng, u, v);
                col += color(&mut rng, &r, &world, 0);
            }
            col /= ns as f64;
            // Poor-man's gamma correction.
            col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
            let ir = (255.99 * col.x) as u32;
            let ig = (255.99 * col.y) as u32;
            let ib = (255.99 * col.z) as u32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
