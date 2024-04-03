use image::RgbImage;
use rav1e::prelude::*;

use crate::{HEIGHT, WIDTH};

pub fn convert(ctx: &Context<u8>, img: &RgbImage) -> rav1e::Frame<u8> {
    let mut luma: [u8; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];
    let mut chroma_b: [u8; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];
    let mut chroma_r: [u8; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];
    let mut frame = ctx.new_frame();

    (0..WIDTH).for_each(|x| {
        (0..HEIGHT).for_each(|y| {
            let (yc, u, v) = rgb_to_ycbcr(img.get_pixel(x as u32, y as u32));
            luma[y * WIDTH + x] = yc;
            chroma_b[y * WIDTH + x] = u;
            chroma_r[y * WIDTH + x] = v;
        })
    });
    frame.planes[0].copy_from_raw_u8(&luma, WIDTH, 1);
    frame.planes[1].copy_from_raw_u8(&chroma_b, WIDTH, 1);
    frame.planes[2].copy_from_raw_u8(&chroma_r, WIDTH, 1);
    frame
}

#[inline(always)]
fn rgb_to_ycbcr(px: &image::Rgb<u8>) -> (u8, u8, u8) {
    let max_val = ((1 << 8) - 1) as f32;
    let shift = (max_val * 0.5).round();
    let scale = max_val / 255.;
    let y = scale * 0.2990 * f32::from(px.0[0])
        + scale * 0.5870 * f32::from(px.0[1])
        + scale * 0.1140 * f32::from(px.0[2]);
    let cb = (f32::from(px.0[2]) * scale - y).mul_add(1.7720, shift);
    let cr = (f32::from(px.0[0]) * scale - y).mul_add(1.4220, shift);
    (y.round() as u8, cb.round() as u8, cr.round() as u8)
}
