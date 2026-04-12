#![allow(dead_code)]

use std::sync::LazyLock;

use nannou::{
    color::{Rgb8, Srgb, rgb8},
    image::{GenericImageView, open},
};

const GRADIENT_PATH: &str = "res/gradient_colourmap_inferno.png";

static GRADIENT: LazyLock<Vec<Srgb<u8>>> = LazyLock::new(|| {
    let img = open(GRADIENT_PATH).unwrap_or_else(|_| panic!("Expect a gradient img at path [{GRADIENT_PATH}]"));

    let width = img.width();

    let mut values = Vec::with_capacity(width as usize);

    for x in 0..width {
        let px = img.get_pixel(x, 0).0;
        values.push(rgb8(px[0], px[1], px[2]));
    }

    values
});

pub fn sample_f32(amount: f32) -> Rgb8 {
    let exact_x = ((GRADIENT.len() - 1) as f32 * amount).trunc() as usize;
    GRADIENT[exact_x]
}

pub fn sample_f64(amount: f64) -> Rgb8 {
    let exact_x = ((GRADIENT.len() - 1) as f64 * amount).trunc() as usize;
    GRADIENT[exact_x]
}
