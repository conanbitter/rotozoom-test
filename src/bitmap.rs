use std::path::Path;

use image::ImageReader;

pub struct Bitmap {
    data: Vec<u32>,
    width: usize,
    height: usize,
}

fn mirror_wrap(mut val: i32, max_val: usize) -> i32 {
    let max = max_val as i32;

    if val < 0 {
        val = -val - 1;
    }

    let rem = val % (2 * max);
    if rem < max { rem } else { (2 * max) - 1 - rem }
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

    pub fn get_pixel_wrapped(&self, x: i32, y: i32) -> u32 {
        let wx = mirror_wrap(x, self.width);
        let wy = mirror_wrap(y, self.height);
        self.data[wx as usize + wy as usize * self.width]
    }
}
