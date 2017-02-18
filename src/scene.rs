use hitable::*;
use camera::*;
use output::*;

#[derive(Debug)]
pub struct Scene {
    pub world: Box<Hitable>,
    pub light_shapes: Box<Hitable>,
    pub camera: Camera,
    pub num_samples: u32, // TODO: put inside RenderQuality
    pub output_settings: OutputSettings,
}
