use image;
use super::*;

pub struct ImageOutput<P, Container>
    where P: image::Pixel + 'static
{
    // buf: image::ImageBuffer<image::Rgb<u8>,Vec<u8>>,
    buf: image::ImageBuffer<P,Container>,
    path: String,
}

pub type ImageOutput8 = ImageOutput<image::Rgb<u8>, Vec<u8>>;

impl<P, Container> ImageOutput<P, Container>
    where P: image::Pixel
{
    pub fn new(settings: &OutputSettings, scene: &Scene) -> Result<ImageOutput8> {
        Ok(ImageOutput{
            buf: image::ImageBuffer::new(settings.width, settings.height),
            path: path_from_template(&settings.filename_template, scene),
        })
    }
}

impl Output for ImageOutput8 {
    fn put_pixel(&mut self, x: u32, y: u32, color: &Vec3<f64>) -> Result<()> {
        let pixel = image::Rgb([(color.x*255.99) as u8,
                                (color.y*255.99) as u8,
                                (color.z*255.99) as u8]);
        self.buf.put_pixel(x, y, pixel);
        Ok(())
    }
    fn end(&mut self) -> Result<()> {
        self.buf.save(&self.path)?;
        Ok(())
    }
}
