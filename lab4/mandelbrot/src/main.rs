use std::ops::{Add, Mul};
use std::sync::{Arc, Mutex};
use std::thread;

use rayon::prelude::*;

use mandelbrot::*;

const WIDTH: u16 = 500;
const HEIGHT: u16 = 500;

const FRAMERATE: f32 = 24.0;

const KEYFRAMES: [Keyframe; 3] = [
    Keyframe {
        x_center: -0.75,
        y_center: 0.0,
        x_size: 3.5,
        y_size: 3.5,
        index: 0,
    },
    Keyframe {
        x_center: -1.35,
        y_center: 0.0,
        x_size: 0.2,
        y_size: 0.2,
        index: 100,
    },
    Keyframe {
        x_center: -0.75,
        y_center: 0.0,
        x_size: 3.5,
        y_size: 3.5,
        index: 300,
    },
];

const MAX_ITER: usize = 255;

fn main() {
    let mut animation =
        Animation::new("anim.gif", WIDTH, HEIGHT, FRAMERATE).expect("Error creating animation.");

    println!("Collecting frames...");
    let frames = frames_native();
    // let frames = frames_rayon();

    animation.add_frames(frames);
    animation
        .write_animation()
        .expect("Error saving animation.");
}

/// Parallel frame builder that only uses Rust threads and synchronization primitives.
pub fn frames_native() -> Vec<Frame> {
    let interpolated_frames: Vec<Keyframe> = get_interpolated_frames(&KEYFRAMES);
    let frames: Vec<Frame> = vec![Frame::empty(); interpolated_frames.len()];
    let mutex = Arc::new(Mutex::new(frames));
    let mut threads = vec![];
    for i in 0..interpolated_frames.len() {
        let mutex = Arc::clone(&mutex);
        let i_frame = interpolated_frames[i];

        threads.push(thread::spawn(move || {
            let mut guard = mutex.lock().unwrap();
            guard[i] = Frame::from_pixels(
                WIDTH,
                HEIGHT,
                draw_frame(WIDTH.into(), HEIGHT.into(), i_frame),
            );
        }))
    }
    for thread in threads {
        let _ = thread.join();
    }
    Arc::try_unwrap(mutex).unwrap().into_inner().unwrap()
}

/// Parallel frame builder that uses Rayon.
pub fn frames_rayon() -> Vec<Frame> {
    let i_frames = get_interpolated_frames(&KEYFRAMES);
    i_frames
        .par_iter()
        .map(|frame| {
            Frame::from_pixels(
                WIDTH,
                HEIGHT,
                draw_frame(WIDTH.into(), HEIGHT.into(), *frame),
            )
        })
        .collect()
}

pub fn calc_pixel((x, y): (f32, f32)) -> Pixel {
    let c = Complex::new(x, y);
    let mut z = Complex::new(0f32, 0f32);
    let mut iters: usize = 0;

    while z.norm() < 8192.0 && iters < MAX_ITER {
        z = z * z + c;
        iters += 1
    }

    let mut intensity = 0.0;
    if iters < MAX_ITER {
        let log_zn = z.norm().log2() / 2.0;
        let nu = log_zn.log2();
        intensity = ((iters + 1) as f32 - nu) / MAX_ITER as f32;
    }
    Pixel::from_rgb(intensity.powi(2), intensity, intensity.sqrt())
}

pub fn draw_frame(width: u32, height: u32, keyframe: Keyframe) -> Vec<Pixel> {
    (0..width * height)
        .map(|i| {
            let (x, y) = keyframe.get_coordinate(i / width, width, i % width, height);
            calc_pixel((x, y))
        })
        .collect()
}

#[derive(Clone, Copy, Debug)]
struct Complex {
    x: f32,
    y: f32,
}

impl Complex {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn norm(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2)
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        // Complex multiplication with only three multiplication operations.
        // Reference: https://mathworld.wolfram.com/ComplexMultiplication.html
        let ac = self.x * rhs.x;
        let bd = self.y * rhs.y;
        Self {
            x: ac - bd,
            y: (self.x + self.y) * (rhs.x + rhs.y) - ac - bd,
        }
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
