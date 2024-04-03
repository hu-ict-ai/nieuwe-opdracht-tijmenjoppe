use num::Complex;
use std::f64::consts::PI;

mod animation;
mod av1;

use crate::animation::Animation;

const HEIGHT: usize = 300;
const WIDTH: usize = 400;
const MAX_ITER: usize = 81;
const FRAMES: usize = 150;
const CYCLE_FRAMES: usize = 250;

#[inline(always)]
fn colourise(h: usize) -> [u8; 3] {
    let h: f64 = (4. - h as f64 * 5. / MAX_ITER as f64).rem_euclid(6.);
    let x: u8 = (255. * (1. - (h.rem_euclid(2.) - 1.).abs())) as u8;

    if h < 1. {
        [255, x, 0]
    } else if h >= 1. && h < 2. {
        [x, 255, 0]
    } else if h >= 2. && h < 3. {
        [0, 255, x]
    } else if h >= 3. && h < 4. {
        [0, x, 255]
    } else if h >= 4. && h < 5. {
        [x, 0, 255]
    } else {
        [255, 0, x]
    }
}

fn render(_animation: &mut Animation, _t: usize, _offset: usize) {
    // TODO - render frame t and store in frames[t-offset]
}

fn main() {
    let id = -1;

    println!("Process at {} is handling {} up to {}", id, 0, FRAMES);

    let mut frames = Animation::new(FRAMES);

    // TODO - parallellisation
    (0..FRAMES).for_each(|t| {
        println!("{} is rendering {}", id, t);
        render(&mut frames, t, 0);
    });

    let root_buf = Animation::preallocate_root_buffer(FRAMES);
    let animation = Animation::reconstruct(root_buf);

    println!("Reconstructed {} frames", animation.len());

    animation.save("test.av1");
}
