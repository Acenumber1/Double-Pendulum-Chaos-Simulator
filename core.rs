pub const PI: f32 = core::f32::consts::PI;

#[cfg(target_os = "horizon")]
extern crate alloc;
#[cfg(target_os = "horizon")]
use alloc::vec::Vec;

#[cfg(not(target_os = "horizon"))]
use std::vec::Vec;

pub const TOTAL_TIME: f32 = 25.0;
pub const DT: f32 = 0.01;
pub const G: f32 = 9.81;
pub const STEPS: usize = (TOTAL_TIME / DT) as usize;
pub const FLIP_THRESHOLD: f32 = PI;
pub const SAFE_COLOR: [u8; 3] = [255, 255, 255];

pub fn build_color_lut() -> Vec<[u8; 3]> {
    const LUT_SIZE: usize = 256;
    let mut lut = Vec::with_capacity(LUT_SIZE);

    for i in 0..LUT_SIZE {
        let normalized = i as f32 / (LUT_SIZE - 1) as f32;
        let h = 270.0 * normalized;
        let x = 1.0 - ((h / 60.0) % 2.0 - 1.0).abs();

        let (rp, gp, bp) = if h < 60.0 {
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

        lut.push([
            (rp * 255.0) as u8,
            (gp * 255.0) as u8,
            (bp * 255.0) as u8,
        ]);
    }

    lut
}

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
            let (s, c) = d.sin_cos();

            let (sin1, cos1) = theta1.sin_cos();
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

pub fn generate_chaos_map(
    width: usize,
    height: usize,
    start1: f32,
    start2: f32,
    end1: f32,
    end2: f32,
    lut: &[[u8; 3]],
) -> Vec<u8> {
    let mut data = Vec::with_capacity(width * height * 3);
    data.resize(width * height * 3, 0);

    let w_denom = (width.saturating_sub(1)).max(1) as f32;
    let h_denom = (height.saturating_sub(1)).max(1) as f32;

    for y in 0..height {
        let theta1 = start2 + (end2 - start2) * y as f32 / h_denom;

        for x in 0..width {
            let theta2 = start1 + (end1 - start1) * x as f32 / w_denom;

            let flip_time = simulate_pendulum(theta1, theta2);
            let idx = (y * width + x) * 3;

            if flip_time < 0.0 {
                data[idx..idx + 3].copy_from_slice(&SAFE_COLOR);
            } else {
                let n = flip_time / TOTAL_TIME;
                let normalized = 1.0 - (1.0 - n) * (1.0 - n);
                let lut_index =
                    ((normalized * (lut.len() - 1) as f32) as usize).min(lut.len() - 1);
                data[idx..idx + 3].copy_from_slice(&lut[lut_index]);
            }
        }
    }

    data
}