#![cfg_attr(target_os = "horizon", no_std)]

extern crate alloc;

mod core;

use core::{build_color_lut, generate_chaos_map, PI};

#[cfg(not(target_os = "horizon"))]
use image::{Rgb, RgbImage};

#[cfg(target_os = "horizon")]
use ctru::prelude::*;
#[cfg(target_os = "horizon")]
use ctru::gfx::{self, Screen};


fn main() {
    let width = 400;
    let height = 240;

    let scale = 1.0;
    let center1 = 0.0;
    let center2 = 0.0;
    let range2 = scale;
    let range1 = 240.0 / 400.0 * range2;

    let start1 = (center1 - range1) * PI;
    let end1 = (center1 + range1) * PI;
    let start2 = (center2 - range2) * PI;
    let end2 = (center2 + range2) * PI;
    
    let lut = build_color_lut();
    let image_data = generate_chaos_map(width, height, start1, start2, end1, end2, &lut);
    

    #[cfg(not(target_os = "horizon"))]
    {
        let mut img = RgbImage::new(width as u32, height as u32);

        for y in 0..height {
	    for x in 0..width {
	        let idx = (y * width + x) * 3;
	        img.put_pixel(
		    x as u32,
		    y as u32,
		    Rgb([
		        image_data[idx],
		        image_data[idx + 1],
		        image_data[idx + 2],
		    ]),
	        );
	    }
        }
        img.save("chaos.png").unwrap(); }
    }