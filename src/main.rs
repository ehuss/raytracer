#![allow(unused)]

extern crate raytracer;
extern crate image;
#[macro_use]
extern crate clap;

use raytracer::*;


/*
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
*/

fn cornell_box() -> Scene {
    let mut list = HitableList::new();
    let red = Rc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(0.65, 0.05, 0.05)))));
    let white = Rc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73)))));
    let green = Rc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(0.12, 0.45, 0.15)))));
    let light = Rc::new(DiffuseLight::new(Box::new(ConstantTexture::new(Vec3::new(15., 15., 15.)))));
    list.add_hitable(FlipNormals::new(Box::new(YZRect::new(0., 555., 0., 555., 555., green.clone()))));
    list.add_hitable(YZRect::new(0., 555., 0., 555., 0., red.clone()));
    list.add_hitable(FlipNormals::new(Box::new(XZRect::new(213., 343., 227., 332., 554., light.clone()))));
    list.add_hitable(FlipNormals::new(Box::new(XZRect::new(0., 555., 0., 555., 555., white.clone()))));
    list.add_hitable(XZRect::new(0., 555., 0., 555., 0., white.clone()));
    list.add_hitable(FlipNormals::new(Box::new(XYRect::new(0., 555., 0., 555., 555., white.clone()))));

    let glass = Rc::new(Dielectric::new(1.5));
    list.add_hitable(Sphere::new(Vec3::new(190., 90., 190.), 90., glass.clone()));

    // let b = Box::new(HBox::new(Vec3::new(0., 0., 0.), Vec3::new(165., 165., 165.), white.clone()));
    // list.add_hitable(Translate::new(Box::new(
    //     RotateY::new(b, -18.)), Vec3::new(130., 0., 65.)));
    // let aluminum = Rc::new(Metal::new(Vec3::new(0.8, 0.85, 0.88), 0.));
    let b = Box::new(HBox::new(Vec3::new(0., 0., 0.), Vec3::new(165., 330., 165.), white.clone()));
    list.add_hitable(Translate::new(Box::new(RotateY::new(b, 15.)), Vec3::new(265., 0., 295.)));


    // XXX: This should be a copy or reference.
    // Use a temporary material, since this is only used for finding the
    // light, and I don't want to muck with making material Optional.
    let red = Rc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(0.65, 0.05, 0.05)))));
    let light_shape = XZRect::new(213., 343., 227., 332., 554., red.clone());
    let glass_sphere = Sphere::new(Vec3::new(190., 90., 90.), 90., red.clone());
    let mut light_shapes = HitableList::new();
    light_shapes.add_hitable(light_shape);
    light_shapes.add_hitable(glass_sphere);


    let lookfrom = Vec3::new(278., 278., -800.);
    let lookat = Vec3::new(278., 278., 0.);
    let dist_to_focus = 10.0;//(lookfrom-lookat).length();
    let aperture = 0.0;
    let nx = 500;
    let ny = 500;
    let camera = Camera::new(lookfrom,
                             lookat,
                             Vec3::new(0.0, 1.0, 0.0), // vup
                             40.0, // vfov
                             nx as f64 / ny as f64, // aspect
                             aperture,
                             dist_to_focus,
                             0.0,
                             1.0);


    let output = OutputSettings {
        format: OutputFormat::Png,
        filename_template: String::from("output.ppm"),
        width: nx,
        height: ny,
    };

    return Scene {
        world: Box::new(list),
        light_shapes: Box::new(light_shapes),
        camera: camera,
        output_settings: output,
        num_samples: 100,
    }
}

/*
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
*/
/*
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
*/

/*
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
*/

// y-up, x is right (right handed coordinate system)
// into screen is -z

fn is_numeric(val: String) -> std::result::Result<(), String> {
    match val.parse::<u32>() {
        Ok(..) => Ok(()),
        Err(..) => Err(String::from("Value must be a number.")),
    }
}

macro_rules! arg_value_with_default {
    ($m:ident, $v:expr, $t:ty, $d:expr) => {
        arg_value_with_default!($m.value_of($v), $t, $d)
    };
    ($m:ident.value_of($v:expr), $t:ty, $d:expr) => {
        if let Some(v) = $m.value_of($v) {
            match v.parse::<$t>() {
                Ok(val) => val,
                Err(_) => clap::Error::value_validation_auto(
                        format!("The argument '{}' isn't a valid value", v)).exit()
            }
        } else {
            $d
        }
    }
}

fn main() {
    let mut app = clap::App::new("raytracer")
        .version("0.1.0")
        .about("Experimental raytracer.")
        .arg(clap::Arg::with_name("output")
            .long("output")
            .takes_value(true)
            .value_name("OUTPUT")
            .display_order(1)
            .conflicts_with("gui")
            .help("Output filename"))
        .arg(clap::Arg::with_name("width")
            .long("width")
            .takes_value(true)
            .value_name("WIDTH")
            .display_order(2)
            .help("Image width"))
        .arg(clap::Arg::with_name("height")
            .long("height")
            .takes_value(true)
            .value_name("HEIGHT")
            .display_order(3)
            .help("Image height"));
    #[cfg(feature="gui")]
    {
        app = app.arg(clap::Arg::with_name("gui")
            .long("gui")
            .help("Render into a GUI window"));
    }
    let matches = app.get_matches();

    let mut scene = cornell_box();
    #[cfg(feature="gui")]
    {
        if matches.is_present("gui") {
            scene.output_settings.format = OutputFormat::Gui;
        }
    }
    if matches.is_present("output") {
        let filename = matches.value_of("output").unwrap().to_string();
        if let Err(e) = scene.output_settings.set_filename_template(filename) {
            let desc = format!("Invalid filename: {}", e.description());
            clap::Error::with_description(&desc, clap::ErrorKind::ValueValidation).exit();
            // perrln!();
            // std::process::exit(1);
        }
    }
    scene.output_settings.width = arg_value_with_default!(matches, "width", u32, scene.output_settings.width);
    scene.output_settings.height = arg_value_with_default!(matches, "height", u32, scene.output_settings.height);

    let mut output = new_output(&scene.output_settings, &scene).unwrap();
    render(&scene, &scene.output_settings, &mut output);
    #[cfg(feature="gui")]
    {
        if matches.is_present("gui") {
            output.wait_to_exit();
        }
    }
}
