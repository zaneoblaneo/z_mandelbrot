
use raylib::prelude::*;

pub mod complex;
use complex::Complex;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

fn map_f32(i: f32, min_i: f32, max_i: f32, min_o: f32, max_o: f32) -> f32 {
    (i - min_i) * (max_o - min_o) / (max_i - min_i) + min_o
}

fn map_f32_to_color(i: f32) -> Color {
    Color {
        r: (i * 255f32) as u8,
        g: (i * 255f32) as u8,
        b: (i * 255f32) as u8,
        a: 0xff,
    }
}

// z = 0
// z_sub_n+1 = z_sub_n*z_sub_n + pos
// TODO: make this multi-threaded?
fn compute_mandelbrot(min_x: f32, 
                      max_x: f32, 
                      min_y: f32, 
                      max_y: f32,
                      max_iter: usize) -> Vec<Color> {
    let mut mandelbrot = Vec::with_capacity((WIDTH * HEIGHT) as usize);
    for y in 0..HEIGHT {
        let mapped_y = 
            map_f32(y as f32, 0f32, HEIGHT as f32, min_y, max_y);
        'a: for x in 0..WIDTH {
            let mapped_x = 
                map_f32(x as f32, 0f32, WIDTH as f32, min_x, max_x);
            let complex_pos = Complex {
                r: mapped_x,
                i: mapped_y,
            };
            let mut last_z: Complex = Complex {r: 0f32, i: 0f32};
            let mut z: Complex = Complex {r: 0f32, i: 0f32};
            let mut color: Color = Color {r: 0xff, g: 0xff, b: 0xff, a: 0xff};
            for i in 0..max_iter {
                last_z = z;
                z = last_z * last_z + complex_pos;
                if z.len() > 40f32 {
                    color = map_f32_to_color(i as f32 / max_iter as f32);
                    mandelbrot.push(color);
                    continue 'a;
                }
            }
            mandelbrot.push(color);
        }
    }
    mandelbrot
}


fn calc_scale(percent: f32, min: f32, max: f32) -> f32 {
    (max - min) * percent
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Mandlebrot Set")
        .build();

    let mut max_iter: usize = 1000usize;
    let mut min_x: f32 = -2f32;
    let mut max_x: f32 = 0.47f32;
    let mut min_y: f32 = -1.12f32;
    let mut max_y: f32 = 1.12f32;
    let mut last_frame_time: f32 = 0f32;

    let mut mandelbrot: Vec<Color> = 
        compute_mandelbrot(min_x, max_x, min_y, max_y, max_iter);

    while !rl.window_should_close() {
        let frame_time = rl.get_frame_time();
        if let Some(key) = rl.get_key_pressed() {
            if key == KeyboardKey::KEY_LEFT {
                let scale = calc_scale(0.05f32, min_x, max_x);
                min_x -= scale;
                max_x -= scale;
            } else if key == KeyboardKey::KEY_RIGHT {
                let scale = calc_scale(0.05f32, min_x, max_x);
                min_x += scale;
                max_x += scale;
            } else if key == KeyboardKey::KEY_DOWN {
                let scale = calc_scale(0.05f32, min_y, max_y);
                min_y += scale;
                max_y += scale;
            } else if key == KeyboardKey::KEY_UP {
                let scale = calc_scale(0.05f32, min_y, max_y);
                min_y -= scale;
                max_y -= scale;
            } else if key == KeyboardKey::KEY_KP_ADD {
                let scale_x = calc_scale(0.05f32, min_x, max_x);
                let scale_y = calc_scale(0.05f32, min_y, max_y);
                min_x += scale_x;
                max_x -= scale_x;
                min_y += scale_y;
                max_y -= scale_y;
            } else if key == KeyboardKey::KEY_KP_SUBTRACT {
                let scale_x = calc_scale(0.05f32, min_x, max_x);
                let scale_y = calc_scale(0.05f32, min_y, max_y);
                min_x -= scale_x;
                max_x += scale_x;
                min_y -= scale_y;
                max_y += scale_y;
            } else if key == KeyboardKey::KEY_F {
                max_iter -= 200usize;
            } else if key == KeyboardKey::KEY_S {
                max_iter += 200usize;
            } else if key == KeyboardKey::KEY_R {
                mandelbrot = 
                    compute_mandelbrot(min_x, max_x, min_y, max_y, max_iter);
                println!("bounds_x: {min_x} < x < {max_x}");
                println!("bounds_y: {min_y} < y < {max_y}");
                println!("max_iter: {max_iter}");
                println!("frame_time: {last_frame_time}");
            } else if key == KeyboardKey::KEY_Q {
                std::process::exit(0);
            } else if key == KeyboardKey::KEY_Z {
                max_iter = 20usize;
                min_x = -2f32;
                max_x = 0.47f32;
                min_y = -1.12f32;
                max_y = 1.12f32;
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
