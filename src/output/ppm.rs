use super::*;
use std::io::prelude::*;

#[derive(Debug)]
pub struct PpmOutput {
    settings: OutputSettings,
    path: String,
    buffer: Vec<u8>
}

impl PpmOutput {
    pub fn new(settings: &OutputSettings, scene: &Scene) -> Result<PpmOutput> {
        let size = (settings.width*settings.height*3) as usize;
        Ok(PpmOutput{settings: settings.clone(),
                     path: path_from_template(&settings.filename_template, scene),
                     buffer: vec![0; size],
        })
    }
}

impl Output for PpmOutput {
    fn put_pixel(&mut self, x: u32, y: u32, color: &Vec3<f64>) -> Result<()> {
        let i = (3*(self.settings.width*y + x)) as usize;
        self.buffer[i] = (color[0] * 255.99) as u8;
        self.buffer[i+1] = (color[1] * 255.99) as u8;
        self.buffer[i+2] = (color[2] * 255.99) as u8;
        Ok(())
    }

    fn end(&mut self) -> Result<()> {
        let mut f = File::create(&self.path)?;
        writeln!(f, "P3\n{} {}\n255", self.settings.width, self.settings.height)?;
        for y in 0..self.settings.height {
            for x in 0..self.settings.width {
                let i = (3*(y*self.settings.width + x)) as usize;
                writeln!(f, "{} {} {}", self.buffer[i], self.buffer[i+1], self.buffer[i+2])?;
            }
        }
        f.sync_all()?;
        Ok(())
    }
}
