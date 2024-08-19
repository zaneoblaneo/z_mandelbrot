#![allow(unused_assignments)]
use raylib::prelude::*;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub mod complex;
use complex::Complex;
extern crate std;

const WIDTH: i32 = 400;
const HEIGHT: i32 = 400;

const RENDER_WIDTH: usize = 2560;
const RENDER_HEIGHT: usize = 1440;

const OUT_FILE_PATH: &str = "mandelbrot_set_2560_1440.ppm";

fn map_f64(i: f64, min_i: f64, max_i: f64, min_o: f64, max_o: f64) -> f64 {
    (i - min_i) * (max_o - min_o) / (max_i - min_i) + min_o
}

fn map_f64_to_color(i: f64) -> Color {
    let _ = Color {
        r: (i * 255f64) as u8,
        g: (i * 255f64) as u8,
        b: (i * 255f64) as u8,
        a: 0xff,
    };
    Color::color_from_hsv((i as f32) * 360f32, 1f32, 1f32)
}

// z = 0
// z_sub_n+1 = z_sub_n*z_sub_n + pos
// TODO: make this multi-threaded?
fn compute_mandelbrot(width: i32,
                      height: i32,
                      min_x: f64, 
                      max_x: f64, 
                      min_y: f64, 
                      max_y: f64,
                      max_iter: usize) -> Vec<Color> {
    let mut mandelbrot = Vec::with_capacity((width * height) as usize);
    for y in 0..height {
        let mapped_y = map_f64(y as f64, 0f64, height as f64, min_y, max_y);
        'a: for x in 0..width {
            let mapped_x = 
                map_f64(x as f64, 0f64, width as f64, min_x, max_x);
            let complex_pos = Complex {
                r: mapped_x,
                i: mapped_y,
            };
            let mut z: Complex = Complex {r: 0f64, i: 0f64};
            let mut color: Color = 
                Color {r: 0xff, g: 0xff, b: 0xff, a: 0xff};
            for i in 0..max_iter {
                z = z * z + complex_pos;
                if z.len() > 4f64 {
                    color = map_f64_to_color(i as f64 / max_iter as f64);
                    mandelbrot.push(color);
                    continue 'a;
                }
            }
            mandelbrot.push(color);
        }
    }
    mandelbrot
}


fn calc_scale(percent: f64, min: f64, max: f64) -> f64 {
    (max - min) * percent
}

fn write_mandelbrot_to_disk(mandelbrot: Vec<Color>, 
                            path: PathBuf, 
                            width: usize, 
                            height: usize) {
    let mut file: File = File::create(path).expect("Failed to create file.");
    let header: String = format!("P6 {width} {height} 255\n");
    let mut out_data: Vec<u8> = Vec::with_capacity(width * height * 3);
    for pixel in mandelbrot {
        out_data.push(pixel.r);
        out_data.push(pixel.g);
        out_data.push(pixel.b);
    }
    file.write(header.as_bytes()).expect("Failed to write to file.");
    file.write(&out_data).expect("Failed to write to file.");
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Mandlebrot Set")
        .build();

    let mut max_iter: usize = 1000usize;
    let mut min_x: f64 = -2f64;
    let mut max_x: f64 = 0.47f64;
    let mut min_y: f64 = -1.12f64;
    let mut max_y: f64 = 1.12f64;
    let mut last_frame_time: f32 = 0f32;

    let mut mandelbrot: Vec<Color> = 
        compute_mandelbrot(WIDTH, 
                           HEIGHT, 
                           min_x, 
                           max_x, 
                           min_y, 
                           max_y, 
                           max_iter);

    while !rl.window_should_close() {
        let frame_time = rl.get_frame_time();
        if let Some(key) = rl.get_key_pressed() {
            if key == KeyboardKey::KEY_LEFT {
                let scale = calc_scale(0.1f64, min_x, max_x);
                min_x -= scale;
                max_x -= scale;
                mandelbrot = 
                    compute_mandelbrot(WIDTH, 
                                       HEIGHT, 
                                       min_x, 
                                       max_x, 
                                       min_y, 
                                       max_y, 
                                       max_iter);
            } else if key == KeyboardKey::KEY_RIGHT {
                let scale = calc_scale(0.1f64, min_x, max_x);
                min_x += scale;
                max_x += scale;
                mandelbrot = 
                    compute_mandelbrot(WIDTH, 
                                       HEIGHT, 
                                       min_x, 
                                       max_x, 
                                       min_y, 
                                       max_y, 
                                       max_iter);
            } else if key == KeyboardKey::KEY_DOWN {
                let scale = calc_scale(0.1f64, min_y, max_y);
                min_y += scale;
                max_y += scale;
                mandelbrot = 
                    compute_mandelbrot(WIDTH, 
                                       HEIGHT, 
                                       min_x, 
                                       max_x, 
                                       min_y, 
                                       max_y, 
                                       max_iter);
            } else if key == KeyboardKey::KEY_UP {
                let scale = calc_scale(0.1f64, min_y, max_y);
                min_y -= scale;
                max_y -= scale;
                mandelbrot = 
                    compute_mandelbrot(WIDTH, 
                                       HEIGHT, 
                                       min_x, 
                                       max_x, 
                                       min_y, 
                                       max_y, 
                                       max_iter);
            } else if key == KeyboardKey::KEY_KP_ADD {
                let scale_x = calc_scale(0.1f64, min_x, max_x);
                let scale_y = calc_scale(0.1f64, min_y, max_y);
                min_x += scale_x;
                max_x -= scale_x;
                min_y += scale_y;
                max_y -= scale_y;
                mandelbrot = 
                    compute_mandelbrot(WIDTH, 
                                       HEIGHT, 
                                       min_x, 
                                       max_x, 
                                       min_y, 
                                       max_y, 
                                       max_iter);
            } else if key == KeyboardKey::KEY_KP_SUBTRACT {
                let scale_x = calc_scale(0.1f64, min_x, max_x);
                let scale_y = calc_scale(0.1f64, min_y, max_y);
                min_x -= scale_x;
                max_x += scale_x;
                min_y -= scale_y;
                max_y += scale_y;
                mandelbrot = 
                    compute_mandelbrot(WIDTH, 
                                       HEIGHT, 
                                       min_x, 
                                       max_x, 
                                       min_y, 
                                       max_y, 
                                       max_iter);
            } else if key == KeyboardKey::KEY_F {
                max_iter -= 200usize;
                mandelbrot = 
                    compute_mandelbrot(WIDTH, 
                                       HEIGHT, 
                                       min_x, 
                                       max_x, 
                                       min_y, 
                                       max_y, 
                                       max_iter);
            } else if key == KeyboardKey::KEY_S {
                max_iter += 200usize;
                mandelbrot = 
                    compute_mandelbrot(WIDTH, 
                                       HEIGHT, 
                                       min_x, 
                                       max_x, 
                                       min_y, 
                                       max_y, 
                                       max_iter);
            } else if key == KeyboardKey::KEY_D {
                mandelbrot = 
                    compute_mandelbrot(WIDTH, 
                                       HEIGHT, 
                                       min_x, 
                                       max_x, 
                                       min_y, 
                                       max_y, 
                                       max_iter);
                println!("bounds_x: {min_x} < x < {max_x}");
                println!("bounds_y: {min_y} < y < {max_y}");
                println!("max_iter: {max_iter}");
                println!("frame_time: {last_frame_time}");
            } else if key == KeyboardKey::KEY_Q {
                std::process::exit(0);
            } else if key == KeyboardKey::KEY_Z {
                max_iter = 20usize;
                min_x = -2f64;
                max_x = 0.47f64;
                min_y = -1.12f64;
                max_y = 1.12f64;
                mandelbrot = 
                    compute_mandelbrot(WIDTH, 
                                       HEIGHT, 
                                       min_x, 
                                       max_x, 
                                       min_y, 
                                       max_y, 
                                       max_iter);
            } else if key == KeyboardKey::KEY_R {
                println!("Computing the mandelbrot for saving.");
                println!("This might take a while depending on the speed of");
                println!("your computer, the quality settings, and the size");
                println!("of the output image.");
                // render the mandelbrot at current settings 
                let mandelbrot = 
                    compute_mandelbrot(RENDER_WIDTH as i32, 
                                       RENDER_HEIGHT as i32, 
                                       min_x, 
                                       max_x, 
                                       min_y, 
                                       max_y, 
                                       max_iter);
                println!("mandelbrot computed.");
                let out_path: PathBuf = PathBuf::from(OUT_FILE_PATH);
                write_mandelbrot_to_disk(mandelbrot.clone(), 
                                         out_path, 
                                         RENDER_WIDTH, 
                                         RENDER_HEIGHT);
                println!("Wrote mandelbrot to disk.");
                println!("Path: {OUT_FILE_PATH}");

            }
        }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color { r: 0x18, g: 0x18, b: 0x18, a: 0xff });
        for i in 0..WIDTH * HEIGHT {
            d.draw_pixel(i % WIDTH, i / WIDTH, mandelbrot[i as usize]);
        }
        last_frame_time = frame_time;
    }
}
