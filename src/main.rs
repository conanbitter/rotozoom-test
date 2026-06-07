use std::f32::consts::PI;

use display_info::DisplayInfo;
use minifb::{Scale, Window, WindowOptions};
use vector2d::Vector2D;

use crate::bitmap::Bitmap;

mod bitmap;

const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 600;

const CENTER: Vector2D<f32> = Vector2D::new(WINDOW_WIDTH as f32 / 2.0, WINDOW_HEIGHT as f32 / 2.0);

const ANGLE_SPEED: f32 = 0.01;

const ZOOM_SPEED: f32 = 0.015;
const ZOOM_MIN: f32 = 0.1;
const ZOOM_MAX: f32 = 1.2;
const ZOOM_ZERO: f32 = (ZOOM_MAX + ZOOM_MIN) / 2.0;
const ZOOM_RADIUS: f32 = ZOOM_ZERO - ZOOM_MIN;

const FLY_SPEED: f32 = 0.007;
const FLY_RADIUS: f32 = 100.0;

fn rotate_vector(vec: Vector2D<f32>, sin: f32, cos: f32) -> Vector2D<f32> {
    Vector2D {
        x: vec.x * cos + vec.y * sin,
        y: -vec.x * sin + vec.y * cos,
    }
}

fn update_buffer(buffer: &mut [u32], image: &Bitmap, angle: f32, zoom: f32, offset: Vector2D<f32>) {
    let image_origin = Vector2D::new(image.width as f32 / 2.0, image.height as f32 / 2.0);
    let sin = angle.sin();
    let cos = angle.cos();

    for y in 0..WINDOW_HEIGHT {
        for x in 0..WINDOW_WIDTH {
            let mut point = Vector2D::new(x as f32, y as f32);

            point -= CENTER + offset;
            point = rotate_vector(point, sin, cos);
            point *= zoom;
            point += image_origin;

            let color = image.get_pixel_wrapped(point.x as i32, point.y as i32);
            buffer[x + y * WINDOW_WIDTH] = color;
        }
    }
}

fn wrap_angle(val: &mut f32) {
    if *val > PI * 2.0 {
        *val -= PI * 2.0;
    }
}

fn main() -> anyhow::Result<()> {
    let mut window = Window::new(
        "RotoZoom",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions {
            resize: false,
            scale: Scale::X1,
            scale_mode: minifb::ScaleMode::Center,
            ..WindowOptions::default()
        },
    )?;

    let (wx, wy) = window.get_position();
    let display_info = DisplayInfo::from_point(wx as i32, wy as i32)?;
    let new_pos_x = (display_info.width as isize - WINDOW_WIDTH as isize) / 2;
    let new_pos_y = (display_info.height as isize - WINDOW_HEIGHT as isize) / 2;
    window.set_position(new_pos_x, new_pos_y);

    window.set_target_fps(60);

    let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];
    let bitmap = Bitmap::from_file("lena.png")?;

    let mut angle = 0.0f32;
    let mut zoom_phase = 0.0f32;
    let mut fly_phase = 0.0f32;

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        angle += ANGLE_SPEED;
        wrap_angle(&mut angle);

        zoom_phase += ZOOM_SPEED;
        wrap_angle(&mut zoom_phase);
        let zoom = zoom_phase.sin() * ZOOM_RADIUS + ZOOM_ZERO;

        fly_phase += FLY_SPEED;
        wrap_angle(&mut fly_phase);
        let fly = Vector2D::new(fly_phase.sin() * FLY_RADIUS, fly_phase.cos() * FLY_RADIUS);

        update_buffer(&mut buffer, &bitmap, angle, zoom, fly);
        window.update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)?;
    }

    Ok(())
}
