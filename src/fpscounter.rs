use image::ImageReader;

pub struct FPSCounter {
    digit_width: usize,
    digit_height: usize,
    digits: [Vec<u32>; 10],
}

fn convert_color(color: &image::Rgb<u8>) -> u32 {
    let r = color[0] as u32;
    let g = color[1] as u32;
    let b = color[2] as u32;
    r << 16 | g << 8 | b
}

impl FPSCounter {
    pub fn new() -> anyhow::Result<FPSCounter> {
        let image = ImageReader::open("fps_digits.png")?.decode()?.to_rgb8();
        let digit_height = image.height() as usize;
        let digit_width = image.width() as usize / 10;
        let digits: [Vec<u32>; 10] = std::array::from_fn(|i| {
            let mut result = vec![0; digit_width * digit_height];

            for y in 0..digit_height {
                for x in 0..digit_width {
                    let color = convert_color(image.get_pixel((x + i * digit_width) as u32, y as u32));
                    result[x + y * digit_width] = color;
                }
            }
            result
        });
        Ok(FPSCounter {
            digit_width,
            digit_height,
            digits,
        })
    }

    fn draw_digit(&self, digit: i32, x: usize, y: usize, buffer: &mut [u32], buffer_width: usize) {
        for src_y in 0..self.digit_height {
            let src_row = src_y * self.digit_width;
            let src_slice = &self.digits[digit as usize][src_row..src_row + self.digit_width];

            let dst_y = src_y + y;
            let dst_row = x + dst_y * buffer_width;
            let dst_slice = &mut buffer[dst_row..dst_row + self.digit_width];

            dst_slice.copy_from_slice(src_slice);
        }
    }

    pub fn draw_fps(&self, mut fps: i32, x: usize, y: usize, buffer: &mut [u32], buffer_width: usize) {
        let digit_count = if fps >= 10000 {
            fps = 9999;
            4
        } else if fps >= 1000 {
            4
        } else if fps >= 100 {
            3
        } else if fps >= 10 {
            2
        } else {
            1
        };

        let mut x_offset = x + self.digit_width * digit_count;

        for _ in 0..digit_count {
            let digit = fps % 10;
            fps /= 10;
            x_offset -= self.digit_width;
            self.draw_digit(digit, x_offset, y, buffer, buffer_width);
        }
    }
}
