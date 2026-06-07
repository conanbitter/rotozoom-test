use std::path::Path;

use image::ImageReader;

pub struct Bitmap {
    data: Vec<u32>,
    width: usize,
    height: usize,
}

impl Bitmap {
    pub fn from_file<P: AsRef<Path>>(file: P) -> anyhow::Result<Bitmap> {
        let image = ImageReader::open(file)?.decode()?.to_rgb8();
        let width = image.width() as usize;
        let height = image.height() as usize;
        let mut data: Vec<u32> = vec![0; width * height];

        for (pxin, pxout) in image.pixels().zip(data.iter_mut()) {
            let r = pxin[0] as u32;
            let g = pxin[1] as u32;
            let b = pxin[2] as u32;
            *pxout = r << 16 | g << 8 | b;
        }

        Ok(Bitmap { width, height, data })
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> u32 {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            self.data[x as usize + y as usize * self.width]
        } else {
            0
        }
    }
}
