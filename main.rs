use rayon::prelude::*;
use image::{RgbImage, Rgb};
use std::f32::consts::PI;
use std::time::Instant;
use std::sync::atomic::{AtomicUsize};
use lazy_static::lazy_static;

lazy_static! {
    static ref COLOR_LUT: Vec<[u8; 3]> = {
        const LUT_SIZE: usize = 256;
        let mut lut = Vec::with_capacity(LUT_SIZE);

        for i in 0..LUT_SIZE {
            let normalized = i as f32 / (LUT_SIZE - 1) as f32;
            let h = 270.0 * normalized;                  // hue in degrees
            let x = 1.0 - ((h / 60.0) % 2.0 - 1.0).abs(); // intermediate for RGB mapping

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
    };
}

// globals (match Python)
const TOTAL_TIME: f32 = 25.0;
const DT: f32 = 0.01;
const G: f32 = 9.81;
const SAFE_COLOR: [u8; 3] = [255, 255, 255];
const DO_PRINTOUT: bool = true;
const STEPS: usize = (TOTAL_TIME / DT) as usize;
const FLIP_THRESHOLD: f32 = PI;

#[inline(always)]
fn simulate_pendulum(theta1_init: f32, theta2_init: f32) -> f32 {
    let mut theta1 = theta1_init;
    let mut theta2 = theta2_init;
    let mut omega1: f32 = 0.0;
    let mut omega2: f32 = 0.0;

    // hoisted constants
    let g = G as f32;

    for k in 0..STEPS {
        // EARLY EXIT — avoids trig when already flipped
        if theta2.abs() >= FLIP_THRESHOLD {
            return k as f32 * DT;
        }

        let d = theta1 - theta2;
        let (s, c) = d.sin_cos();

        let sin1 = theta1.sin();
        let sin2 = theta2.sin();
        let cos1 = theta1.cos();

        let omega1s = omega1 * omega1;
        let omega2s = omega2 * omega2;

        let denom = 2.0 - c * c;

        let alpha1 =
            (-g * (2.0 * sin1 - sin2 * c) - s * (omega2s + omega1s * c)) / denom;
        let alpha2 =
            (2.0 * s * (omega1s + g * cos1 + omega2s * c)) / denom;

        omega1 += alpha1 * DT;
        omega2 += alpha2 * DT;
        theta1 += omega1 * DT;
        theta2 += omega2 * DT;
    }

    -1.0
}

fn generate_chaos_map(
    graph_res_x: usize,
    graph_res_y: usize,
    start1: f32,
    start2: f32,
    end1: f32,
    end2: f32,
) -> Vec<u8> {
    let row_counter = AtomicUsize::new(0);
    let mut data = vec![0u8; graph_res_x * graph_res_y * 3];

    // precompute theta ranges
    let theta_vals_x: Vec<f32> = (0..graph_res_x)
        .map(|j| start1 + (end1 - start1) * j as f32 / (graph_res_x - 1) as f32)
        .collect();

    let theta_vals_y: Vec<f32> = (0..graph_res_y)
        .map(|i| start2 + (end2 - start2) * i as f32 / (graph_res_y - 1) as f32)
        .collect();
    
    data.par_chunks_mut(graph_res_x * 3)
        .enumerate()
        .for_each(|(i, row)| {
            let theta1 = theta_vals_y[i];

            for j in 0..graph_res_x {
                let theta2 = theta_vals_x[j];
                let flip_time = simulate_pendulum(theta1, theta2);

                let idx = (graph_res_x - 1 - j) * 3;

                if flip_time < 0.0 {
                    row[idx..idx + 3].copy_from_slice(&SAFE_COLOR);
                } else {
		    let n = flip_time / TOTAL_TIME;
                    let normalized = 1.0-(1.0-n).powi(2);
        	    let lut_index = (normalized * (COLOR_LUT.len() - 1) as f32) as usize;
    		    row[idx..idx + 3].copy_from_slice(&COLOR_LUT[lut_index]);
                }
            }
            if DO_PRINTOUT {
                let done = row_counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
                if done % (graph_res_y / 20).max(1) == 0 {
                    println!("{:.1} %", 100.0 * done as f32 / graph_res_y as f32);
                }
            }
        });

    data
}

fn full_map(
    center1: f32,
    center2: f32,
    range1: f32,
    range2: f32,
    graph_res: usize,
) -> (Vec<u8>, usize, usize) {
    let start1 = (center1 - range1) * PI;
    let end1 = (center1 + range1) * PI;
    let start2 = (center2 - range2) * PI;
    let end2 = (center2 + range2) * PI;

    let mut graph_res_x =
        graph_res as f32 * ((end1 - start1) / PI);
    let mut graph_res_y =
        graph_res as f32 * ((end2 - start2) / PI);

    if graph_res_x >= graph_res_y {
        graph_res_y *= graph_res as f32 / graph_res_x;
        graph_res_x = graph_res as f32;
    } else {
        graph_res_x *= graph_res as f32 / graph_res_y;
        graph_res_y = graph_res as f32;
    }

    let graph_res_x = graph_res_x as usize;
    let graph_res_y = graph_res_y as usize;

    let data = generate_chaos_map(
        graph_res_x,
        graph_res_y,
        start1,
        start2,
        end1,
        end2,
    );

    (data, graph_res_x, graph_res_y)
}

fn main() {
    let graph_res = 400;
    let scale = 1.0;
    let center1 = 0.0;
    let center2 = 0.0;
    let range2 = scale;
    let range1 = 240.0 / 400.0 * range2;

    let start = Instant::now();

    let (data, width, height) =
        full_map(center1, center2, range1, range2, graph_res);

    if DO_PRINTOUT {
        println!("\nElapsed: {:?}", start.elapsed());
    }

    let mut img = RgbImage::new(height as u32, width as u32);

    for y in 0..width {
        for x in 0..height {
            let idx = (x * width + y) * 3;
            img.put_pixel(
                x as u32,
                y as u32,
                Rgb([
                    data[idx],
                    data[idx + 1],
                    data[idx + 2],
                ]),
            );
        }
    }

    img.save("chaos.png").unwrap();
}
