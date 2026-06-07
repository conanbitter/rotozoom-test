#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{f32::consts::PI, time::Instant};

use clap::Parser;
use display_info::DisplayInfo;
use minifb::{Scale, Window, WindowOptions};
use vector2d::Vector2D;

use crate::bitmap::Bitmap;

mod bitmap;

const ANGLE_SPEED: f32 = 0.01;

const ZOOM_SPEED: f32 = 0.015;
const ZOOM_MIN: f32 = 0.1;
const ZOOM_MAX: f32 = 1.2;
const ZOOM_ZERO: f32 = (ZOOM_MAX + ZOOM_MIN) / 2.0;
const ZOOM_RADIUS: f32 = ZOOM_ZERO - ZOOM_MIN;

const FLY_SPEED: f32 = 0.007;
const FLY_RADIUS: f32 = 100.0;

const DELTAS_COUNT: usize = 50;

fn rotate_vector(vec: Vector2D<f32>, sin: f32, cos: f32) -> Vector2D<f32> {
    Vector2D {
        x: vec.x * cos + vec.y * sin,
        y: -vec.x * sin + vec.y * cos,
    }
}

fn update_buffer(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    center: Vector2D<f32>,
    image: &Bitmap,
    angle: f32,
    zoom: f32,
    offset: Vector2D<f32>,
) {
    let image_origin = Vector2D::new(image.width as f32 / 2.0, image.height as f32 / 2.0);
    let sin = angle.sin();
    let cos = angle.cos();
    let total_offset = center + offset;

    for y in 0..height {
        for x in 0..width {
            let mut point = Vector2D::new(x as f32, y as f32);

            point -= total_offset;
            point = rotate_vector(point, sin, cos);
            point *= zoom;
            point += image_origin;

            let color = image.get_pixel_wrapped(point.x as i32, point.y as i32);
            buffer[x + y * width] = color;
        }
    }
}

fn wrap_angle(val: &mut f32) {
    if *val > PI * 2.0 {
        *val -= PI * 2.0;
    }
}

#[derive(clap::Parser, Debug)]
#[command(about, long_about = None, disable_help_flag = true)]
struct Args {
    #[arg(short, long, default_value_t = 800)]
    width: usize,

    #[arg(short, long, default_value_t = 600)]
    height: usize,

    #[arg(long, default_value_t = 0)]
    fps: usize,

    #[arg(short, long, default_value_t = false)]
    fullscreen: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let center = Vector2D::new(args.width as f32 / 2.0, args.height as f32 / 2.0);

    let (window_width, window_height) = if args.fullscreen {
        let mut display_info = DisplayInfo::from_point(0, 0)?;
        let display_info_list = DisplayInfo::all()?;
        for info in display_info_list {
            if info.is_primary {
                display_info = info;
                break;
            }
        }
        (display_info.width as usize, display_info.height as usize)
    } else {
        (args.width, args.height)
    };

    let mut window = Window::new(
        "RotoZoom",
        window_width,
        window_height,
        WindowOptions {
            borderless: args.fullscreen,
            title: !args.fullscreen,
            resize: false,
            scale: Scale::X1,
            scale_mode: if args.fullscreen {
                minifb::ScaleMode::Stretch
            } else {
                minifb::ScaleMode::Center
            },
            topmost: false,
            transparency: false,
            none: args.fullscreen,
        },
    )?;

    if !args.fullscreen {
        let (wx, wy) = window.get_position();
        let display_info = DisplayInfo::from_point(wx as i32, wy as i32)?;
        let new_pos_x = (display_info.width as isize - args.width as isize) / 2;
        let new_pos_y = (display_info.height as isize - args.height as isize) / 2;
        window.set_position(new_pos_x, new_pos_y);
    } else {
        window.set_cursor_visibility(false);
    }

    window.set_target_fps(args.fps);

    let mut buffer: Vec<u32> = vec![0; args.width * args.height];
    let bitmap = Bitmap::from_file("test_image.png")?;

    let mut angle = 0.0f32;
    let mut zoom_phase = 0.0f32;
    let mut fly_phase = 0.0f32;

    let mut last_time = Instant::now();
    let mut deltas: [f64; DELTAS_COUNT] = [1.0; DELTAS_COUNT];
    let mut delta_pos = 0;

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        angle += ANGLE_SPEED;
        wrap_angle(&mut angle);

        zoom_phase += ZOOM_SPEED;
        wrap_angle(&mut zoom_phase);
        let zoom = zoom_phase.sin() * ZOOM_RADIUS + ZOOM_ZERO;

        fly_phase += FLY_SPEED;
        wrap_angle(&mut fly_phase);
        let fly = Vector2D::new(fly_phase.sin() * FLY_RADIUS, fly_phase.cos() * FLY_RADIUS);

        update_buffer(&mut buffer, args.width, args.height, center, &bitmap, angle, zoom, fly);

        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_time);
        last_time = current_time;
        let delta_seconds = delta_time.as_secs_f64();
        deltas[delta_pos] = delta_seconds;
        delta_pos += 1;
        if delta_pos >= DELTAS_COUNT {
            delta_pos = 0;
        }
        let avg_delta = deltas.iter().sum::<f64>() / (DELTAS_COUNT as f64);
        let fps = if avg_delta > 0.0 { (1.0 / avg_delta) as i32 } else { 0 };

        window.set_title(&format!("RotoZoom   FPS: {}", fps));
        window.update_with_buffer(&buffer, args.width, args.height)?;
    }

    Ok(())
}
