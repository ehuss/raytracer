pub mod error;
pub use self::error::*;
mod ppm;
mod image;
#[cfg(feature="gui")]
mod gui;
mod buckets;

pub use self::buckets::*;
use scene::*;
use vec3::*;
use std::fs::File;
use std::path::Path;

pub trait Output {
    fn begin(&mut self) -> Result<()> {Ok(())}
    fn put_pixel(&mut self, x: u32, y: u32, color: &Vec3<f64>) -> Result<()>;
    fn begin_bucket(&mut self, bucket: &Bucket) -> Result<()> {Ok(())}
    fn put_bucket(&mut self, bucket: &Bucket, pixels: &Vec<Vec<Vec3<f64>>>) -> Result<()> {
        for (y, row) in pixels.iter().enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                self.put_pixel(bucket.x+x as u32, bucket.y+y as u32, pixel)?;
            }
        }
        Ok(())
    }
    fn end(&mut self) -> Result<()> {Ok(())}
    fn wait_to_exit(&mut self) {}
}

#[derive(Debug, Clone)]
pub enum OutputFormat {
    #[cfg(feature="gui")]
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

impl OutputSettings {
    pub fn set_filename_template(&mut self, tmpl: String) -> Result<()> {
        let p = Path::new(&tmpl);
        if let Some(ext) = p.extension() {
            self.format = match ext.to_str().unwrap() {
                "png" => OutputFormat::Png,
                "jpg" => OutputFormat::Jpg,
                "jpegg" => OutputFormat::Jpg,
                _ => { return Err(Error::UnsupportedImageFormat.into()) }
            };
            self.filename_template = tmpl.clone();
            Ok(())
        } else {
            Err(Error::UnsupportedImageFormat.into())
        }
    }
}

pub fn new_output(settings: &OutputSettings, scene: &Scene) -> Result<Box<Output>> {
    match settings.format {
        #[cfg(feature="gui")]
        OutputFormat::Gui => Ok(Box::new(gui::GuiOutput::new(settings, scene)?)),
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
