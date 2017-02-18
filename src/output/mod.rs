pub mod error;
pub use error::*;
mod ppm;
mod image;

use scene::*;
use vec3::*;
use std::fs::File;

pub trait Output {
    fn begin(&mut self) -> Result<()> {Ok(())}
    fn put_pixel(&mut self, x: u32, y: u32, color: &Vec3<f64>) -> Result<()>;
    fn end(&mut self) -> Result<()> {Ok(())}
}

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Gui,
    Ppm,
    Png,
    Jpg,
}

#[derive(Debug, Clone)]
pub struct OutputSettings {
    pub format: OutputFormat,
    pub filename_template: String,
    pub width: u32,
    pub height: u32,
}

pub fn new_output(settings: &OutputSettings, scene: &Scene) -> Result<Box<Output>> {
    match settings.format {
        OutputFormat::Gui => unimplemented!(),
        OutputFormat::Ppm => Ok(Box::new(ppm::PpmOutput::new(settings, scene)?)),
        OutputFormat::Png => {
            let png = image::ImageOutput8::new(settings, scene)?;
            Ok(Box::new(png))
        },
        OutputFormat::Jpg => {
            let jpg = image::ImageOutput8::new(settings, scene)?;
            Ok(Box::new(jpg))
        },
    }
}

// Frame # (w/padding), Scene, RenderLayer, Camera, RenderPass, Extension (and custom), Version (custom label), date, time
fn path_from_template(template: &str, scene: &Scene) -> String {
    String::from(template)
}
