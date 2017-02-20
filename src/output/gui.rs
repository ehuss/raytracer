use minifb::{Window, WindowOptions, Key, KeyRepeat};
use super::*;

pub struct GuiOutput {
    settings: OutputSettings,
    window: Window,
    buffer: Vec<u32>,
}

impl GuiOutput {
    pub fn new(settings: &OutputSettings, scene: &Scene) -> Result<GuiOutput> {
        let window = Window::new("Raytracer - ESC to exit",
                settings.width as usize,
                settings.height as usize,
                WindowOptions::default())?;
        Ok(GuiOutput{
            settings: settings.clone(),
            window: window,
            buffer: vec![0; (settings.width*settings.height) as usize],

        })
    }
    fn set_pixel(&mut self, x: u32, y: u32, color: &Vec3<f64>) {
        let i = (y*self.settings.width + x) as usize;
        let r = (color[0] * 255.99) as u32;
        let g = (color[1] * 255.99) as u32;
        let b = (color[2] * 255.99) as u32;
        let c32 = r << 16 | g << 8 | b;
        self.buffer[i] = c32;
    }
    fn vertical_line(&mut self, x: u32, y: u32, len: u32, color: &Vec3<f64>) {
        for y in y..y+len {
            self.set_pixel(x, y, color);
        }
    }
    fn horizontal_line(&mut self, x: u32, y: u32, len: u32, color: &Vec3<f64>) {
        for x in x..x+len {
            self.set_pixel(x, y, color);
        }
    }
    fn outline_bucket(&mut self, bucket: &Bucket) {
        let color = Vec3::new(1., 1., 1.);
        self.horizontal_line(bucket.x, bucket.y, bucket.width, &color);
        self.horizontal_line(bucket.x, bucket.y+bucket.height-1, bucket.width, &color);
        self.vertical_line(bucket.x, bucket.y, bucket.height, &color);
        self.vertical_line(bucket.x+bucket.width-1, bucket.y, bucket.height, &color);
    }
}

impl Output for GuiOutput {
    fn begin(&mut self) -> Result<()> {
        self.window.update_with_buffer(&self.buffer);
        Ok(())
    }
    fn put_pixel(&mut self, x: u32, y: u32, color: &Vec3<f64>) -> Result<()> {
        if self.window.is_key_down(Key::Escape) {
            return Err(Error::ExitRequested.into())
        }
        self.set_pixel(x, y, color);
        self.window.update_with_buffer(&self.buffer);
        Ok(())
    }
    fn begin_bucket(&mut self, bucket: &Bucket) -> Result<()> {
        self.outline_bucket(bucket);
        self.window.update_with_buffer(&self.buffer);
        Ok(())
    }
    fn put_bucket(&mut self, bucket: &Bucket, pixels: &Vec<Vec<Vec3<f64>>>) -> Result<()> {
        // XXX: This does not work very well.
        if self.window.is_key_pressed(Key::Escape, KeyRepeat::No) {
            return Err(Error::ExitRequested.into())
        }
        for (y, row) in pixels.iter().enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                let i = ((bucket.y+y as u32) *self.settings.width + bucket.x+x as u32) as usize;
                let r = (pixel[0] * 255.99) as u32;
                let g = (pixel[1] * 255.99) as u32;
                let b = (pixel[2] * 255.99) as u32;
                let c32 = r << 16 | g << 8 | b;
                self.buffer[i] = c32;
            }
        }
        self.window.update_with_buffer(&self.buffer);
        Ok(())
    }
    fn end(&mut self) -> Result<()> {
        Ok(())
    }
    fn wait_to_exit(&mut self) {
        // Unfortunate that there isn't an idle function.
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.window.update();
        }
    }
}
