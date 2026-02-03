#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use ctru::prelude::*;
use ctru::gfx::{self, Screen};
use libm::{sinf, cosf};
use core::panic::PanicInfo;

/* ===================== panic handler ===================== */
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/* ===================== constants ===================== */
pub const PI: f32 = core::f32::consts::PI;
pub const TOTAL_TIME: f32 = 25.0;
pub const DT: f32 = 0.01;
pub const G: f32 = 9.81;
pub const STEPS: usize = (TOTAL_TIME / DT) as usize;
pub const FLIP_THRESHOLD: f32 = PI;
pub const SAFE_COLOR: [u8; 3] = [255, 255, 255];

/* ===================== color LUT ===================== */
pub fn build_color_lut() -> Vec<[u8; 3]> {
    let mut lut = Vec::with_capacity(256);
    for i in 0..256 {
        let n = i as f32 / 255.0;
        let h = 270.0 * n;
        let x = 1.0 - ((h / 60.0) % 2.0 - 1.0).abs();

        let (r, g, b) = if h < 60.0 {
            (1.0, x, 0.0)
        } else if h < 120.0 {
            (x, 1.0, 0.0)
        } else if h < 180.0 {
            (0.0, 1.0, x)
        } else if h < 240.0 {
            (0.0, x, 1.0)
        } else {
            (x, 0.0, 1.0)
        };

        lut.push([(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8]);
    }
    lut
}

/* ===================== physics ===================== */
#[inline(always)]
pub fn simulate_pendulum(theta1_init: f32, theta2_init: f32) -> f32 {
    let mut theta1 = theta1_init;
    let mut theta2 = theta2_init;
    let mut omega1 = 0.0;
    let mut omega2 = 0.0;
    let mut k = 0;

    while k + 1 < STEPS {
        for step in 0..4 {
            let d = theta1 - theta2;
            let s = sinf(d);
            let c = cosf(d);

            let sin1 = sinf(theta1);
            let cos1 = cosf(theta1);
            let sin2 = sin1 * c - cos1 * s;

            let omega1s = omega1 * omega1;
            let omega2s = omega2 * omega2;

            let denom = 2.0 - c * c;

            let alpha1 =
                (-G * (2.0 * sin1 - sin2 * c) - s * (omega2s + omega1s * c)) / denom;
            let alpha2 =
                (2.0 * s * (omega1s + G * cos1 + omega2s * c)) / denom;

            omega1 += alpha1 * DT;
            omega2 += alpha2 * DT;
            theta1 += omega1 * DT;
            theta2 += omega2 * DT;

            if theta2.abs() >= FLIP_THRESHOLD {
                return (k + step + 1) as f32 * DT;
            }
        }
        k += 4;
    }
    -1.0
}

/* ===================== chaos map ===================== */
pub fn generate_chaos_map(
    width: usize,
    height: usize,
    start1: f32,
    start2: f32,
    end1: f32,
    end2: f32,
    lut: &[[u8; 3]],
) -> Vec<u8> {
    let mut buffer = vec![0u8; width * height * 3];

    let w_denom = (width.saturating_sub(1)).max(1) as f32;
    let h_denom = (height.saturating_sub(1)).max(1) as f32;

    for y in 0..height {
        let theta1 = start2 + (end2 - start2) * y as f32 / h_denom;

        for x in 0..width {
            let theta2 = start1 + (end1 - start1) * x as f32 / w_denom;
            let t = simulate_pendulum(theta1, theta2);
            let idx = (y * width + x) * 3;

            if t < 0.0 {
                buffer[idx..idx + 3].copy_from_slice(&SAFE_COLOR);
            } else {
                let n = t / TOTAL_TIME;
                let n = 1.0 - (1.0 - n) * (1.0 - n);
                let i = ((n * (lut.len() - 1) as f32) as usize).min(lut.len() - 1);
                buffer[idx..idx + 3].copy_from_slice(&lut[i]);
            }
        }
    }
    buffer
}

/* ===================== entry ===================== */
#[ctru::entry]
fn main() -> ! {
    gfx::init_default();
    let top = Screen::Top;

    let width = 400;
    let height = 240;

    let scale = 1.0;
    let center1 = 0.0;
    let center2 = 0.0;

    let range2 = scale;
    let range1 = (height as f32 / width as f32) * range2;

    let start1 = (center1 - range1) * PI;
    let end1 = (center1 + range1) * PI;
    let start2 = (center2 - range2) * PI;
    let end2 = (center2 + range2) * PI;

    let lut = build_color_lut();
    let rgb = generate_chaos_map(width, height, start1, start2, end1, end2, &lut);

    // Draw buffer to top screen
    // 3DS expects BGR555, so we convert RGB888
    let framebuffer = gfx::get_framebuffer(top);
    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) * 3;
            let r = rgb[idx] >> 3;
            let g = rgb[idx + 1] >> 3;
            let b = rgb[idx + 2] >> 3;
            let pixel = (b as u16) << 10 | (g as u16) << 5 | (r as u16);
            framebuffer[y * width + x] = pixel;
        }
    }

    loop {
        gfx::flush_buffers();
        gfx::swap_buffers();
        gfx::wait_for_vblank();
    }
}
